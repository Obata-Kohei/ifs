use std::{f64, fs, time::{SystemTime, UNIX_EPOCH}};
use std::path::Path;
use clap::Parser;
use serde::Serialize;
use ifs::{ifs::{IFS, Point}, util::analysis::{bounding_box, correct_aspect}};
use ifs::util::io::*;

#[derive(Parser)]
struct Args {
    // 作成する画像の枚数
    #[arg(short, long, default_value_t = 1)]
    n: usize,

    #[arg(long, default_value_t = 128)]
    width: u32,
    #[arg(long, default_value_t = 128)]
    height: u32,

    #[arg(short, long, default_value = "result")]
    path: String,

    #[arg(short, long, default_value_t = false)]
    silent: bool,

    #[arg(long, default_value_t = 0.0)]
    initial_point_x: f64,
    #[arg(long, default_value_t = 0.0)]
    initial_point_y: f64,
    #[arg(long, default_value_t = 1000)]
    burn_in: usize,
    #[arg(long, default_value_t = 20000)]
    trial_iterations: usize,
    #[arg(long, default_value_t = 100000)]
    final_iterations: usize,

    #[arg(long, default_value_t = 1000)]
    minimum_point_count: usize,
    #[arg(long, default_value_t = 1e-3)]
    minimum_attractor_size_width: f64,
    #[arg(long, default_value_t = 1e-3)]
    minimum_attractor_size_height: f64,
    #[arg(long, default_value_t = 0.2)]
    fill_rate_range_min: f64,
    #[arg(long, default_value_t = 0.5)]
    fill_rate_range_max: f64,
    #[arg(long, default_value_t = 0.1)]
    aspect_range_min: f64,  // aspect = ヨコ/タテの値がこの範囲にない場合に除外
    #[arg(long, default_value_t = 10.0)]
    aspect_range_max: f64,  // aspect = ヨコ/タテの値がこの範囲にない場合に除外
    #[arg(long, default_value_t = 0.0)]
    maximum_avg_contractivity: f64,  // 平均contractivityがこれより小さいときには除外
    #[arg(long, default_value_t = 1.5)]
    max_spectral_norm: f64,  // spectral normがこれより大きい場合に除外
}

fn main() {
    let args: Args = Args::parse();

    let n = args.n;
    let width = args.width;
    let height = args.height;
    let path_name = args.path;
    let silent_mode = args.silent;
    let burn_in = args.burn_in;
    let trial_iterations = args.trial_iterations;
    let final_iterations = args.final_iterations;

    let quality_config = QualityConfig {
        minimum_point_count: args.minimum_point_count,
        minimum_attractor_size: (args.minimum_attractor_size_width, args.minimum_attractor_size_height),
        fill_rate_range: (args.fill_rate_range_min, args.fill_rate_range_max),
        aspect_range: (args.aspect_range_min, args.aspect_range_max),
        maximum_avg_contractivity: args.maximum_avg_contractivity,
        maximum_spectral_norm: args.max_spectral_norm,
    };

    let mut rng = rand::rng();
    fs::create_dir_all(&path_name).expect("Path name should be valid.");
    let mut records = Vec::with_capacity(n);

    for id in 0..n {
        if id % 10 == 0 && !silent_mode {
            println!("id {}", id);
        }

        // いいIFSができるまで生成
        let (ifs, qmetrics) = loop {
            let ifs = IFS::random_ifs(&mut rng);
            let init = Point {x: 0.0, y: 0.0};
            let pts = ifs.generate(&init, trial_iterations, burn_in);

            if let Some(qmetrics) = quality_check(&quality_config, &pts, &ifs, width, height) {
                break (ifs, qmetrics);
            }
            println!("aaa");
        };

        let init = Point {x: 0.0, y: 0.0};
        let pts = ifs.generate(&init, final_iterations, burn_in);
        let img = render(&pts, width, height);

        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("should be valid time")
            .as_millis();
        let filename = format!("{}/{}_{:03}.png", path_name, ts, id);
        img.save(&filename).expect("An image should be saved.");

        // qcfgとifsのパラメタをjosnに保存
        records.push(Record {
            id,
            file: filename.clone(),
            metrics: qmetrics,
            transforms: serialize_ifs(&ifs),
        });
    }

    // jsonへ保存
    let dir_name = Path::new(&path_name)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("result");

    let json_path = format!("{}/{}.json", path_name, dir_name);
    let json_str = serde_json::to_string_pretty(&records)
        .expect("JSON serialization failed");

    std::fs::write(&json_path, json_str)
        .expect("Failed to write JSON");

}

#[derive(Debug, Clone)]
pub struct QualityConfig {
    pub minimum_point_count: usize,  // 点の数がこれより小さいときには除外
    pub minimum_attractor_size: (f64, f64),  // (x方向, y方向)でこれよりアトラクタの幅が小さいときは除外
    pub fill_rate_range: (f64, f64),  // fill rateがこの範囲にないものを除外
    pub aspect_range: (f64, f64),  // aspect = ヨコ/タテの値がこの範囲にない場合に除外
    pub maximum_avg_contractivity: f64,  // 平均contractivityがこれより大きいときには除外
    pub maximum_spectral_norm: f64,  // spectral normがこれより大きい場合に除外
}

#[derive(Debug, Clone, Serialize)]
pub struct QualityMetrics {
    pub point_count: usize,
    pub attractor_size: (f64, f64),
    pub fill_rate: f64,
    pub aspect: f64,
    pub avg_contractivity: f64,
    pub max_spectral_norm: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct SerializableTransform {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
    pub weight: f64,
}

// 1レコード分のデータ
#[derive(Debug, Serialize)]
struct Record {
    id: usize,
    file: String,
    metrics: QualityMetrics,
    transforms: Vec<SerializableTransform>,
}

fn serialize_ifs(ifs: &IFS) -> Vec<SerializableTransform> {
    ifs.transforms.iter().map(|t| {
        SerializableTransform {
            a: t.affine.a,
            b: t.affine.b,
            c: t.affine.c,
            d: t.affine.d,
            e: t.affine.e,
            f: t.affine.f,
            weight: t.weight,
        }
    }).collect()
}

fn quality_check(
    qcfg: &QualityConfig,
    points: &[Point],
    ifs: &IFS,
    width: u32,
    height: u32,
) -> Option<QualityMetrics> {
    // 点の数をはかる
    let point_count = points.len();
    if point_count < qcfg.minimum_point_count {
        return None;
    }

    // 線や点のような見た目になっているときは除外
    let (xmin, xmax, ymin, ymax) = bounding_box(points);
    let attractor_size = (xmax - xmin, ymax - ymin);
    if attractor_size.0 < qcfg.minimum_attractor_size.0 || attractor_size.1 < qcfg.minimum_attractor_size.1 {
        return None;
    }

    // 非ゼロピクセルの割合チェック(fill rate)
    let (xmin2, xmax2, ymin2, ymax2) = correct_aspect(xmin, xmax, ymin, ymax, width, height);
    let mut occupied = std::collections::HashSet::new();
    for p in points {
        let px = ((p.x - xmin2) / (xmax2 - xmin2) * width as f64) as i32;
        let py = ((ymax2 - p.y) / (ymax2 - ymin2) * height as f64) as i32;
        if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
            occupied.insert((px, py));
        }
    }
    let fill_rate = occupied.len() as f64 / (width * height) as f64;
    if fill_rate < qcfg.fill_rate_range.0 || qcfg.fill_rate_range.1 < fill_rate {
        return None;
    }

    // アスペクト比が極端なものを除外
    let aspect = (xmax - xmin) / (ymax - ymin);
    if aspect < qcfg.aspect_range.0 || qcfg.aspect_range.1 < aspect {
        return None;
    }

    // IFSの変換の平均収縮性を求める
    let avg_contractivity = ifs.avg_contractivity();
    if avg_contractivity >= qcfg.maximum_avg_contractivity {
        return None;
    }

    // 各変換の個別スペクトルノルムチェック．極端に発散する変換を弾く
    let mut max_spectral_norm = f64::NEG_INFINITY;
    for tr in &ifs.transforms {
        let spec_norm = tr.affine.spectral_norm();
        if spec_norm > max_spectral_norm {
            max_spectral_norm = spec_norm;
        }
        if spec_norm > qcfg.maximum_spectral_norm {
            return None;
        }
    }

    Some(QualityMetrics {
        point_count,
        attractor_size,
        fill_rate,
        aspect,
        avg_contractivity,
        max_spectral_norm,
    })
}
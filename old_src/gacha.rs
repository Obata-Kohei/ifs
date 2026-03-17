use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
//use image::GrayImage;
//use rand::distr::Uniform;
use rand::prelude::*;
use crate::ifs::*;

fn det(a: f64, b: f64, c: f64, d: f64) -> f64 {
    a * d - b * c
}

fn random_affine(rng: &mut ThreadRng) -> Affine {
    let (sx, sy) = (rng.random_range(0.3..0.7), rng.random_range(0.3..0.7));
    let rot = rng.random_range(0.0..360.0);
    let (tx, ty) = (rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0));
    Affine::scale(sx, sy)
        .combine(Affine::rotate_deg(rot))
        .combine(Affine::translate(tx, ty))
}

fn random_ifs(rng: &mut ThreadRng) -> IFS {
    //let uniform = Uniform::try_from(-1.0..1.0).unwrap();

    // ランダムなAffine変換を作る
    let n_transform: usize = rng.random_range(2..=8);

    let mut affines: Vec<Affine> = Vec::new();
    for _ in 0..n_transform {
        affines.push(random_affine(rng))
    }

    // 確率pを算出: det(A_i) / sum(det(A_j))
    let mut weights = Vec::new();
    for af in &affines {
        weights.push(det(af.a, af.b, af.c, af.d).abs());
    }
    let det_sum: f64 = weights.iter().sum();
    let probs: Vec<f64> = weights.iter().map(|w| w / det_sum).collect();

    // IFSオブジェクト作成
    IFS {
        transforms: affines.into_iter().zip(probs).collect(),
    }
}

/// 2×2アフィン行列の最大特異値（= スペクトルノルム = Lipschitz定数）
fn spectral_norm(af: &Affine) -> f64 {
    let (a, b, c, d) = (af.a, af.b, af.c, af.d);
    let m = a*a + b*b + c*c + d*d;        // tr(AᵀA)
    let s = (a*d - b*c).powi(2);          // det(AᵀA) = det(A)²
    let discriminant = (m*m - 4.0*s).max(0.0);  // 数値誤差で負になる場合を防ぐ
    ((m + discriminant.sqrt()) / 2.0).sqrt()
}

// 平均の収縮性
fn average_contractivity(ifs: &IFS) -> f64 {
    ifs.transforms
        .iter()
        .map(|(af, prob)| spectral_norm(af) * prob)
        .sum()
}

fn quality_check(points: &[Point], ifs: &IFS, width: u32, height: u32) -> bool {
    if points.len() < 1000 {
        return false;
    }

    // 線や点となっているものは除外する
    let (xmin, xmax, ymin, ymax) = bounding_box(points);
    let w = xmax - xmin;
    let h = ymax - ymin;
    if w < 1e-3 || h < 1e-3 {
        return false;
    }

    // 非ゼロピクセル割合チェック（fill rate）
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
    if fill_rate < 0.2 || fill_rate > 0.5 {
        return false;
    }

    // アスペクト比が極端なものを除外
    let aspect = (xmax - xmin) / (ymax - ymin);
    if aspect > 10.0 || aspect < 0.1 {
        return false;
    }

    // 平均収縮性チェック
    // 理論保証：< 1.0 でアトラクタが一意に存在
    // 実用的マージン：0.95 以下に制限するとより安定
    let avg_c = average_contractivity(ifs);
    if avg_c >= 0.95 { return false; }

    // 各変換の個別スペクトルノルムチェック．極端に発散する変換を弾く
    for (af, _) in &ifs.transforms {
        if spectral_norm(af) > 1.5 { return false; }
    }

    true
}

pub fn exe_gacha(
    n: usize,
    width: u32,
    height: u32,
) {

    let mut rng = rand::rng();
    fs::create_dir_all("result").unwrap();

    for id in 0..n {

        if id % 10 == 0 {
            println!("id {}", id);
        }

        // 良いIFSができるまで生成
        let ifs = loop {

            let ifs = random_ifs(&mut rng);

            let init = Point { x: 0.0, y: 0.0 };
            let pts = ifs.generate(&init, 20000, 1000);

            if quality_check(&pts, &ifs, width, height) {
                break ifs;
            }
        };

        //let class_dir = format!("dataset/class_{:05}", id);
        //fs::create_dir_all(&class_dir).unwrap();

        let init = Point { x: 0.0, y: 0.0 };

        let pts = ifs.generate(&init, 100000, 1000);

        let img = render(&pts, width, height);

        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let filename = format!("result/{}_{:03}.png", ts, id);

        img.save(filename).unwrap();
    }
}
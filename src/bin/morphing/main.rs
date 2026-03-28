/*
resultの一つ上で以下のコマンドを打つと動画化できる:

ffmpeg -framerate 2 -i result/%05d.png -c:v libx264 -pix_fmt yuv420p output.mp4

framerateを適宜設定
*/

use std::fs;
use clap::Parser;
use ifs::ifs::*;
use ifs::ifs_presets::*;
use ifs::util::io::render;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    n: usize,  // 作成する画像枚数

    #[arg(long, default_value_t = 128)]
    width: u32,  // 画像の横幅
    #[arg(long, default_value_t = 128)]
    height: u32,  // 画像の高さ

    #[arg(short, long, default_value = "result")]
    path: String,

    #[arg(short, long, default_value = "Barnsley fern")]
    src_ifs: String,
    #[arg(short, long, default_value = "fractal tree")]
    dst_ifs: String,

    #[arg(long, default_value_t = 1_000_000)]
    iterations: usize,  // 計算の反復回数
    #[arg(long, default_value_t = 1_000)]
    burn_in: usize,  // burn_in(= 描画せずに計算だけする)の回数
}

fn main() {
    let args: Args = Args::parse();

    let n = args.n;
    let width = args.width;
    let height = args.height;
    let path = args.path;
    let src_ifs = ifs_presets(&args.src_ifs).expect("Preset name for src_ifs should be valid.");
    let dst_ifs = ifs_presets(&args.dst_ifs).expect("Preset name for dst_ifs should be valid.");
    let iterations = args.iterations;
    let burn_in = args.burn_in;

    fs::create_dir_all(&path).expect("Path name should be valid.");

    let t_iter = (0..=n).map(|x| x as f64 / n as f64);

    for (i, t) in t_iter.enumerate() {
        let ifs_t = lerp_ifs(&src_ifs, &dst_ifs, t);
        let pts = ifs_t.generate(
            &Point { x: 0.0, y: 0.0 },
            iterations,
            burn_in
        );
        let img = render(&pts, width, height);

        // 画像保存
        let filename = format!("{}/{:05}.png", path, i);
        img.save(&filename).expect("An image should be saved.");
        //img.save(Local::now().format("%Y%m%d%H%M%S%.3f").to_string() + ".png").expect("Image should be saved");
    }

}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}

fn lerp_ifs(ifs1: &IFS, ifs2: &IFS, t: f64) -> IFS {
    let mut ifs1 = ifs1.clone();
    let mut ifs2 = ifs2.clone();
    let n_transforms1 = ifs1.transforms.len();
    let n_transforms2 = ifs2.transforms.len();

    let n_transforms = n_transforms2.max(n_transforms1);
    // ifs間のtransformsの数をそろえる
    if n_transforms1 > n_transforms2 {
        ifs2.add_transform(ifs1.transforms.last().unwrap_or(&Transform { affine: Affine::id(), weight: 0.0 }));
    } else {
        ifs1.add_transform(ifs2.transforms.last().unwrap_or(&Transform { affine: Affine::id(), weight: 0.0 }));
    }

    // 各パラメタを補間する
    let mut transforms: Vec<Transform> = Vec::new();

    for i in 0..n_transforms {
        if let Some(tr1) = ifs1.transforms.get(i) {
            if let Some(tr2) = ifs2.transforms.get(i) {
                let af = Affine::new(
                    lerp(tr1.affine.a, tr2.affine.a, t),
                    lerp(tr1.affine.b, tr2.affine.b, t),
                    lerp(tr1.affine.c, tr2.affine.c, t),
                    lerp(tr1.affine.d, tr2.affine.d, t),
                    lerp(tr1.affine.e, tr2.affine.e, t),
                    lerp(tr1.affine.f, tr2.affine.f, t),
                );
                let w = lerp(tr1.weight, tr2.weight, t);
                transforms.push(Transform::new(af, w));
            }
        }
    }

    let ret = IFS::new(transforms);

    ret
}

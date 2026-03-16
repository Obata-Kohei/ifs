use std::fs;
use image::GrayImage;
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

fn quality_check(points: &[Point]) -> bool {
    if points.len() < 1000 {
        return false;
    }

    let (xmin, xmax, ymin, ymax) = bounding_box(points);

    let w = xmax - xmin;
    let h = ymax - ymin;

    if w < 1e-3 || h < 1e-3 {
        return false;
    }

    true
}

fn augment(img: &GrayImage) -> Vec<GrayImage> {
    vec![
        img.clone(),
        image::imageops::rotate90(img),
        image::imageops::rotate180(img),
        image::imageops::rotate270(img),
        image::imageops::flip_horizontal(img),
    ]
}

pub fn make_db(
    n_class: usize,
    n_instance: usize,
    width: u32,
    height: u32,
) {

    let mut rng = rand::rng();

    fs::create_dir_all("dataset").unwrap();

    for class_id in 0..n_class {

        println!("class {}", class_id);

        // 良いIFSができるまで生成
        let ifs = loop {

            let ifs = random_ifs(&mut rng);

            let init = Point { x: 0.0, y: 0.0 };
            let pts = ifs.generate(&init, 20000, 1000);

            if quality_check(&pts) {
                break ifs;
            }
        };

        let class_dir = format!("dataset/class_{:05}", class_id);
        fs::create_dir_all(&class_dir).unwrap();

        for inst_id in 0..n_instance {

            let init = Point { x: 0.0, y: 0.0 };

            let pts = ifs.generate(&init, 100000, 1000);

            let img = render(&pts, width, height);

            let augmented: Vec<GrayImage> = augment(&img);

            for (k, aug) in augmented.iter().enumerate() {

                let filename = format!(
                    "{}/{}_{}.png",
                    class_dir,
                    inst_id,
                    k
                );

                aug.save(filename).unwrap();
            }
        }
    }
}
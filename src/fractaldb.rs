use rand::distr::Uniform;
use rand::prelude::*;
use crate::ifs::*;

fn det(a: f64, b: f64, c: f64, d: f64) -> f64 {
    a * d - b * c
}

fn random_affine(rng: &mut ThreadRng, uniform: &Uniform<f64>) -> Affine {  // uniforomはふつう -1.0..1.0
    Affine::new(
        uniform.sample(rng),
        uniform.sample(rng),
        uniform.sample(rng),
        uniform.sample(rng),
        uniform.sample(rng),
        uniform.sample(rng),
    )
}

pub fn make_db() {
    let mut rng = rand::rng();
    let uniform_affine_param = Uniform::try_from(-1.0..=1.0).unwrap();

    // ランダムなAffine変換を作る
    let n_transform: usize = rng.random_range(2..=8);
    let mut affines: Vec<Affine> = Vec::with_capacity(n_transform);
    for _ in 1..=n_transform {
        affines.push(random_affine(&mut rng, &uniform_affine_param))
    }

    // 確率pを算出: det(A_i) / sum(det(A_j))
    let mut probs = Vec::with_capacity(n_transform);
    for af in &affines {
        probs.push(det(af.a, af.b, af.c, af.d));
    }
    let sum_det: f64 = probs.iter().sum();
    let probs = probs.iter().map(|d| d / sum_det).collect::<Vec<f64>>();

    // IFSオブジェクト作成
    let ifs = IFS {
        transforms: affines.into_iter().zip(probs).collect(),
    };

    // 点の生成
    let p = Point { x: 0.0, y: 0.0 };
    let points = ifs.generate(&p, 100000, 1000);
    // 画像保存
    render(&points, 362, 362);

}
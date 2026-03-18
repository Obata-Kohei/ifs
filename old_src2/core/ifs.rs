use crate::core::affine::{Affine, Point};
use crate::util::*;
use rand::prelude::*;
use rand::distr::weighted::WeightedIndex;

pub struct Transform {
    pub affine: Affine,
    pub weight: f64,
}

impl Transform {
    pub fn new(affine: Affine, weight: f64) -> Self {
        Self { affine, weight }
    }
}

pub struct IFS {
    pub transforms: Vec<Transform>,  // (Affine変換: Affine, その変換が選ばれる確率: f64)
    dist: WeightedIndex<f64>,
}

impl IFS {
    pub fn new(transforms: Vec<Transform>) -> Self {
        let weights: Vec<f64> = transforms.iter().map(|t| t.weight).collect();
        let dist = WeightedIndex::new(&weights).unwrap();

        Self { transforms, dist }
    }

    pub fn generate(
        &self,
        init_point: &Point,
        iterations: usize,
        burn_in: usize,
    ) -> Vec<Point> {
        let mut rng = rand::rng();

        let mut p = *init_point;
        let mut points = Vec::with_capacity(iterations);

        // burn-in
        for _ in 0..burn_in {
            let idx = self.dist.sample(&mut rng);
            p = self.transforms[idx].affine.apply(p);
        }

        // 本生成
        for _ in 0..iterations {
            let idx = self.dist.sample(&mut rng);
            p = self.transforms[idx].affine.apply(p);
            points.push(p);
        }

        points
    }

    pub fn iter(&self, init: Point) -> impl Iterator<Item = Point> {
        unimplemented!();
    }

    pub fn random_ifs(rng: &mut ThreadRng) -> Self {
        let n_transform: usize = rng.random_range(2..=8);

        let mut affines = Vec::with_capacity(n_transform);
        for _ in 0..n_transform {
            affines.push(Affine::random_affine(rng));
        }

        // det ベースの重み
        let weights: Vec<f64> = affines
            .iter()
            .map(|af| det2x2(af.a, af.b, af.c, af.d).abs())
            .collect();

        let det_sum: f64 = weights.iter().sum();

        let transforms: Vec<Transform> = affines
            .into_iter()
            .zip(weights.into_iter())
            .map(|(af, w)| Transform {
                affine: af,
                weight: w / det_sum,
            })
            .collect();

        Self::new(transforms)
    }
}

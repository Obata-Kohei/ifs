pub struct IFS {
    pub transforms: Vec<(Affine, f64)>,  // (Affine変換: Affine, その変換が選ばれる確率: f64)
}

impl IFS {
    pub fn generate(&self, init_point: &Point, iterations: usize, burn_in: usize) -> Vec<Point> {
        let mut rng = rand::rng();

        let weights: Vec<f64> = self.transforms.iter().map(|t| t.1).collect();
        let dist = WeightedIndex::new(&weights).unwrap();

        let mut p = *init_point;
        let mut points: Vec<Point> = Vec::with_capacity(iterations);

        // burn in: 最初のN回は描画しない
        for _ in 0..burn_in {
            let idx = dist.sample(&mut rng);
            p = self.transforms[idx].0.apply(p);
        }

        // 本生成
        for _ in 0..iterations {
            let idx = dist.sample(&mut rng);
            p = self.transforms[idx].0.apply(p);
            points.push(p);
        }

        points
    }

    fn random_ifs(rng: &mut ThreadRng) -> Self {
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
        Self {
            transforms: affines.into_iter().zip(probs).collect(),
        }
    }
}

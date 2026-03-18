use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use crate::{
    core::{
        affine::{
            //Affine,
            Point,
        },
        ifs::IFS
    },
    util::*,
};

#[derive(Debug, Clone)]
pub struct CreateParams {
    pub n: usize,
    pub width: u32,
    pub height: u32,

    pub burn_in: usize,
    pub trial_iterations: usize,
    pub final_iterations: usize,

    pub quality: QualityParams,
}

pub fn create(
    create_params: &CreateParams,
) -> Result<(), Box<dyn Error>> {
    let mut rng = rand::rng();
    fs::create_dir_all("result")?;

    for id in 0..create_params.n {
        if id % 10 == 0 {
            println!("id {}", id);
        }

        // 良いIFSができるまで生成
        let (ifs, qmetrics) = loop {
            let ifs = IFS::random_ifs(&mut rng);

            let init = Point {x: 0.0, y: 0.0};
            let pts = ifs.generate(&init, create_params.trial_iterations, create_params.burn_in);

            if let Some(qmetrics) = quality_check(&create_params, &pts, &ifs) {
                break (ifs, qmetrics);
            }
        };

        let init = Point { x: 0.0, y: 0.0 };

        let pts = ifs.generate(&init, create_params.final_iterations, create_params.burn_in);

        let img = render(&pts, create_params.width, create_params.height);

        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_millis();

        let filename = format!("result/{}_{:03}.png", ts, id);

        img.save(filename)?;

        // qmetricsとaffine paramsを保存
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub struct QualityParams {
    pub min_points: usize,
    pub min_bbox_size: f64,
    pub fill_rate_min: f64,
    pub fill_rate_max: f64,
    pub aspect_min: f64,
    pub aspect_max: f64,
    pub max_avg_contractivity: f64,
    pub max_spectral_norm: f64,
}

#[derive(Debug, Clone)]
pub struct QualityMetrics {
    pub point_count: usize,
    pub width: f64,
    pub height: f64,
    pub fill_rate: f64,
    pub aspect: f64,
    pub avg_contractivity: f64,
    pub max_spectral_norm: f64,
}

fn quality_check(
    create_params: &CreateParams,
    points: &[Point],
    ifs: &IFS,
) -> Option<QualityMetrics> {

}
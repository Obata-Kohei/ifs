use crate::ifs::*;

fn t(affine: Affine, w: f64) -> Transform {
    Transform::new(affine, w)
}

pub fn sierpinski_gasket() -> IFS {
    let s = 0.5;
    let h = (3.0f64).sqrt() / 2.0;

    IFS::new(vec![
        t(Affine::scale(s, s), 1.0),
        t(Affine::scale(s, s).then(Affine::translate(0.5, 0.0)), 1.0),
        t(Affine::scale(s, s).then(Affine::translate(0.25, h * 0.5)), 1.0),
    ])
}

pub fn barnsley_fern() -> IFS {
    IFS::new(vec![
        // stem
        t(
            Affine::scale(0.0, 0.16),
            0.01,
        ),
        // successively smaller leaflets
        t(
            Affine::scale(0.85, 0.85)
                .then(Affine::shear(0.04, -0.04))
                .then(Affine::translate(0.0, 1.6)),
            0.85,
        ),
        t(
            Affine::scale(0.2, 0.22)
                .then(Affine::rotate_deg(-60.0))
                .then(Affine::translate(0.0, 1.6)),
            0.07,
        ),
        t(
            Affine::scale(0.2, 0.22)
                .then(Affine::rotate_deg(60.0))
                .then(Affine::translate(0.0, 0.44)),
            0.07,
        ),
    ])
}

pub fn fractal_tree() -> IFS {
    IFS::new(vec![
        t(
            Affine::scale(0.0, 0.5),
            0.05
        ),
        t(
            Affine::scale(0.6, 0.6)
                .then(Affine::rotate_deg(45.0))
                .then(Affine::translate(0.0, 0.2)),
            0.475,
        ),
        t(
            Affine::scale(0.6, 0.6)
                .then(Affine::rotate_deg(-45.0))
                .then(Affine::translate(0.0, 0.2)),
            0.475,
        ),
    ])
}

pub fn dragon_curve() -> IFS {
    let s = (0.5f64).sqrt();

    IFS::new(vec![
        t(
            Affine::scale(s, s)
                .then(Affine::rotate_deg(45.0)),
            0.5,
        ),
        t(
            Affine::scale(s, s)
                .then(Affine::rotate_deg(135.0))
                .then(Affine::translate(1.0, 0.0)),
            0.5,
        ),
    ])
}

pub fn cyclosorus_fern() -> IFS {
    IFS::new(vec![
        // 茎 (Stem)
        t(Affine::scale(0.0, 0.25).then(Affine::translate(0.0, -0.4)), 0.02),
        // 全体の相似（少しの回転と高い維持率）
        t(
            Affine::scale(0.95, 0.93)
                .then(Affine::rotate_deg(-0.3)) // 微小な回転
                .then(Affine::translate(-0.002, 0.5)),
            0.84,
        ),
        // 左の葉
        t(
            Affine::scale(0.2, 0.2) 
                .then(Affine::rotate_deg(80.0))
                .then(Affine::translate(-0.09, 0.02)),
            0.07,
        ),
        // 右の葉
        t(
            Affine::scale(0.2, 0.2)
                .then(Affine::rotate_deg(-80.0))
                .then(Affine::translate(0.083, 0.12)),
            0.07,
        ),
    ])
}

pub fn square_fractal() -> IFS {
    let s = 1.0 / 3.0;
    let mut transforms = Vec::new();

    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 { continue; } // 中央を抜く
            transforms.push(t(
                Affine::scale(s, s).then(Affine::translate(i as f64 * s, j as f64 * s)),
                1.0
            ));
        }
    }
    IFS::new(transforms)
}

pub fn triangle_dust() -> IFS {
    let s = 1.0 / 3.0;
    let h = (3.0f64).sqrt() / 2.0;

    IFS::new(vec![
        t(Affine::scale(s, s), 1.0),
        t(Affine::scale(s, s).then(Affine::translate(2.0/3.0, 0.0)), 1.0),
        t(Affine::scale(s, s).then(Affine::translate(1.0/3.0, h)), 1.0),
    ])
}

pub fn vicsek_fractal() -> IFS {
    let s = 1.0 / 3.0;

    IFS::new(vec![
        t(Affine::scale(s, s).then(Affine::translate(1.0/3.0, 1.0/3.0)), 1.0),
        t(Affine::scale(s, s).then(Affine::translate(0.0, 1.0/3.0)), 1.0),
        t(Affine::scale(s, s).then(Affine::translate(2.0/3.0, 1.0/3.0)), 1.0),
        t(Affine::scale(s, s).then(Affine::translate(1.0/3.0, 0.0)), 1.0),
        t(Affine::scale(s, s).then(Affine::translate(1.0/3.0, 2.0/3.0)), 1.0),
    ])
}

pub fn pentagon_fractal() -> IFS {
    let s = 0.382;
    IFS::new(vec![
        // 真上 (90度)
        t(Affine::scale(s, s).then(Affine::translate(0.0, 1.0)), 1.0),
        // 左上 (162度) : cos(162)=-0.951, sin(162)=0.309
        t(Affine::scale(s, s).then(Affine::translate(-0.951, 0.309)), 1.0),
        // 左下 (234度) : cos(234)=-0.588, sin(234)=-0.809
        t(Affine::scale(s, s).then(Affine::translate(-0.588, -0.809)), 1.0),
        // 右下 (306度) : cos(306)=0.588,  sin(306)=-0.809
        t(Affine::scale(s, s).then(Affine::translate(0.588, -0.809)), 1.0),
        // 右上 (18度)  : cos(18)=0.951,   sin(18)=0.309
        t(Affine::scale(s, s).then(Affine::translate(0.951, 0.309)), 1.0),
    ])
}

pub fn spiral_fractal() -> IFS {
    IFS::new(vec![
        // メインの螺旋: 約 0.9倍のスケールで約 20度回転
        t(
            Affine::scale(0.89, 0.89)
                .then(Affine::rotate_deg(17.0))
                .then(Affine::translate(1.758, 1.408)),
            0.9,
        ),
        // 接続・補正用
        t(
            Affine::scale(0.28, 0.16)
                .then(Affine::rotate_deg(115.0))
                .then(Affine::translate(-6.721, 1.377)),
            0.1,
        ),
    ])
}

pub fn koch_curve() -> IFS {
    let s = 1.0 / 3.0;

    IFS::new(vec![
        t(Affine::scale(s, s), 1.0),
        t(
            Affine::scale(s, s)
                .then(Affine::rotate_deg(60.0))
                .then(Affine::translate(1.0/3.0, 0.0)),
            1.0
        ),
        t(
            Affine::scale(s, s)
                .then(Affine::rotate_deg(-60.0))
                .then(Affine::translate(0.5, (3.0f64).sqrt()/6.0)),
            1.0
        ),
        t(
            Affine::scale(s, s)
                .then(Affine::translate(2.0/3.0, 0.0)),
            1.0
        ),
    ])
}

pub fn koch_snowflake() -> IFS {

    let s3 = 3.0_f64.sqrt();

    IFS::new(vec![
            t(
                Affine::new(
                    -1.0/6.0,  s3/6.0,  1.0/6.0,
                    -s3/6.0, -1.0/6.0,  s3/6.0
                ),
                1.0
            ),
            t(
                Affine::new(
                     1.0/6.0, -s3/6.0, 1.0/6.0,
                     s3/6.0,  1.0/6.0, s3/6.0
                ),
                1.0
            ),
            t(
                Affine::new(
                    1.0/3.0, 0.0, 1.0/3.0,
                    0.0, 1.0/3.0, s3/3.0
                ),
                1.0
            ),
            t(
                Affine::new(
                    1.0/6.0,  s3/6.0, 2.0/3.0,
                   -s3/6.0,  1.0/6.0, s3/3.0
                ),
                1.0
            ),
            t(
                Affine::new(
                    1.0/2.0, -s3/6.0, 1.0/3.0,
                    s3/6.0,  1.0/2.0, 0.0
                ),
                1.0
            ),
            t(
                Affine::new(
                   -1.0/3.0, 0.0, 2.0/3.0,
                    0.0,-1.0/3.0, 0.0
                ),
                1.0
            ),
            t(
                Affine::new(
                    1.0/3.0, 0.0, 2.0/3.0,
                    0.0, 1.0/3.0, 0.0
                ),
                1.0
            ),

        ])
}

pub fn ifs_presets(name: &str) -> Option<IFS> {
    match name {
        "Sierpinski gasket" | "Sierpinski" | "sierpinski" | "gasket" | "triangle" | "tri" => Some(sierpinski_gasket()),

        "Barnsley fern" | "Barnsley" | "barnsley" | "fern" => Some(barnsley_fern()),

        "fractal tree" | "tree" => Some(fractal_tree()),

        "dragon curve" | "dragon" | "Heighway dragon" | "heighway" => Some(dragon_curve()),

        "cyclosorus fern" | "cyclosorus" | "fern2" => Some(cyclosorus_fern()),

        "square fractal" | "square" => Some(square_fractal()),

        "triangle dust" | "dust" => Some(triangle_dust()),

        "Vicsek fractal" | "Vicsek" | "vicsek" => Some(vicsek_fractal()),

        "pentagon fractal" | "pentagon" => Some(pentagon_fractal()),

        "spiral fractal" | "spiral" => Some(spiral_fractal()),

        "Koch curve" | "Koch" | "koch" => Some(koch_curve()),

        "Koch snowflake" | "snowflake" => Some(koch_snowflake()),

        _ => None,
    }
}

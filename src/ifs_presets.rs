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
        t(Affine::scale(0.0, 0.5), 0.05),
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
            (Transform::new(Affine { a:0.0, b:0.0, c:0.0, d:0.0, e:0.25, f:-0.4 }, 0.02)),
            (Transform::new(Affine { a:0.95, b:0.005, c:-0.002, d:-0.005, e:0.93, f:0.5 }, 0.84)),
            (Transform::new(Affine { a:0.035, b:-0.2, c:-0.09, d:0.16, e:0.04, f:0.02 }, 0.07)),
            (Transform::new(Affine { a:-0.04, b:0.2, c:0.083, d:0.16, e:0.04, f:0.12 }, 0.07)),
        ])
}

pub fn square_fractal() -> IFS {
    let s = 0.5;

    IFS::new(vec![
        t(Affine::scale(s, s), 1.0),
        t(Affine::scale(s, s).then(Affine::translate(0.5, 0.0)), 1.0),
        t(Affine::scale(s, s).then(Affine::translate(0.0, 0.5)), 1.0),
        t(Affine::scale(s, s).then(Affine::translate(0.5, 0.5)), 1.0),
    ])
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
    IFS::new(vec![
            (Transform::new(Affine { a:0.382, b:0.0, c:0.309, d:0.0, e:0.382, f:0.951 }, 1.0)),
            (Transform::new(Affine { a:0.382, b:0.0, c:-0.809, d:0.0, e:0.382, f:0.588 }, 1.0)),
            (Transform::new(Affine { a:0.382, b:0.0, c:-0.5, d:0.0, e:0.382, f:-0.588 }, 1.0)),
            (Transform::new(Affine { a:0.382, b:0.0, c:0.5, d:0.0, e:0.382, f:-0.588 }, 1.0)),
            (Transform::new(Affine { a:0.382, b:0.0, c:0.809, d:0.0, e:0.382, f:0.588 }, 1.0)),
        ])
}

pub fn spiral_fractal() -> IFS {
    IFS::new(vec![
        (Transform::new(Affine { a:0.787879, b:-0.424242, c:1.758647, d:0.242424, e:0.859848, f:1.408065 }, 0.9)),
        (Transform::new(Affine { a:-0.121212, b:0.257576, c:-6.721654, d:0.151515, e:0.053030, f:1.377236 }, 0.1)),
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

        "cyclosorus fern" | "cyclosorus" | "fern 2" => Some(cyclosorus_fern()),

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

use crate::ifs::*;

pub fn sierpinski_gasket() -> IFS {
    IFS {
        transforms: vec![
            (Affine { a:0.5,b:0.0,c:0.0, d:0.0,e:0.5,f:0.0 }, 1./3.),
            (Affine { a:0.5,b:0.0,c:0.5, d:0.0,e:0.5,f:0.0 }, 1./3.),
            (Affine { a:0.5,b:0.0,c:0.25, d:0.0,e:0.5,f:0.433 }, 1./3.),
        ],
    }
}

pub fn barnsley_gasket() -> IFS {
    IFS {
        transforms: vec![
            (Affine { a:0., b:0., c:0., d:0., e:0.16, f:0. }, 0.01),
            (Affine { a:0.85, b:0.04, c:0., d:-0.04, e:0.85, f:1.6 }, 0.85),
            (Affine { a:0.2, b:-0.26, c:0., d:0.23, e:0.22, f:1.6 }, 0.07),
            (Affine { a:-0.15, b:0.28, c:0., d:0.26, e:0.24, f:0.44 }, 0.07),
        ],
    }
}

pub fn fractal_tree() -> IFS {
    IFS {
        transforms: vec![
            (Affine { a:0., b:0., c:0., d:0., e:0.5, f:0.}, 0.05),
            (Affine { a:0.42, b:-0.42, c:0., d:0.42, e:0.42, f:0.2 }, 0.475),
            (Affine { a:0.42, b:0.42, c:0., d:-0.42, e:0.42, f:0.2 }, 0.475),
        ],
    }
}

pub fn dragon_curve() -> IFS {
    IFS {
        transforms: vec![
            (Affine { a:0.824074, b:0.281428, c:-0.882290, d:-0.212346, e:0.864198, f:-0.110607 }, 0.5),
            (Affine { a:0.088272, b:0.520988, c:0.785360, d:-0.463889, e:-0.377778, f:8.095795 }, 0.5),
        ]
    }
}

pub fn cyclosorus_fern() -> IFS {
    IFS {
        transforms: vec![
            (Affine { a:0.0, b:0.0, c:0.0, d:0.0, e:0.25, f:-0.4 }, 0.02),
            (Affine { a:0.95, b:0.005, c:-0.002, d:-0.005, e:0.93, f:0.5 }, 0.84),
            (Affine { a:0.035, b:-0.2, c:-0.09, d:0.16, e:0.04, f:0.02 }, 0.07),
            (Affine { a:-0.04, b:0.2, c:0.083, d:0.16, e:0.04, f:0.12 }, 0.07),
        ]
    }
}

pub fn square_fractal() -> IFS {
    IFS {
        transforms: vec![
            (Affine { a:0.5, b:0.0, c:0.0, d:0.0, e:0.5, f:0.0 }, 1.0),
            (Affine { a:0.5, b:0.0, c:0.5, d:0.0, e:0.5, f:0.0 }, 1.0),
            (Affine { a:0.5, b:0.0, c:0.0, d:0.0, e:0.5, f:0.5 }, 1.0),
            (Affine { a:0.5, b:0.0, c:0.5, d:0.0, e:0.5, f:0.5 }, 1.0),
        ]
    }
}

pub fn triangle_dust() -> IFS {
    IFS {
        transforms: vec![
            (Affine { a:0.333, b:0.0, c:0.0, d:0.0, e:0.333, f:0.0 }, 1.0),
            (Affine { a:0.333, b:0.0, c:0.666, d:0.0, e:0.333, f:0.0 }, 1.0),
            (Affine { a:0.333, b:0.0, c:0.333, d:0.0, e:0.333, f:0.577 }, 1.0),
        ]
    }
}

pub fn vicsek_fractal() -> IFS {
    IFS {
        transforms: vec![
            (Affine { a:0.333, b:0.0, c:0.333, d:0.0, e:0.333, f:0.333 }, 1.0),
            (Affine { a:0.333, b:0.0, c:0.0, d:0.0, e:0.333, f:0.333 }, 1.0),
            (Affine { a:0.333, b:0.0, c:0.666, d:0.0, e:0.333, f:0.333 }, 1.0),
            (Affine { a:0.333, b:0.0, c:0.333, d:0.0, e:0.333, f:0.0 }, 1.0),
            (Affine { a:0.333, b:0.0, c:0.333, d:0.0, e:0.333, f:0.666 }, 1.0),
        ]
    }
}

pub fn pentagon_fractal() -> IFS {
    IFS {
        transforms: vec![
            (Affine { a:0.382, b:0.0, c:0.309, d:0.0, e:0.382, f:0.951 }, 1.0),
            (Affine { a:0.382, b:0.0, c:-0.809, d:0.0, e:0.382, f:0.588 }, 1.0),
            (Affine { a:0.382, b:0.0, c:-0.5, d:0.0, e:0.382, f:-0.588 }, 1.0),
            (Affine { a:0.382, b:0.0, c:0.5, d:0.0, e:0.382, f:-0.588 }, 1.0),
            (Affine { a:0.382, b:0.0, c:0.809, d:0.0, e:0.382, f:0.588 }, 1.0),
        ]
    }
}

pub fn spiral_fractal() -> IFS {
    IFS {
        transforms: vec![
            (Affine { a:0.787879, b:-0.424242, c:1.758647, d:0.242424, e:0.859848, f:1.408065 }, 0.9),
            (Affine { a:-0.121212, b:0.257576, c:-6.721654, d:0.151515, e:0.053030, f:1.377236 }, 0.1),
        ]
    }
}

pub fn koch_curve() -> IFS {
    let s = 1.0 / 3.0;

    IFS {
        transforms: vec![
            (
                Affine::scale(s, s),
                1.0
            ),
            (
                Affine::scale(s, s)
                    .combine(Affine::rotate_deg(60.0))
                    .combine(Affine::translate(1.0/3.0, 0.0)),
                1.0
            ),
            (
                Affine::scale(s, s)
                    .combine(Affine::rotate_deg(-60.0))
                    .combine(Affine::translate(0.5, 0.288675)),
                1.0
            ),
            (
                Affine::scale(s, s)
                    .combine(Affine::translate(2.0/3.0, 0.0)),
                1.0
            )
        ]
    }
}

pub fn koch_snowflake() -> IFS {
    let s = 1.0 / 3.0;
    let h = 3.0_f64.sqrt() / 6.0;

    let t1 = Affine::scale(s, s);

    let t2 = Affine::scale(s, s)
        .combine(Affine::rotate_deg(60.0))
        .combine(Affine::translate(1.0/3.0, 0.0));

    let t3 = Affine::scale(s, s)
        .combine(Affine::rotate_deg(-60.0))
        .combine(Affine::translate(0.5, h));

    let t4 = Affine::scale(s, s)
        .combine(Affine::translate(2.0/3.0, 0.0));

    let base = vec![t1, t2, t3, t4];

    let r0 = Affine::id();
    let r120 = Affine::rotate_deg(120.0);
    let r240 = Affine::rotate_deg(240.0);

    let mut transforms = Vec::new();

    for r in [r0, r120, r240] {
        for t in &base {
            transforms.push((t.clone().combine(r), 1.0));
        }
    }

    IFS {transforms}
}

pub fn ifs_presets(name: &str) -> Option<IFS> {
    match name {
        "Sierpinski gasket" | "Sierpinski" | "sierpinski" | "gasket" | "triangle" | "tri" => Some(sierpinski_gasket()),

        "Barnsley fern" | "Barnsley" | "barnsley" | "fern" => Some(barnsley_gasket()),

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

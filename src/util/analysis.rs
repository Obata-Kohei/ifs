use crate::ifs::Point;

// フラクタルの最小・最大座標を取得
pub fn bounding_box(points: &[Point]) -> (f64, f64, f64, f64) {
    let mut xmin = f64::INFINITY;
    let mut xmax = f64::NEG_INFINITY;
    let mut ymin = f64::INFINITY;
    let mut ymax = f64::NEG_INFINITY;

    for p in points {
        if p.x < xmin {
            xmin = p.x;
        }
        if p.x > xmax {
            xmax = p.x;
        }
        if p.y < ymin {
            ymin = p.y;
        }
        if p.y > ymax {
            ymax = p.y;
        }
    }

    (xmin, xmax, ymin, ymax)
}

// アスペクト比を補正した(xmin, xmax, ymin, ymax)を返す
pub fn correct_aspect(
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    width: u32,
    height: u32,
) -> (f64, f64, f64, f64) {
    let mut xmin = xmin;
    let mut xmax = xmax;
    let mut ymin = ymin;
    let mut ymax = ymax;

    let x_range = xmax - xmin;
    let y_range = ymax - ymin;

    let fractal_aspect = x_range / y_range;  // ヨコ / タテの比率
    let image_aspect = width  as f64 / height as f64;

    if image_aspect < fractal_aspect {
        let new_y_range = x_range / image_aspect;
        let delta = (new_y_range - y_range) / 2.0;
        ymin -= delta;
        ymax += delta;
    } else {
        let new_x_range = y_range * image_aspect;
        let delta = (new_x_range - x_range) / 2.0;
        xmin -= delta;
        xmax += delta;
    }

    (xmin, xmax, ymin, ymax)
}

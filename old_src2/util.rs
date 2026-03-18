use image::{GrayImage, Luma};
use crate::core::affine::Point;

// 2x2の行列の固有値を求める[[a, b], [c, d]]
pub fn det2x2(a: f64, b: f64, c: f64, d: f64) -> f64 {
    a * d - b * c
}

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

// 点の配列から画像を生成
pub fn render(points: &[Point], width: u32, height: u32) -> GrayImage {
    let (xmin, xmax, ymin, ymax) = bounding_box(points);
    let (xmin, xmax, ymin, ymax) = correct_aspect(xmin, xmax, ymin, ymax, width, height);

    let mut counts = vec![0u32; (width * height) as usize];

    for p in points {
        let x = ((p.x - xmin) / (xmax - xmin) * width as f64) as u32;
        let y = ((ymax - p.y) / (ymax - ymin) * height as f64) as u32;

        if x < width && y < height {
            let idx = (y * width + x) as usize;
            counts[idx] += 1;
        }
    }

    let mut img = GrayImage::new(width, height);

    let max = counts.iter().copied().max().unwrap_or(1) as f64;
    let log_max = (max + 1.0).ln();

    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            let c = counts[idx] as f64;

            let v = if c == 0.0 {
                0
            } else {
                ((c + 1.0).ln() / log_max * 255.0) as u8
            };

            img.put_pixel(x, y, Luma([v]));
        }
    }

    img
}

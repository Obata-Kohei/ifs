use crate::ifs::Point;
use crate::util::analysis::*;
use image::{GrayImage, Luma};

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

// パラメタ保存

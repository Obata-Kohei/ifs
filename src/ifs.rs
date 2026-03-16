use rand::prelude::*;
use rand::distr::weighted::WeightedIndex;
use image::{GrayImage, Luma};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Affine {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

impl Affine {
    pub fn new(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64,) -> Self {
        Self {a, b, c, d, e, f}
    }

    pub fn apply(&self, p: Point) -> Point {
        Point {
            x: self.a * p.x + self.b * p.y + self.c,
            y: self.d * p.x + self.e * p.y + self.f,
        }
    }

    pub fn id() -> Self {
        Self {
            a:1.0, b:0.0, c:0.0,
            d:0.0, e:1.0, f:0.0,
        }
    }

    pub fn translate(x: f64, y: f64) -> Self {
        Self {
            a: 1.0, b: 0.0, c: x,
            d: 0.0, e: 1.0, f: y,
        }
    }

    pub fn scale(x: f64, y: f64) -> Self {
        Self {
            a: x, b: 0.0, c: 0.0,
            d: 0.0, e: y, f: 0.0,
        }
    }

    pub fn rotate_deg(deg: f64) -> Self {
        let t = deg.to_radians();
        let sin = t.sin();
        let cos = t.cos();

        Self {
            a: cos, b: -sin, c: 0.0,
            d: sin, e: cos, f: 0.0,
        }
    }

    pub fn shear(x: f64, y: f64) -> Self {
        Self {
            a: 1.0, b: x, c: 0.0,
            d: y, e: 1.0, f: 0.0,
        }
    }

    pub fn combine(self, other: Self) -> Self {
        Self {
            a: other.a*self.a + other.b*self.d,
            b: other.a*self.b + other.b*self.e,
            c: other.a*self.c + other.b*self.f + other.c,
            d: other.d*self.a + other.e*self.d,
            e: other.d*self.b + other.e*self.e,
            f: other.d*self.c + other.e*self.f + other.f,
        }
    }
}

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
}

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

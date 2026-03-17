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
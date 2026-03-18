use rand::prelude::*;

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

    // アフィン変換A, Bについて，A.combine(B)とすれば，Aを適用したのちBを適用することを意味する
    pub fn then(self, next: Self) -> Self {
        Self {
            a: next.a*self.a + next.b*self.d,
            b: next.a*self.b + next.b*self.e,
            c: next.a*self.c + next.b*self.f + next.c,
            d: next.d*self.a + next.e*self.d,
            e: next.d*self.b + next.e*self.e,
            f: next.d*self.c + next.e*self.f + next.f,
        }
    }

    // ランダムなアフィン変換を作る
    pub fn random_affine(rng: &mut ThreadRng) -> Self {
        let (sx, sy) = (rng.random_range(0.3..0.7), rng.random_range(0.3..0.7));
        let rot = rng.random_range(0.0..360.0);
        let (tx, ty) = (rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0));
        Self::scale(sx, sy)
            .then(Self::rotate_deg(rot))
            .then(Self::translate(tx, ty))
    }

    // 2×2アフィン行列の最大特異値（= スペクトルノルム = Lipschitz定数）
    fn spectral_norm(&self) -> f64 {
        let (a, b, c, d) = (self.a, self.b, self.c, self.d);
        let m = a*a + b*b + c*c + d*d;        // tr(AᵀA)
        let s = (a*d - b*c).powi(2);          // det(AᵀA) = det(A)²
        let discriminant = (m*m - 4.0*s).max(0.0);  // 数値誤差で負になる場合を防ぐ
        ((m + discriminant.sqrt()) / 2.0).sqrt()
    }
}

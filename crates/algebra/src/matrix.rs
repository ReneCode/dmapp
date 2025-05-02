//

use crate::point2d::Point2d;

pub struct Matrix {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
    g: f64,
    h: f64,
    i: f64,
}

impl Matrix {
    fn new(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, g: f64, h: f64, i: f64) -> Self {
        Self {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
            i,
        }
    }

    pub fn identity() -> Self {
        Self::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0)
    }

    pub fn translate(tx: f64, ty: f64) -> Self {
        Self::new(1.0, 0.0, tx, 0.0, 1.0, ty, 0.0, 0.0, 1.0)
    }
    pub fn scale(sx: f64, sy: f64) -> Self {
        Self::new(sx, 0.0, 0.0, 0.0, sy, 0.0, 0.0, 0.0, 1.0)
    }
    fn rotate(theta: f64) -> Self {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();
        Self::new(
            cos_theta, -sin_theta, 0.0, sin_theta, cos_theta, 0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn multiply(&self, point: &Point2d) -> Point2d {
        let x = self.a * point.x + self.b * point.y + self.c;
        let y = self.d * point.x + self.e * point.y + self.f;
        Point2d::new(x / (self.g * point.x + self.h * point.y + self.i), y)
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.a * (self.e * self.i - self.f * self.h)
            - self.b * (self.d * self.i - self.f * self.g)
            + self.c * (self.d * self.h - self.e * self.g);

        if det == 0.0 {
            return None; // No inverse exists
        }

        let inv_det = 1.0 / det;

        Some(Self::new(
            (self.e * self.i - self.f * self.h) * inv_det,
            (self.c * self.h - self.b * self.i) * inv_det,
            (self.b * self.f - self.c * self.e) * inv_det,
            (self.f * self.g - self.d * self.i) * inv_det,
            (self.a * self.i - self.c * self.g) * inv_det,
            (self.c * self.d - self.a * self.f) * inv_det,
            (self.d * self.h - self.e * self.g) * inv_det,
            (self.b * self.g - self.a * self.h) * inv_det,
            (self.a * self.e - self.b * self.d) * inv_det,
        ))
    }
}

impl std::ops::Mul for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Matrix {
        Matrix::new(
            self.a * other.a + self.b * other.d + self.c * other.g,
            self.a * other.b + self.b * other.e + self.c * other.h,
            self.a * other.c + self.b * other.f + self.c * other.i,
            self.d * other.a + self.e * other.d + self.f * other.g,
            self.d * other.b + self.e * other.e + self.f * other.h,
            self.d * other.c + self.e * other.f + self.f * other.i,
            self.g * other.a + self.h * other.d + self.i * other.g,
            self.g * other.b + self.h * other.e + self.i * other.h,
            self.g * other.c + self.h * other.f + self.i * other.i,
        )
    }
}
impl std::ops::Add for Matrix {
    type Output = Matrix;

    fn add(self, other: Matrix) -> Matrix {
        Matrix::new(
            self.a + other.a,
            self.b + other.b,
            self.c + other.c,
            self.d + other.d,
            self.e + other.e,
            self.f + other.f,
            self.g + other.g,
            self.h + other.h,
            self.i + other.i,
        )
    }
}
impl std::ops::Sub for Matrix {
    type Output = Matrix;

    fn sub(self, other: Matrix) -> Matrix {
        Matrix::new(
            self.a - other.a,
            self.b - other.b,
            self.c - other.c,
            self.d - other.d,
            self.e - other.e,
            self.f - other.f,
            self.g - other.g,
            self.h - other.h,
            self.i - other.i,
        )
    }
}
impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[[{}, {}, {}], [{}, {}, {}], [{}, {}, {}]]",
            self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h, self.i
        )
    }
}
impl std::fmt::Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Matrix3x3 {{ a: {}, b: {}, c: {}, d: {}, e: {}, f: {}, g: {}, h: {}, i: {} }}",
            self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h, self.i
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate() {
        let pt = Point2d::new(10.0, 20.0);
        let translate = Matrix::translate(50.0, 80.0);
        let result = translate.multiply(&pt);
        assert_eq!(result.x, 60.0);
        assert_eq!(result.y, 100.0);
    }

    #[test]
    fn scale() {
        let pt = Point2d::new(10.0, 20.0);
        let scale = Matrix::scale(2.0, 3.0);
        let result = scale.multiply(&pt);
        assert_eq!(result.x, 20.0);
        assert_eq!(result.y, 60.0);
    }
}

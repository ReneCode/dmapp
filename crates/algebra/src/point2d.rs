pub struct Point2D {
    pub x: f64,
    pub y: f64,
}
impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Point2D) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}
impl std::ops::Add for Point2D {
    type Output = Point2D;

    fn add(self, other: Point2D) -> Point2D {
        Point2D::new(self.x + other.x, self.y + other.y)
    }
}
impl std::ops::Sub for Point2D {
    type Output = Point2D;

    fn sub(self, other: Point2D) -> Point2D {
        Point2D::new(self.x - other.x, self.y - other.y)
    }
}
impl std::ops::Mul<f64> for Point2D {
    type Output = Point2D;

    fn mul(self, scalar: f64) -> Point2D {
        Point2D::new(self.x * scalar, self.y * scalar)
    }
}
impl std::ops::Div<f64> for Point2D {
    type Output = Point2D;

    fn div(self, scalar: f64) -> Point2D {
        Point2D::new(self.x / scalar, self.y / scalar)
    }
}
impl std::fmt::Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point2D({}, {})", self.x, self.y)
    }
}
impl std::fmt::Debug for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point2D {{ x: {}, y: {} }}", self.x, self.y)
    }
}

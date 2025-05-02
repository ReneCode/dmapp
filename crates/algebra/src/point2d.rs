use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}
#[wasm_bindgen]
impl Point2d {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Point2d) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}
impl std::ops::Add for Point2d {
    type Output = Point2d;

    fn add(self, other: Point2d) -> Point2d {
        Point2d::new(self.x + other.x, self.y + other.y)
    }
}
impl std::ops::Sub for Point2d {
    type Output = Point2d;

    fn sub(self, other: Point2d) -> Point2d {
        Point2d::new(self.x - other.x, self.y - other.y)
    }
}
impl std::ops::Mul<f64> for Point2d {
    type Output = Point2d;

    fn mul(self, scalar: f64) -> Point2d {
        Point2d::new(self.x * scalar, self.y * scalar)
    }
}
impl std::ops::Div<f64> for Point2d {
    type Output = Point2d;

    fn div(self, scalar: f64) -> Point2d {
        Point2d::new(self.x / scalar, self.y / scalar)
    }
}
impl std::fmt::Display for Point2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point2D({}, {})", self.x, self.y)
    }
}

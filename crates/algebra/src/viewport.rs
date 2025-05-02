//

use crate::{Matrix, Point2d};

pub fn round(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}
#[derive(Debug)]
pub struct Viewport {
    canvas_width: f64,
    canvas_height: f64,
    canvas_id: String,

    pub scale: f64,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Viewport {
    pub fn new() -> Self {
        Self {
            canvas_width: 600.0,
            canvas_height: 400.0,
            canvas_id: String::new(),
            scale: 1.0,

            x: -200.0,
            y: -200.0,
            width: 400.0,
            height: 400.0,
        }
    }

    pub fn set_canvas_id(&mut self, canvas_id: String) {
        self.canvas_id = canvas_id;
    }

    pub fn get_canvas_id(&self) -> &str {
        &self.canvas_id
    }

    pub fn set_canvas_size(&mut self, width: f64, height: f64) {
        self.canvas_width = width;
        self.canvas_height = height;
        self.x = -width / 2.0;
        self.y = -height / 2.0;
        self.width = width;
        self.height = height;
        self.scale = 1.0;
    }

    // fix width or height if the ratio does not fit to the ratio of the canvas
    pub fn set_viewport(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let canvas_ratio = self.canvas_width / self.canvas_height;
        let new_width = height * canvas_ratio;
        let new_height = width / canvas_ratio;

        self.x = x;
        self.y = y;
        if width < new_width {
            self.width = new_width;
        } else {
            self.width = width;
        }
        if height < new_height {
            self.height = new_height;
        } else {
            self.height = height;
        }

        self.scale = self.canvas_width / self.width;
    }

    pub fn zoom_viewport(&mut self, delta_y: f64, center_x: f64, center_y: f64) {
        // Zoom the viewport based on the mouse wheel event

        let MAX_DELTA = 10.0;
        let mut delta = delta_y.abs().min(MAX_DELTA);
        let sign = -delta_y.signum();
        delta = delta * sign;

        let old_scale = self.scale;
        let new_scale = old_scale * (1.0 + delta / 100.0);

        let x = self.x + center_x / self.scale;
        let y = self.y + center_y / self.scale;

        let new_x = x - (old_scale / new_scale) * (x - self.x);
        let new_y = y - (old_scale / new_scale) * (y - self.y);

        self.x = new_x;
        self.y = new_y;
        self.scale = new_scale;
        self.width = self.canvas_width / new_scale;
        self.height = self.canvas_height / new_scale;
    }

    pub fn panning_viewport(&mut self, delta_x: f64, delta_y: f64) {
        self.x = round(self.x + delta_x / self.scale);
        self.y = round(self.y + delta_y / self.scale);
    }

    pub fn client_to_canvas(&self, pt: Point2d) -> Point2d {
        // Convert client coordinates to canvas coordinates
        let x = (pt.x + self.x * self.scale) / self.scale;
        let y = (-pt.y - self.y * self.scale) / self.scale;
        Point2d::new(x, y)
    }

    // ------------------------
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_viewport() {
        let mut viewport = Viewport::new();
        viewport.set_canvas_size(800.0, 600.0);
        viewport.set_viewport(10.0, 10.0, 80.0, 60.0);
        viewport.set_canvas_id("canvas".to_string());

        assert_eq!(viewport.canvas_width, 800.0);
        assert_eq!(viewport.canvas_height, 600.0);
        assert_eq!(viewport.canvas_id, "canvas");
        assert_eq!(viewport.x, 10.0);
        assert_eq!(viewport.y, 10.0);
        assert_eq!(viewport.width, 80.0);
        assert_eq!(viewport.height, 60.0);
        assert_eq!(viewport.scale, 10.0);
    }

    #[test]
    fn test_set_viewport_height_larger() {
        let mut viewport = Viewport::new();
        viewport.set_canvas_size(800.0, 400.0);
        viewport.set_viewport(10.0, 20.0, 50.0, 40.0);

        assert_eq!(viewport.x, 10.0);
        assert_eq!(viewport.width, 80.0); // width is fixed
        assert_eq!(viewport.y, 20.0);
        assert_eq!(viewport.height, 40.0);
        assert_eq!(viewport.scale, 10.0);
    }

    #[test]
    fn test_set_viewport_width_larger() {
        let mut viewport = Viewport::new();
        viewport.set_canvas_size(800.0, 400.0);
        viewport.set_viewport(100.0, 100.0, 80.0, 10.0);

        assert_eq!(viewport.x, 100.0);
        assert_eq!(viewport.width, 80.0);
        assert_eq!(viewport.y, 100.0);
        assert_eq!(viewport.height, 40.0);
        assert_eq!(viewport.scale, 10.0);
    }

    #[test]
    fn test_set_viewport_scale() {
        let mut viewport = Viewport::new();
        viewport.set_canvas_size(800.0, 400.0);
        viewport.set_viewport(0.0, 0.0, 1200.0, 600.0);

        assert_eq!(viewport.scale, 1.0 / 1.5);
    }

    #[test]
    fn test_zoom_viewport() {
        let mut viewport = Viewport::new();
        viewport.set_canvas_size(800.0, 400.0);
        viewport.set_viewport(0.0, 0.0, 800.0, 400.0);

        viewport.zoom_viewport(1.0, 400.0, 200.0);

        assert_eq!(round(viewport.scale), 0.99);
        assert_eq!(round(viewport.x), -4.04);
        assert_eq!(round(viewport.y), -2.02);
        assert_eq!(round(viewport.width), 808.08);
        assert_eq!(round(viewport.height), 404.04);
    }
}

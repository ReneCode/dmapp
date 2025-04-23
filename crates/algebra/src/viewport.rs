//

use crate::Matrix;

pub struct Viewport {
    canvas_width: f64,
    canvas_height: f64,
    canvas_id: String,
    scale: f64,
    center_x: f64,
    center_y: f64,

    wc_to_canvas: Matrix,
    canvas_to_wc: Matrix,
}

impl Viewport {
    pub fn new() -> Self {
        Self {
            canvas_width: 400.0,
            canvas_height: 400.0,
            canvas_id: String::new(),
            scale: 1.0,
            center_x: 0.0,
            center_y: 0.0,
            wc_to_canvas: Matrix::identity(),
            canvas_to_wc: Matrix::identity(),
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
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
    }

    pub fn set_center(&mut self, x: f64, y: f64) {
        self.center_x = x;
        self.center_y = y;
    }

    // pub fn wc_to_canvas(&self, point: &Point2D) -> Point2D {
    //     // Convert a point from world coordinates to canvas coordinates
    //     self.wc_to_canvas.multiply(point)
    // }
    // pub fn canvas_to_wc(&self, point: &Point2D) -> Point2D {
    //     // Convert a point from canvas coordinates to world coordinates
    //     self.canvas_to_wc.multiply(point)
    // }

    // ------------------------

    fn update_matrices(&mut self) {
        // Update the world-to-canvas and canvas-to-world matrices based on the current scale and center
        self.wc_to_canvas = Matrix::scale(self.scale, self.scale)
            * Matrix::translate(-self.center_x, -self.center_y);
        self.canvas_to_wc = self.wc_to_canvas.inverse().unwrap();
    }
}

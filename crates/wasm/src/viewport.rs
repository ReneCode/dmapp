//

pub struct Viewport {
    canvas_width: f64,
    canvas_height: f64,
    canvas_id: String,
    scale: f64,
    offset_x: f64,
    offset_y: f64,
}

impl Viewport {
    pub fn new() -> Self {
        Self {
            canvas_width: 400.0,
            canvas_height: 400.0,
            canvas_id: String::new(),
            scale: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
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

    pub fn set_offset(&mut self, x: f64, y: f64) {
        self.offset_x = x;
        self.offset_y = y;
    }
}

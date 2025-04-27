//

use algebra::Viewport;
use wasm_bindgen::prelude::*;

use command::{CommandHandler, CommandLine};
use datamodel::DataModel;
use render::Renderer;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn console_log(s: &str) {
    log(s);
}

#[wasm_bindgen]
pub struct ECAPI {
    // Add fields here if needed
    data_model: DataModel,
    command_handler: CommandHandler,
    viewport: Viewport,
}

#[wasm_bindgen]
impl ECAPI {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let ecapi = ECAPI {
            data_model: DataModel::default(),
            command_handler: CommandHandler::default(),
            viewport: Viewport::new(),
        };
        log("WASM ECAPI initialized");
        ecapi
    }

    #[wasm_bindgen]
    pub fn init(&mut self, canvas_id: String) {
        // Initialize the data model or any other necessary components
        self.viewport.set_canvas_id(canvas_id);
    }

    #[wasm_bindgen]
    pub fn render_current_page(&mut self) {
        // Find the page by ID

        if let Some(page) = self
            .data_model
            .get_page(&self.data_model.get_current_page_id())
        {
            let renderer = Renderer::new(&self.data_model, &self.viewport);
            renderer.render_page(&page);
            log(format!("Rendering page: {}", page.get_name()).as_str());
        } else {
            log("Page not found");
        }
    }

    pub fn zoom_viewport(&mut self, delta_y: f64, center_x: f64, center_y: f64) {
        // Zoom the viewport based on the mouse wheel event
        self.viewport.zoom_viewport(delta_y, center_x, center_y);

        log(format!("{:?}", self.viewport).as_str());

        self.render_current_page();
        log(format!("Zooming viewport: {} {} {}", delta_y, center_x, center_y).as_str());
    }

    #[wasm_bindgen]
    pub fn resize_canvas(&mut self, width: f64, height: f64) {
        log(format!("Resizing canvas to {}x{}", width, height).as_str());
        // Set the viewport dimensions
        self.viewport.set_canvas_size(width, height);
    }

    #[wasm_bindgen]
    pub fn create_page(&mut self, name: String) -> String {
        // Create a new page in the data model
        let id = self.data_model.next_id();
        let page = datamodel::Page::new(id.clone(), name.clone(), "page description".to_string());
        self.data_model.insert_page(page);
        id
    }

    #[wasm_bindgen]
    pub fn run_command(&mut self, command_line: String) {
        CommandLine::parse(&mut self.data_model, command_line.as_str())
            .map(|cmd| {
                self.command_handler.execute(&mut self.data_model, cmd);
            })
            .unwrap_or_else(|err| {
                eprintln!("Error: {}", err);
                log(&format!("Error: {}", err));
            });
    }

    #[wasm_bindgen]
    pub fn get_version(&self) -> String {
        "Hi DataModel, wasm from Rust!".to_string()
    }
}

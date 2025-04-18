//

use wasm_bindgen::prelude::*;

use command::{CommandHandler, CommandLine};
use datamodel::DataModel;
use render::render_page;

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
    canvas_id: String,
}

#[wasm_bindgen]
impl ECAPI {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        ECAPI {
            data_model: DataModel::default(),
            command_handler: CommandHandler::default(),
            canvas_id: String::new(),
        }
    }

    #[wasm_bindgen]
    pub fn init(&mut self, canvas_id: String) {
        // Initialize the data model or any other necessary components
        self.canvas_id = canvas_id;
    }

    #[wasm_bindgen]
    pub fn render_current_page(&mut self) {
        // Find the page by ID

        if let Some(page) = self
            .data_model
            .get_page(&self.data_model.get_current_page_id())
        {
            render_page(&self.data_model, &page, &self.canvas_id);
        } else {
            log("Page not found");
        }
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
                log("success");
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

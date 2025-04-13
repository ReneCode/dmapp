//

use wasm_bindgen::prelude::*;

use command::{CommandHandler, CreatePageCommand};
use datamodel::DataModel;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    log("Hello, wasm from Rust!");
}

#[wasm_bindgen]
pub struct EDataModel {
    // Add fields here if needed
    datamodel: DataModel,
    commandhandler: CommandHandler,
}

#[wasm_bindgen]
impl EDataModel {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        EDataModel {
            datamodel: DataModel::default(),
            commandhandler: CommandHandler::default(),
        }
    }

    pub fn get_data(&self) -> String {
        "Hi DataModel, wasm from Rust!".to_string()
    }

    pub fn create_page(&mut self, name: String) {
        log(&format!("create page: {}", name));

        let id = self.datamodel.next_id();
        let cmd = CreatePageCommand::new(id, name, "Page Description".to_string());

        self.commandhandler
            .execute(&mut self.datamodel, Box::new(cmd));
        // Here you can set the data in your datamodel
        // self.datamodel.set_data(data);
    }
}

use gloo_utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
//

use algebra::{Point2D, Viewport};
use datamodel::{Line, Node};
use wasm_bindgen::prelude::*;

use command::{CommandHandler, CommandLine};
use datamodel::DataModel;
use render::Renderer;

#[derive(Debug, Serialize, Deserialize)]
struct BaseNode {
    id: String,
    node_type: String,
}

#[wasm_bindgen]
struct Point2DWrapper {
    x: f64,
    y: f64,
}
impl Point2DWrapper {
    fn new(x: f64, y: f64) -> Self {
        Point2DWrapper { x, y }
    }
}
impl From<Point2DWrapper> for Point2D {
    fn from(wrapper: Point2DWrapper) -> Self {
        Point2D::new(wrapper.x, wrapper.y)
    }
}
impl From<Point2D> for Point2DWrapper {
    fn from(point: Point2D) -> Self {
        Point2DWrapper::new(point.x, point.y)
    }
}

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
        } else {
            log("Page not found");
        }
    }

    pub fn zoom_viewport(&mut self, delta_y: f64, center_x: f64, center_y: f64) {
        // Zoom the viewport based on the mouse wheel event
        self.viewport.zoom_viewport(delta_y, center_x, center_y);

        self.render_current_page();
    }

    pub fn panning_viewport(&mut self, delta_x: f64, delta_y: f64) {
        // Pan the viewport based on mouse movement
        self.viewport.panning_viewport(delta_x, delta_y);

        self.render_current_page();
    }

    #[wasm_bindgen]
    pub fn resize_canvas(&mut self, width: f64, height: f64) {
        // Set the viewport dimensions
        self.viewport.set_canvas_size(width, height);

        log("resize canvas");
    }

    #[wasm_bindgen]
    pub fn get_selection(&self) -> Vec<String> {
        if let Some(page) = self.data_model.get_current_page() {
            // Get the selected node IDs from the page
            let selected_ids = page.get_selected_ids();
            let result = selected_ids.clone();
            // log(&format!("Selected IDs: {:?}", result));
            return result;
        } else {
            log("No page found");
            return vec![];
        }
    }

    pub fn set_selection(&mut self, ids: Vec<String>) {
        // Set the selected node IDs in the current page
        if let Some(page) = self.data_model.get_current_page_mut() {
            page.set_selected_ids(ids.clone());
        } else {
            log("No page found");
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
    pub fn create_line(&mut self) -> Result<JsValue, JsValue> {
        // Create a new line in the data model
        // and add it to the current page
        if let None = self.data_model.get_current_page() {
            log("No current page found");
            return Err(JsValue::from_str("No current page found"));
        }

        let id = self.data_model.next_id();
        let line = datamodel::Line::new(id.clone());

        let a = JsValue::from_serde(&line).unwrap();

        let result = serde_wasm_bindgen::to_value(&line)?;

        self.data_model.insert_node(Box::new(line));
        if let Some(page) = self.data_model.get_current_page_mut() {
            page.add_node_id(id.clone());
        } else {
            log("No page found");
        }

        Ok(result)
    }

    pub fn patch_node(&mut self, patch: JsValue) -> Result<JsValue, JsValue> {
        let base_node = patch.into_serde::<BaseNode>().unwrap();
        match base_node.node_type.as_str() {
            "Line" => {
                let patch_line: Line = serde_wasm_bindgen::from_value(patch)?;
                if let Some(node) = self.data_model.get_node_mut(&patch_line.get_id()) {
                    if let Some(line) = node.as_any_mut().downcast_mut::<Line>() {
                        // Update the line properties
                        line.x1 = patch_line.x1;
                        line.y1 = patch_line.y1;
                        line.x2 = patch_line.x2;
                        line.y2 = patch_line.y2;
                    } else {
                        log("Failed to downcast node to Line");
                    }
                } else {
                    log("Node not found");
                }
            }
            _ => {
                log("Unknown node type");
            }
        }

        Ok(JsValue::NULL)
    }

    pub fn do_callback(&mut self, callback: &str) {
        // Call the JavaScript callback function
        let js_callback = js_sys::Function::new_no_args(callback);
        js_callback.call0(&JsValue::NULL).unwrap();
    }

    #[wasm_bindgen]
    pub fn do_callback1(&mut self, callback: &js_sys::Function) {
        // Call the JavaScript callback function with a string argument

        let s = format!("Callback called with message: {}", 4711);

        let pt = Point2D::new(1.0, 2.0);

        let ptw = Point2DWrapper::from(pt);
        let a = JsValue::from(ptw);

        callback.call1(&JsValue::NULL, &a).unwrap_or_else(|err| {
            log(&format!("Error calling callback: {:?}", err));
            err
        });
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

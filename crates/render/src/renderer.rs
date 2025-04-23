//

use algebra::Viewport;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, SvgElement, Window};

use datamodel::{Arc, DataModel, Line, Node, Page};

use crate::transform::Transform;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn round(x: f64) -> String {
    let r = (x * 100.0).round() / 100.0;
    r.to_string()
}

trait Graphic: Node {
    fn render(&self, document: &Document, svg_parent: &Element) -> Result<web_sys::Node, JsValue>;
}

impl Graphic for Line {
    fn render(&self, document: &Document, svg_parent: &Element) -> Result<web_sys::Node, JsValue> {
        let svg_line = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "line")?;

        svg_line.set_attribute("id", self.get_id())?;
        svg_line.set_attribute("x1", &round(self.get_x1()))?;
        svg_line.set_attribute("y1", &round(self.get_y1()))?;
        svg_line.set_attribute("x2", &round(self.get_x2()))?;
        svg_line.set_attribute("y2", &round(self.get_y2()))?;

        svg_line.set_attribute("stroke", &"black")?;
        svg_line.set_attribute("stroke-width", &round(1.0))?;

        svg_parent.append_child(&svg_line)
    }
}

impl Graphic for Arc {
    fn render(&self, document: &Document, svg_parent: &Element) -> Result<web_sys::Node, JsValue> {
        let svg_arc = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;

        //TODO: use the angle_start and angle_end and the values of the Arc struct

        svg_arc.set_attribute("id", "ABC")?;
        svg_arc.set_attribute("cx", "10")?;
        svg_arc.set_attribute("cy", "10")?;
        svg_arc.set_attribute("r", &"20")?;

        svg_parent.append_child(&svg_arc)
    }
}

pub struct Renderer<'a> {
    data_model: &'a DataModel,
    viewport: &'a Viewport,
    window: Window,
    document: Document,
}

impl<'a> Renderer<'a> {
    pub fn new(data_model: &'a DataModel, viewport: &'a Viewport) -> Self {
        let window: Window = web_sys::window().expect("no global `window` exists");
        let document: Document = window.document().expect("should have a document on window");

        Self {
            data_model,
            viewport,
            window,
            document,
        }
    }

    pub fn render_page(&self, page: &Page) -> Result<(), JsValue> {
        let svg_canvas: SvgElement = self.get_svg_element();
        svg_canvas.set_inner_html(""); // Clear the canvas

        let root_group = self.create_root_group()?;
        svg_canvas.append_child(&root_group)?;

        self.render_nodes(&root_group, &page.get_node_ids())?;

        Ok(())
    }

    // ----------------------

    fn get_svg_element(&self) -> SvgElement {
        let svg_canvas: SvgElement = self
            .document
            .get_element_by_id(self.viewport.get_canvas_id())
            .expect("No element with your svg_canvas_id")
            .dyn_into::<SvgElement>()
            .map_err(|_| ())
            .expect("The element with your_svg_id is not an SVGElement");

        svg_canvas
    }

    fn create_root_group(&self) -> Result<SvgElement, JsValue> {
        let svg_group = self
            .document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "g")
            .expect("can't create svg group")
            .dyn_into::<SvgElement>()
            .map_err(|_| ())
            .expect("The element with your_svg_id is not an SVGElement");

        svg_group.set_attribute("id", "root_group")?;
        let transform = Transform {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            dx: 40.0,
            dy: 80.0,
        };
        let matrix = format!(
            "matrix({},{},{},{},{},{})",
            &round(transform.a),
            &round(transform.b),
            &round(transform.c),
            &round(transform.d),
            &round(transform.dx),
            &round(transform.dy)
        );
        svg_group.set_attribute("transform", &matrix)?;

        Ok(svg_group)

        // // svg_group.set_attribute("transform", "transform(matrix(1,0,0,1,40,80))")?;
        // svg_canvas.append_child(&svg_group)?;

        // render_page(page, canvas_id)
    }

    fn render_nodes(&self, svg_parent: &SvgElement, node_ids: &Vec<String>) -> Result<(), JsValue> {
        for id in node_ids {
            if let Some(node) = self.data_model.get_node(id) {
                if let Some(line) = node.as_any().downcast_ref::<Line>() {
                    line.render(&self.document, &svg_parent)?;
                } else if let Some(arc) = node.as_any().downcast_ref::<Arc>() {
                    arc.render(&self.document, &svg_parent)?;
                }
            }
        }
        Ok(())
    }
}

//

use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, SvgElement, Window};

use datamodel::{Arc, DataModel, Line, Node, Page};

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
    fn render(
        &self,
        document: &mut Document,
        svg_parent: &Element,
    ) -> Result<web_sys::Node, JsValue>;
}

impl Graphic for Line {
    fn render(
        &self,
        document: &mut Document,
        svg_parent: &Element,
    ) -> Result<web_sys::Node, JsValue> {
        log(&format!(
            "x1: {}, y1: {}, x2: {}, y2: {}",
            self.get_x1(),
            self.get_y1(),
            self.get_x2(),
            self.get_y2()
        ));

        let svg_line = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "line")?;

        svg_line.set_attribute("id", self.get_id())?;
        svg_line.set_attribute("x1", &round(self.get_x1()))?;
        svg_line.set_attribute("y1", &round(self.get_y1()))?;
        svg_line.set_attribute("x2", &round(self.get_x2()))?;
        svg_line.set_attribute("y2", &round(self.get_y2()))?;

        svg_line.set_attribute("stroke", &"black")?;
        svg_line.set_attribute("stroke-width", &round(3.0))?;

        svg_parent.append_child(&svg_line)
    }
}

impl Graphic for Arc {
    fn render(
        &self,
        document: &mut Document,
        svg_parent: &Element,
    ) -> Result<web_sys::Node, JsValue> {
        let svg_arc = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;

        //TODO: use the angle_start and angle_end and the values of the Arc struct

        svg_arc.set_attribute("id", "ABC")?;
        svg_arc.set_attribute("cx", "10")?;
        svg_arc.set_attribute("cy", "10")?;
        svg_arc.set_attribute("r", &"20")?;

        svg_parent.append_child(&svg_arc)
    }
}

pub fn render_page(data_model: &DataModel, page: &Page, canvas_id: &str) {
    render_nodes(data_model, canvas_id, &page.get_node_ids()).ok();
}

fn render_nodes(
    data_model: &DataModel,
    canvas_id: &str,
    node_ids: &Vec<String>,
) -> Result<(), JsValue> {
    let window: Window = web_sys::window().expect("no global `window` exists");
    let mut document: Document = window.document().expect("should have a document on window");

    let svg_canvas: SvgElement = get_svg_element(&document, canvas_id);
    svg_canvas.set_inner_html(""); // Clear the canvas

    log(&format!(
        "canvas_id: {}, node_ids: {:?}",
        canvas_id, node_ids
    ));
    for id in node_ids {
        log(&format!("rendering node {}", id));
        if let Some(node) = data_model.get_node(id) {
            if let Some(line) = node.as_any().downcast_ref::<Line>() {
                line.render(&mut document, &svg_canvas)?;
            } else if let Some(arc) = node.as_any().downcast_ref::<Arc>() {
                arc.render(&mut document, &svg_canvas)?;
            }
        }
    }
    Ok(())
}

fn get_svg_element(document: &Document, canvas_id: &str) -> SvgElement {
    let svg_canvas: SvgElement = document
        .get_element_by_id(canvas_id)
        .expect("No element with your svg_canvas_id")
        .dyn_into::<SvgElement>()
        .map_err(|_| ())
        .expect("The element with your_svg_id is not an SVGElement");

    svg_canvas
}

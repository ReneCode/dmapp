//

import { ECEvent } from "./Event";
import { BaseTool } from "./BaseTool";
import { Point2d } from "wasm";

export class SelectTool extends BaseTool {
  nr: number = 0;

  handleEvent(event: ECEvent): void {
    switch (event.type) {
      case "mouse_down":
        // console.log("Mouse down at:", event.clientX, event.clientY);

        const selection = this.editor.api.get_selection();
        // console.log("Current selection:", selection);
        selection.push(`${++this.nr}`);
        this.editor.api.set_selection(selection);

        let line = this.editor.api.create_line();
        // console.log("Line created:", line);

        let pt = new Point2d(event.clientX, event.clientY);
        let svg_pt = this.editor.api.client_to_canvas(pt);

        line.x1 = svg_pt.x;
        line.y1 = svg_pt.y;

        this.editor.api.patch_node(line);

        this.editor.api.render_current_page();

        break;
      // Add more event types as needed
    }
  }
}

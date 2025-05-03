//

import { ECEvent } from "./Event";
import { BaseTool } from "./BaseTool";

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

        line.x1 = event.canvasX;
        line.y1 = event.canvasY;

        line.x2 = event.canvasX + 100;
        line.y2 = event.canvasY + 100;

        this.editor.api.patch_node(line);

        this.editor.api.render_current_page();

        break;
      // Add more event types as needed
    }
  }
}

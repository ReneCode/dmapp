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
        console.log("Current selection:", selection);
        selection.push(`${++this.nr}`);
        this.editor.api.set_selection(selection);
        break;
      // Add more event types as needed
    }
  }
}

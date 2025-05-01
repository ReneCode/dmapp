//

import { ECEvent } from "./Event";
import { BaseTool } from "./BaseTool";

export class CommandTool extends BaseTool {
  handleEvent(event: ECEvent): void {
    if (event.type === "command") {
      // Handle command event

      this.editor.activateTool(event.command);

      return;
    }
  }
}

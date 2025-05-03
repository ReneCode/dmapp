//

import { ECEvent } from "./Event";
import { BaseTool } from "./BaseTool";

export class CommandTool extends BaseTool {
  handleEvent(event: ECEvent): void {
    if (event.type !== "command") {
      throw new Error("CommandTool can only handle command events");
    }

    this.editor.activateTool(event.command);
  }
}

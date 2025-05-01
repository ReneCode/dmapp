//

import { CommandTool } from "./tool/CommandTool";
import { ECEvent } from "./tool/Event";
import { BaseTool } from "./tool/BaseTool";
import { createNewTool } from "./tool/ToolFactory";
import { ECAPI } from "wasm";

export class Editor {
  private activeTool: BaseTool | null = null;

  constructor(public api: ECAPI) {}

  activateTool(toolName: string) {
    const newTool = createNewTool(toolName, this);
    if (newTool) {
      if (this.activeTool) {
        this.activeTool.stop();
      }
      this.activeTool = newTool;
      this.activeTool.start();
    } else {
      console.error(`Tool ${toolName} not recognized`);
    }
  }

  getActiveTool(): BaseTool | null {
    return this.activeTool;
  }

  dispatchEvent(event: ECEvent) {
    // global Events - not tool specific
    if (event.type === "command") {
      const cmdTool = new CommandTool(this);
      cmdTool.handleEvent(event);
    } else {
      if (this.activeTool) {
        this.activeTool.handleEvent(event);
      }
    }
  }
}

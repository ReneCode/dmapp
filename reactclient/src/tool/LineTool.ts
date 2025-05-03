//

import { BaseTool } from "./BaseTool";
import { ECEvent, MouseDownEvent } from "./Event";

function dispatchEvent(machine: { state: string }, event: ECEvent) {
  let curState = (machine as any)[machine.state];
  if (!curState) {
    console.error("Invalid state:", machine.state);
    return;
  }
  const on = curState.on;

  switch (event.type) {
    case "mouse_down":
      {
        const todo = on.mouse_down;
        if (todo) {
          todo.action(event);
          if (todo.target) {
            machine.state = todo.target;
          }
        }
      }
      break;
    case "mouse_move":
      {
        const todo = on.mouse_move;
        if (todo) {
          todo.action(event);
          if (todo.target) {
            machine.state = todo.target;
          }
        }
      }
      break;
    case "mouse_up":
      break;
  }
}

export class LineTool extends BaseTool {
  machine = {
    state: "idle",
    idle: {
      on: {
        mouse_down: {
          action: (ev: MouseDownEvent) => {
            this.line = this.editor.api.create_line();
            this.line.x1 = ev.canvasX;
            this.line.y1 = ev.canvasY;
            this.editor.api.patch_node(this.line);
          },
          target: "got_point1",
        },
      },
    },
    got_point1: {
      on: {
        mouse_down: {
          action: (ev: MouseDownEvent) => {
            this.line.x2 = ev.canvasX;
            this.line.y2 = ev.canvasY;
            this.editor.api.patch_node(this.line);
            this.editor.api.render_current_page();
          },
          target: "idle",
        },
        mouse_move: {
          action: (ev: MouseDownEvent) => {
            this.line.x2 = ev.canvasX;
            this.line.y2 = ev.canvasY;
            this.editor.api.patch_node(this.line);
            this.editor.api.render_current_page();
          },
        },
      },
    },
  };

  line: any;

  handleEvent(event: ECEvent): void {
    switch (event.type) {
      case "mouse_down":
        dispatchEvent(this.machine, event);
        break;

      case "mouse_move":
        dispatchEvent(this.machine, event);
        break;
      case "mouse_up":
        break;
      // Add more event types as needed
    }
  }

  stop() {}
}

//

import { BaseTool } from "./BaseTool";
import { ECEvent, MouseDownEvent } from "./Event";
import { setup, createMachine, assign, createActor } from "xstate";

export class LineTool extends BaseTool {
  /*
  machine = createMachine({
    context: {
      canvasX: 0.0,
      canvasY: 0.0,

      point1: { x: 0, y: 0 },
      point2: { x: 0, y: 0 },
    },
    id: "lineTool",
    initial: "idle",
    states: {
      idle: {
        on: {
          mouse_down: {
            actions: assign({
              point1: (context, event) => ({
                x: event.canvasX,
                y: event.canvasY,
              }),
            }),
            target: "got_point1",
          },
        },
      },
      got_point1: {
        on: {
          mouse_down: {
            target: "got_point2",
            actions: "takeSecondPoint",
          },
        },
      },
    },
  });

  actor = createActor(this.machine).start();

  */
  // state:
  //   | {
  //       name: "idle";
  //     }
  //   | {
  //       name: "got_point1";
  //       point1: { x: number; y: number };
  //     }
  //   | {
  //       name: "got_point2";
  //       point1: { x: number; y: number };
  //       point2: { x: number; y: number };
  //     } = {
  //   name: "idle",
  // };

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

    dispatchEvent(event: ECEvent) {
      let curState = (this as any)[this.state];
      if (!curState) {
        console.error("Invalid state:", this.state);
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
                this.state = todo.target;
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
                this.state = todo.target;
              }
            }
          }
          break;
        case "mouse_up":
          break;
      }
    },
  };

  line: any;

  handleEvent(event: ECEvent): void {
    switch (event.type) {
      case "mouse_down":
        this.machine.dispatchEvent(event);
        break;

      case "mouse_move":
        this.machine.dispatchEvent(event);
        break;
      case "mouse_up":
        break;
      // Add more event types as needed
    }
  }

  stop() {}
}

//

export type MouseDownEvent = {
  type: "mouse_down";
  clientX: number;
  clientY: number;
  canvasX: number;
  canvasY: number;
};

export type ECEvent =
  | {
      type: "command";
      command: string;
    }
  | MouseDownEvent
  | {
      type: "mouse_move";
      clientX: number;
      clientY: number;
      canvasX: number;
      canvasY: number;
    }
  | {
      type: "mouse_up";
      clientX: number;
      clientY: number;
      canvasX: number;
      canvasY: number;
    }
  | {
      type: "key_down";
      keyCode: string;
    }
  | {
      type: "key_up";
      keyCode: string;
    };

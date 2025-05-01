//

import { Editor } from "../Editor";
import { ECEvent } from "./Event";

export abstract class BaseTool {
  constructor(protected editor: Editor) {}

  abstract handleEvent(event: ECEvent): void;
  start() {}
  stop() {}
}

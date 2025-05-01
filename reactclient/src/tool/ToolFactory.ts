//

import { Editor } from "../Editor";
import { BaseTool } from "./BaseTool";
import { SelectTool } from "./SelectTool";

export function createNewTool(
  toolName: string,
  editor: Editor
): BaseTool | null {
  switch (toolName) {
    case "select":
      return new SelectTool(editor);
      break;
    // case "Text":
    //   tool = new TextTool(editor);
    //   break;
    // case "Image":
    //   tool = new ImageTool(editor);
    //   break;
    // case "Shape":
    //   tool = new ShapeTool(editor);
    //   break;
    // case "Line":
    //   tool = new LineTool(editor);
    //   break;
    // case "Eraser":
    //   tool = new EraserTool(editor);
    //   break;
    default:
      //   throw new Error(`Unknown tool name: ${toolName}`);
      return null;
  }
}

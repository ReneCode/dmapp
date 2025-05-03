import { useContext, useEffect, useRef } from "react";

import { APIContext } from "./APIContext";
import { EditorContext } from "./EditorContext";

import useWindowDimensions from "./hook/useWindowSize";

import "./Canvas.css";
import { useMiddleMousePanning } from "./hook/useMiddleMousePanning";
import { Point2d } from "wasm";

export const Canvas = () => {
  const svgRef = useRef<SVGSVGElement>(null);

  const api = useContext(APIContext);
  const editor = useContext(EditorContext);

  const { width, height } = useWindowDimensions((width, height) => {
    api.resize_canvas(width, height);
  });
  const { panningStart, panningMove, panningStop } = useMiddleMousePanning();

  useEffect(() => {
    if (svgRef.current) {
      api.init(svgRef.current.id);
    }

    const svgRoot = svgRef.current;
    if (svgRoot) {
      svgRoot.addEventListener("wheel", onWheel, { passive: false });
    }

    return () => {
      if (svgRoot) {
        svgRoot.removeEventListener("wheel", onWheel);
      }
    };
  }, []);

  const onWheel = (event: WheelEvent) => {
    event.preventDefault();
    if (event.metaKey || event.ctrlKey) {
      api.zoom_viewport(event.deltaY, event.clientX, event.clientY);
    } else {
      api.panning_viewport(event.deltaX, event.deltaY);
    }
  };

  const onMouseDown = (event: React.MouseEvent<SVGSVGElement>) => {
    event.preventDefault();
    if (event.buttons === 4) {
      panningStart(event);
    } else {
      let pt_client = new Point2d(event.clientX, event.clientY);
      let pt_canvas = api.client_to_canvas(pt_client);
      editor.dispatchEvent({
        type: "mouse_down",
        clientX: event.clientX,
        clientY: event.clientY,
        canvasX: pt_canvas.x,
        canvasY: pt_canvas.y,
      });
    }
  };

  const onMouseUp = (event: React.MouseEvent<SVGSVGElement>) => {
    event.preventDefault();
    if (event.buttons === 4) {
      panningStop();
    }
    // api?.mouse_up(event.clientX, event.clientY);
  };
  const onMouseMove = (event: React.MouseEvent<SVGSVGElement>) => {
    event.preventDefault();
    if (event.buttons === 4) {
      panningMove(event);
    }

    let pt_client = new Point2d(event.clientX, event.clientY);
    let pt_canvas = api.client_to_canvas(pt_client);

    editor.dispatchEvent({
      type: "mouse_move",
      clientX: event.clientX,
      clientY: event.clientY,
      canvasX: pt_canvas.x,
      canvasY: pt_canvas.y,
    });
    // api?.mouse_move(event.clientX, event.clientY);
  };

  return (
    <svg
      ref={svgRef}
      className="canvas"
      id="svgroot"
      xmlns="http://www.w3.org/2000/svg"
      width={`${width}`}
      height={`${height}`}
      onMouseDown={onMouseDown}
      onMouseUp={onMouseUp}
      onMouseMove={onMouseMove}
    ></svg>
  );
};

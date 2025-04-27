import { useContext, useEffect, useRef } from "react";

import { APIContext } from "./APIContext";
import useWindowDimensions from "./hook/useWindowSize";

import "./Editor.css";

export const Editor = () => {
  const svgRef = useRef<SVGSVGElement>(null);
  const api = useContext(APIContext);
  const { width, height } = useWindowDimensions((width, height) => {
    api?.resize_canvas(width, height);
  });

  useEffect(() => {
    if (svgRef.current) {
      api?.init(svgRef.current.id);
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
    if (event.metaKey || event.ctrlKey) {
      // onZoom(event);
      event.preventDefault();

      api?.zoom_viewport(event.deltaY, event.clientX, event.clientY);
    } else {
      // onPanning(event);
      console.log("onPanning");
    }
  };

  const onPanning = (event: WheelEvent) => {
    event.preventDefault();
    event.stopPropagation();
    const svg = svgRef.current;
    if (svg) {
      const viewBox = svg.viewBox.baseVal;
      const deltaX = event.deltaX * 0.1;
      const deltaY = event.deltaY * 0.1;
      viewBox.x += deltaX;
      viewBox.y += deltaY;
      svg.setAttribute(
        "viewBox",
        `${viewBox.x} ${viewBox.y} ${viewBox.width} ${viewBox.height}`
      );
    }
  };

  const onMouseDown = (event: React.MouseEvent<SVGSVGElement>) => {
    event.preventDefault();
  };

  return (
    <svg
      ref={svgRef}
      className="canvas"
      id="svgroot"
      xmlns="http://www.w3.org/2000/svg"
      width={`${width}`}
      height={`${height}`}
      // viewBox={`${viewport.current.x} ${viewport.current.y} ${viewport.current.width} ${viewport.current.height}`}
      // viewBox={`0 0 100 100`}
      // onWheel={onWheel}
      // onMouseDown={onmousedown}
    ></svg>
  );
};

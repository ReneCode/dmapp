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
  }, []);

  return (
    <svg
      ref={svgRef}
      className="canvas"
      id="svgroot"
      xmlns="http://www.w3.org/2000/svg"
      width={`${width}px`}
      height={`${height}px`}
      viewBox={`0 0 ${width} ${height}`}
    ></svg>
  );
};

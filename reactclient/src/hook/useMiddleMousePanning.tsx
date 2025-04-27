//

import { useContext, useState } from "react";
import { APIContext } from "../APIContext";

export const useMiddleMousePanning = () => {
  const api = useContext(APIContext);

  const [lastPos, setLastPos] = useState({ x: 0, y: 0 });
  const [panning, setPanning] = useState(false);

  const panningStart = (event: React.MouseEvent<SVGSVGElement, MouseEvent>) => {
    if (event.buttons === 4) {
      setLastPos({ x: event.clientX, y: event.clientY });
      setPanning(true);
    }
  };

  const panningMove = (event: React.MouseEvent<SVGSVGElement, MouseEvent>) => {
    if (!panning) {
      return;
    }
    const deltaX = lastPos.x - event.clientX;
    const deltaY = lastPos.y - event.clientY;
    setLastPos({ x: event.clientX, y: event.clientY });

    api?.panning_viewport(deltaX, deltaY);
  };

  const panningStop = () => {
    setPanning(false);
    setLastPos({ x: 0, y: 0 });
  };

  return { panningStart, panningMove, panningStop };
};

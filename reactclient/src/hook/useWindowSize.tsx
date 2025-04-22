import React, { useState, useEffect } from "react";

const useWindowDimensions = (callback: (w: number, h: number) => void) => {
  const [windowDimensions, setWindowDimensions] = useState({
    width: window.innerWidth,
    height: window.innerHeight,
  });

  useEffect(() => {
    const handleResize = () => {
      setWindowDimensions({
        width: window.innerWidth,
        height: window.innerHeight,
      });
      callback(window.innerWidth, window.innerHeight);
    };
    // Set initial dimensions
    callback(window.innerWidth, window.innerHeight);

    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  return windowDimensions;
};

export default useWindowDimensions;

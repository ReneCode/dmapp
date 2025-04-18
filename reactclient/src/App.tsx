import "./App.css";

import { APIContext } from "./APIContext";
import { useContext } from "react";
import CommandLine from "./CommandLine";

function App() {
  const api = useContext(APIContext);

  const onCommandEntered = (command: string) => {
    // console.log("Command entered:", command);

    api?.init("svgroot");

    api?.run_command(command);
    api?.render_current_page();
    // api.

    // api.render();
  };

  return (
    <>
      {/* <canvas id="canvas"> */}
      <svg
        id="svgroot"
        xmlns="http://www.w3.org/2000/svg"
        width="400px"
        height="400px"
        viewBox="0 0 400 400"
      >
        <circle
          cx="50"
          cy="50"
          r="40"
          stroke="black"
          strokeWidth="1"
          fill="red"
        />
        <text x="50" y="55" fontSize="20" textAnchor="middle" fill="white">
          Hello
        </text>
      </svg>
      {/* </canvas> */}
      <div className="commandline">
        <CommandLine onChanged={onCommandEntered} />
      </div>
    </>
  );
}

export default App;

import "./App.css";

import { APIContext } from "./APIContext";
import { useContext } from "react";
import CommandLine from "./CommandLine";
import { Statusbar } from "./Statusbar";
import { Canvas } from "./Canvas";

function App() {
  const api = useContext(APIContext);

  const onCommandEntered = (command: string) => {
    // console.log("Command entered:", command);

    api?.run_command(command);
    api?.render_current_page();
    // api.

    // api.render();
  };

  return (
    <>
      <Canvas></Canvas>

      <Statusbar></Statusbar>
      <div className="commandline">
        <CommandLine onChanged={onCommandEntered} />
      </div>
    </>
  );
}

export default App;

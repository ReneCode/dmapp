import "./App.css";

import { APIContext } from "./APIContext";
import { useContext } from "react";
import CommandLine from "./CommandLine";
import { Editor } from "./Editor";
import { Statusbar } from "./Statusbar";

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
      <Editor></Editor>

      <Statusbar></Statusbar>
      <div className="commandline">
        <CommandLine onChanged={onCommandEntered} />
      </div>
    </>
  );
}

export default App;

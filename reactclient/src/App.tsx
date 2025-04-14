import "./App.css";

import { APIContext } from "./APIContext";
import { useContext } from "react";
import CommandLine from "./CommandLine";

function App() {
  const api = useContext(APIContext);

  const onClick = () => {
    // greet();
    console.log("====>", api?.get_version());
  };

  const onCommandEntered = (command: string) => {
    // console.log("Command entered:", command);
    api?.run_command(command);
  };

  return (
    <>
      <div className="commandline">
        <CommandLine onChanged={onCommandEntered} />
      </div>

      <button onClick={onClick}>OK</button>
    </>
  );
}

export default App;

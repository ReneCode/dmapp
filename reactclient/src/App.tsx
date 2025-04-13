import "./App.css";

import { greet } from "wasm";
import { DatamodelContext } from "./datamodelContext";
import { useContext } from "react";
import CommandLine from "./CommandLine";

function App() {
  const datamodel = useContext(DatamodelContext);

  const onClick = () => {
    console.log("Button clicked");
    greet();
    console.log("=>", datamodel?.get_data());
    datamodel?.create_page("NewPage");
  };

  const onCommandEntered = (command: string) => {
    console.log("Command entered:", command);
    // Handle the command here
    // For example, you can call a function or update the state
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

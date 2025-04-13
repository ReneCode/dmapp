import { createContext, StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";

import init, { EDataModel } from "wasm";
import { DatamodelContext } from "./datamodelContext.ts";

init()
  .then(() => {
    console.log("wasm initialized");

    const dataModel = new EDataModel();
    const dataModelContext = createContext(dataModel);

    createRoot(document.getElementById("root")!).render(
      <StrictMode>
        <DatamodelContext value={dataModel}>
          <App />
        </DatamodelContext>
      </StrictMode>
    );
  })
  .catch((err) => {
    console.error("wasm initialization failed", err);
  });

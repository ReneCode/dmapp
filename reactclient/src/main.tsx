import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";

import init, { ECAPI } from "wasm";
import { APIContext } from "./APIContext.ts";

init()
  .then(() => {
    console.log("wasm initialized");

    const api = new ECAPI();

    createRoot(document.getElementById("root")!).render(
      <StrictMode>
        <APIContext value={api}>
          <App />
        </APIContext>
      </StrictMode>
    );
  })
  .catch((err) => {
    console.error("wasm initialization failed", err);
  });

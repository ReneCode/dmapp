import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";

import init, { ECAPI } from "wasm";
import { APIContext } from "./APIContext.ts";

init()
  .then(() => {
    const api = new ECAPI();
    let page_id = api?.create_page("new page") ?? "";
    console.log("created PageId:", page_id);

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

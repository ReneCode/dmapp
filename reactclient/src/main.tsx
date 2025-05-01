import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";

import init, { ECAPI } from "wasm";
import { APIContext } from "./APIContext.ts";
import { EditorContext } from "./EditorContext.ts";
import { Editor } from "./Editor.ts";

init()
  .then(() => {
    const api = new ECAPI();

    api.init("svgroot");
    let page_id = api?.create_page("new page") ?? "";
    console.log("created PageId:", page_id);

    let editor = new Editor(api);

    createRoot(document.getElementById("root")!).render(
      <StrictMode>
        <APIContext value={api}>
          <EditorContext.Provider value={editor}>
            <App />
          </EditorContext.Provider>
        </APIContext>
      </StrictMode>
    );
  })
  .catch((err) => {
    console.error("wasm initialization failed", err);
  });

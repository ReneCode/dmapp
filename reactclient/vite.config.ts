import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vite.dev/config/

// currently the webassembly is loaded via npm link
// and the path is outside the frontend-folder
// vite is complaining about this
// so we set fs.strict to false as a workaround
export default defineConfig({
  plugins: [react()],
  server: {
    fs: {
      strict: false,
    },
  },
});

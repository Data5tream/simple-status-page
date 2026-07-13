import { defineConfig } from "vite";
import preact from "@preact/preset-vite";

// https://vite.dev/config/
export default defineConfig({
  plugins: [preact()],
  define: {
    "import.meta.env.PUBLIC_API_URL": JSON.stringify(
      process.env.PUBLIC_API_URL,
    ),
  },
});

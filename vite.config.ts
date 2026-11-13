import { defineConfig } from "vite";

export default defineConfig({
  publicDir: false,
  build: {
    outDir: "assets",
    emptyOutDir: false, // Keep other files in assets like images
    rollupOptions: {
      input: {
        app: "resources/js/app.ts",
      },
      output: {
        entryFileNames: "js/[name].js",
        chunkFileNames: "js/[name].js",
        assetFileNames: (assetInfo) => {
          if (assetInfo.name && assetInfo.name.endsWith(".css")) {
            return "css/[name][extname]";
          }
          return "assets/[name]-[hash][extname]";
        },
      },
    },
  },
});

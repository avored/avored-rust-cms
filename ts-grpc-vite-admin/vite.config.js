import { defineConfig, loadEnv } from 'vite';
import react from '@vitejs/plugin-react';
import tailwindcss from '@tailwindcss/vite';
import { viteRequire } from 'vite-require';
import svgLoader from 'vite-svg-loader';


export default defineConfig(({ mode }) => {
      const env = loadEnv(mode, process.cwd(), 'VITE_'); // Load env variables with VITE_ prefix

      return {
        plugins: [
        react(),
        tailwindcss(),
        viteRequire(),
        svgLoader()
        ],
        resolve: {
            preserveSymlinks: true,
        },
        server: {
          port: parseInt(env.VITE_PORT) || 5173, // Use the VITE_PORT or default to 5173
        },
      };
});
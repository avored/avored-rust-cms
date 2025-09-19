import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import tailwindcss from '@tailwindcss/vite';
import { viteRequire } from 'vite-require';
import svgLoader from 'vite-svg-loader';
// https://vite.dev/config/
export default defineConfig({
    plugins: [
        react(),
        tailwindcss(),
        viteRequire(),
        svgLoader()
    ],
    resolve: {
        preserveSymlinks: true,
    }
});

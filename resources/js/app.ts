import "../css/app.css";

import "./init";
import { setupPage } from "./misc/SetupPage";
import { entityIndexPage } from "./entity/EntityIndexPage";
import { entityCreatePage } from "./entity/EntityCreatePage";
import { entityEditPage } from "./entity/EntityEditPage";

declare global {
  interface Window {
    setupPage: typeof setupPage;
    entityIndexPage: typeof entityIndexPage;
    entityCreatePage: typeof entityCreatePage;
    entityEditPage: typeof entityEditPage;
  }
}

window.setupPage = setupPage;
window.entityIndexPage = entityIndexPage;
window.entityCreatePage = entityCreatePage;
window.entityEditPage = entityEditPage;

if (window.Alpine) {
  window.Alpine.data("setupPage", setupPage);
  window.Alpine.data("entityIndexPage", entityIndexPage);
  window.Alpine.data("entityCreatePage", entityCreatePage);
  window.Alpine.data("entityEditPage", entityEditPage);
}

const initApp = () => {
  if (window.Alpine) {
    window.Alpine.start();
  }
  if (window.feather) {
    window.feather.replace();
  }
};

if ((window as any).leptos_hydrated) {
  initApp();
} else {
  window.addEventListener("leptos-hydrated", () => {
    initApp();
  }, { once: true });
}

// Fallback when DOM content is loaded
document.addEventListener("DOMContentLoaded", () => {
  if (window.feather) {
    window.feather.replace();
  }
});


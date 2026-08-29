import "../css/app.css";

import "./init";

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


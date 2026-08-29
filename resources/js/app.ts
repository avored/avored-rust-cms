import "../css/app.css";

import "./init";

const startAlpine = () => {
  if (window.Alpine) {
    window.Alpine.start();
  }
};

if ((window as any).leptos_hydrated) {
  startAlpine();
} else {
  window.addEventListener("leptos-hydrated", () => {
    startAlpine();
  }, { once: true });
}

// Rule: Alpine must never be attached to Leptos-managed DOM.
// Use Alpine only on standalone DOM nodes or widgets that are not rendered by Leptos.

import "../css/app.css";

import "./init";


console.log("Alpine components registered, waiting for Leptos hydration...");

const startAlpine = () => {
  if (window.Alpine) {
    console.log("Starting Alpine.js");
    window.Alpine.start();
  } else {
    console.error("Alpine not found on window");
  }
};

// Check if Leptos already hydrated (race condition: hydration script may finish first)
if ((window as any).leptos_hydrated) {
  console.log("Leptos already hydrated, starting Alpine immediately");
  startAlpine();
} else {
  console.log("Waiting for leptos-hydrated event");
  window.addEventListener("leptos-hydrated", () => {
    console.log("Received leptos-hydrated event");
    startAlpine();
  });
}

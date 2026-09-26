import "../css/app.css";

import "./init";
import { setupPage } from "./misc/SetupPage";
import { entityIndexPage } from "./entity/EntityIndexPage";
import { entityCreatePage } from "./entity/EntityCreatePage";
import { entityEditPage } from "./entity/EntityEditPage";
import { collectionCreatePage } from "./collection/CollectionCreatePage";
import { collectionEditPage } from "./collection/CollectionEditPage";
import { collectionIndexPage } from "./collection/CollectionIndexPage";
import { emailTemplateCreatePage } from "./email/EmailTemplateCreatePage";
import { emailTemplateEditPage } from "./email/EmailTemplateEditPage";
import { emailTemplateIndexPage } from "./email/EmailTemplateIndexPage";

declare global {
  interface Window {
    setupPage: typeof setupPage;
    entityIndexPage: typeof entityIndexPage;
    entityCreatePage: typeof entityCreatePage;
    entityEditPage: typeof entityEditPage;
    collectionCreatePage: typeof collectionCreatePage;
    collectionEditPage: typeof collectionEditPage;
    collectionIndexPage: typeof collectionIndexPage;
    emailTemplateCreatePage: typeof emailTemplateCreatePage;
    emailTemplateEditPage: typeof emailTemplateEditPage;
    emailTemplateIndexPage: typeof emailTemplateIndexPage;
  }
}

window.setupPage = setupPage;
window.entityIndexPage = entityIndexPage;
window.entityCreatePage = entityCreatePage;
window.entityEditPage = entityEditPage;
window.collectionCreatePage = collectionCreatePage;
window.collectionEditPage = collectionEditPage;
window.collectionIndexPage = collectionIndexPage;
window.emailTemplateCreatePage = emailTemplateCreatePage;
window.emailTemplateEditPage = emailTemplateEditPage;
window.emailTemplateIndexPage = emailTemplateIndexPage;

if (window.Alpine) {
  window.Alpine.data("setupPage", setupPage);
  window.Alpine.data("entityIndexPage", entityIndexPage);
  window.Alpine.data("entityCreatePage", entityCreatePage);
  window.Alpine.data("entityEditPage", entityEditPage);
  window.Alpine.data("collectionIndexPage", collectionIndexPage);
  window.Alpine.data("collectionCreatePage", collectionCreatePage);
  window.Alpine.data("collectionEditPage", collectionEditPage);
  window.Alpine.data("emailTemplateCreatePage", emailTemplateCreatePage);
  window.Alpine.data("emailTemplateEditPage", emailTemplateEditPage);
  window.Alpine.data("emailTemplateIndexPage", emailTemplateIndexPage);
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

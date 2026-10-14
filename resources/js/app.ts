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
import { workflowFormPage } from "./workflow/WorkflowFormPage";
import { workflowIndexPage } from "./workflow/WorkflowIndexPage";
import { apiKeyIndexPage } from "./api_key/ApiKeyIndexPage";
import { apiKeyCreatePage } from "./api_key/ApiKeyCreatePage";
import { apiKeyEditPage } from "./api_key/ApiKeyEditPage";
import { appLayout } from "./layouts/AppLayout";

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
    workflowFormPage: typeof workflowFormPage;
    workflowIndexPage: typeof workflowIndexPage;
    apiKeyIndexPage: typeof apiKeyIndexPage;
    apiKeyCreatePage: typeof apiKeyCreatePage;
    apiKeyEditPage: typeof apiKeyEditPage;
    appLayout: typeof appLayout;
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
window.workflowFormPage = workflowFormPage;
window.workflowIndexPage = workflowIndexPage;
window.apiKeyIndexPage = apiKeyIndexPage;
window.apiKeyCreatePage = apiKeyCreatePage;
window.apiKeyEditPage = apiKeyEditPage;
window.appLayout = appLayout;

if (window.Alpine) {
  window.Alpine.data("appLayout", appLayout);
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
  window.Alpine.data("workflowFormPage", workflowFormPage);
  window.Alpine.data("workflowIndexPage", workflowIndexPage);
  window.Alpine.data("apiKeyIndexPage", apiKeyIndexPage);
  window.Alpine.data("apiKeyCreatePage", apiKeyCreatePage);
  window.Alpine.data("apiKeyEditPage", apiKeyEditPage);
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

// import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App.tsx";
import "./index.css";
import reportWebVitals from "./reportWebVitals";
import { QueryClientProvider, QueryClient } from "@tanstack/react-query";
import en_locales from "./locales/en.json";
import fr_locales from "./locales/fr.json";
import i18next from "i18next";
import { I18nextProvider } from "react-i18next";

const queryClient = new QueryClient();
const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
);

const currentLocale: string = localStorage.getItem("LOCALE") ?? "en";

i18next.init({
  interpolation: { escapeValue: false },
  lng: currentLocale,
  fallbackLng: currentLocale,
  resources: {
    en: {
      global: en_locales,
    },
    fr: {
      global: fr_locales,
    },
  },
});

root.render(
  <I18nextProvider i18n={i18next}>
    <QueryClientProvider client={queryClient}>
      <App />
    </QueryClientProvider>
  </I18nextProvider>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();

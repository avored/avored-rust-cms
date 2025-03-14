import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';
import reportWebVitals from './reportWebVitals';
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import i18next from "i18next";
import en_locales from "./locales/en.json";
import fr_locales from "./locales/fr.json";
import {I18nextProvider} from "react-i18next";

const queryClient = new QueryClient()


const currentLocale: string = localStorage.getItem("LOCALE") ?? "en";
i18next.init({
    interpolation: {escapeValue: false},
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

const root = ReactDOM.createRoot(
    document.getElementById('root') as HTMLElement
);
root.render(
    <QueryClientProvider client={queryClient}>
        <I18nextProvider i18n={i18next}>
            <App/>
        </I18nextProvider>
    </QueryClientProvider>,
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();

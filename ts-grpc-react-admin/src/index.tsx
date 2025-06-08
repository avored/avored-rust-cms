import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';
import reportWebVitals from './reportWebVitals';
import {MutationCache, QueryCache, QueryClient, QueryClientProvider} from "@tanstack/react-query";
import i18next from "i18next";
import en_locales from "./locales/en.json";
import fr_locales from "./locales/fr.json";
import {I18nextProvider} from "react-i18next";
import {GrpcErrorCode, GrpcErrorType} from "./types/common/ErrorType";
import toast, {resolveValue, Toaster} from "react-hot-toast";
import {ExclamationTriangleIcon} from "@heroicons/react/24/solid";

const queryClient = new QueryClient({

    queryCache : new QueryCache({
        onError: (error: Error, query) => {
            handleError(error);
        }
    }),
    mutationCache: new MutationCache({
        onError: (error: Error, mutation) => {
            handleError(error);
        }
    })
})

const handleError = (error: Error) => {
    const grpcError = error as unknown as GrpcErrorType;
    switch (grpcError.code) {
        case GrpcErrorCode.PermissionDenied:
            toast.error(grpcError.message)

            break;
        case GrpcErrorCode.Unknown:
            toast.error(grpcError.message)

            break;
    }
}


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
            <Toaster position="bottom-right" toastOptions={{
                duration: 3000,
                removeDelay: 0
            }}>
                {(t) => (
                    <div
                        className={`max-w-md w-full bg-white shadow-lg rounded-lg pointer-events-auto flex ring-1 ring-black`}
                    >
                        <div className="flex-1 w-0 p-4">
                            <div className="flex items-center justify-center">
                                <div className="flex-shrink-0 pt-0.5">
                                    <ExclamationTriangleIcon className=" h-6 w-6 text-primary-400" aria-hidden="true"
                                    />
                                </div>
                                <div className="ml-3 flex-1">
                                    <p className="mt-1 text-sm text-gray-500">
                                        {resolveValue(t.message, t)}
                                    </p>
                                </div>
                            </div>
                        </div>
                        <div className="flex border-l border-gray-200">
                            <button
                                type="button"
                                onClick={(e) =>  {
                                    e.preventDefault();
                                    toast.dismiss(t.id)
                                }}
                                className="w-full rounded-none rounded-r-lg p-4 flex items-center justify-center text-sm text-primary-600 hover:text-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
                            >
                                Close
                            </button>
                        </div>
                    </div>

                )}
            </Toaster>
            <App/>
        </I18nextProvider>
    </QueryClientProvider>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();

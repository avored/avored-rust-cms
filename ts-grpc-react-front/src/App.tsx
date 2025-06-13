import React from 'react';
import {BrowserRouter, Route, Routes} from "react-router-dom";
import AppLayout from "./layout/AppLayout";
import HomePage from "./pages/home/HomePage";
import {PrivacyPage} from "./pages/PrivacyPage";
import { DocsPage } from './pages/docs/DocsPage';

function App() {
    return (
        <BrowserRouter>
            <Routes>
                <Route element={<AppLayout />}>
                    <Route path="/" element={<HomePage />} />
                    <Route path="/docs" element={<DocsPage />} />

                    <Route path="/privacy" element={<PrivacyPage />} />
                </Route>
            </Routes>
        </BrowserRouter>
    );
}

export default App;

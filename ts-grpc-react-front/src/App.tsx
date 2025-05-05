import React from 'react';
import {BrowserRouter, Route, Routes} from "react-router-dom";
import AppLayout from "./layout/AppLayout";
import HomePage from "./pages/home/HomePage";

function App() {
    return (
        <BrowserRouter>
            <Routes>
                <Route element={<AppLayout />}>
                    <Route path="/" element={<HomePage />} />
                </Route>
            </Routes>
        </BrowserRouter>
    );
}

export default App;

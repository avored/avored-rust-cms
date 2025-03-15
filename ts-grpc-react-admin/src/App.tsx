import React from 'react';
import {BrowserRouter, Route, Routes} from "react-router-dom";
import {HomePage} from "./pages/HomePage";
import {SetupPage} from "./pages/misc/SetupPage";
import {LoginPage} from "./pages/auth/LoginPage";

function App() {

    return (
        <BrowserRouter>
            <Routes>
                <Route path="/" element={<HomePage/>}/>
                <Route path="/setup" element={<SetupPage/>}/>
                <Route path="/admin/login" element={<LoginPage />} />
            </Routes>
        </BrowserRouter>
    );
}

export default App;

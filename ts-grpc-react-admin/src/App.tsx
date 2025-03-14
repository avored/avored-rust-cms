import React from 'react';
import {BrowserRouter, Route, Routes} from "react-router-dom";
import {HomePage} from "./pages/HomePage";
import {SetupPage} from "./pages/misc/SetupPage";

function App() {

    return (
        <BrowserRouter>
            <Routes>
                <Route path="/" element={<HomePage/>}/>
                <Route path="/setup" element={<SetupPage/>}/>
            </Routes>
        </BrowserRouter>

    );
}

export default App;

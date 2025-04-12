import React from 'react';
import {BrowserRouter, Route, Routes} from "react-router-dom";
import {HomePage} from "./pages/HomePage";
import {SetupPage} from "./pages/misc/SetupPage";
import {LoginPage} from "./pages/auth/LoginPage";
import {DashboardPage} from "./pages/DashboardPage";
import AppLayout from "./layouts/AppLayout";
import {AdminUserTablePage} from "./pages/admin_user/AdminUserTablePage";
import {AdminUserCreatePage} from "./pages/admin_user/AdminUserCreatePage";

function App() {

    return (
        <BrowserRouter>
            <Routes>
                <Route path="/" element={<HomePage/>}/>
                <Route path="/setup" element={<SetupPage/>}/>
                <Route path="/admin/login" element={<LoginPage />} />


                <Route element={<AppLayout />}>
                    <Route path="/admin/dashboard" element={<DashboardPage />} />
                    <Route path="/admin/admin-user" element={<AdminUserTablePage />} />
                    <Route path="/admin/admin-user-create" element={<AdminUserCreatePage />} />
                </Route>
            </Routes>
        </BrowserRouter>
    );
}

export default App;

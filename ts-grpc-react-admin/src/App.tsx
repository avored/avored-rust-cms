import React from 'react';
import {BrowserRouter, Route, Routes} from "react-router-dom";
import {HomePage} from "./pages/HomePage";
import {SetupPage} from "./pages/misc/SetupPage";
import {LoginPage} from "./pages/auth/LoginPage";
import {DashboardPage} from "./pages/DashboardPage";
import AppLayout from "./layouts/AppLayout";
import {AdminUserTablePage} from "./pages/admin_user/AdminUserTablePage";
import {AdminUserCreatePage} from "./pages/admin_user/AdminUserCreatePage";
import {AdminUserEditPage} from "./pages/admin_user/AdminUserEditPage";
import {RoleTablePage} from "./pages/admin_user/RoleTablePage";
import {RoleCreatePage} from "./pages/admin_user/RoleCreatePage";
import {RoleEditPage} from "./pages/admin_user/RoleEditPage";
import {ContentTablePage} from "./pages/content/ContentTablePage";

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
                    <Route path="/admin/admin-user-edit/:admin_user_id"   element={<AdminUserEditPage />} />

                    <Route path="/admin/role" element={<RoleTablePage />} />
                    <Route path="/admin/role-create" element={<RoleCreatePage />} />
                    <Route path="/admin/role-edit/:role_id" element={<RoleEditPage />} />

                    <Route path="/admin/content" element={<ContentTablePage />} />
                </Route>
            </Routes>
        </BrowserRouter>
    );
}

export default App;

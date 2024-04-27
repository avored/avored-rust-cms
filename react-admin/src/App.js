import './App.css';
import {BrowserRouter, Route, Routes} from 'react-router-dom';
import Dashboard from './pages/Dashboard'
import Login from './pages/auth/Login'
import Home from "./pages/Home";
import AppLayout from "./layouts/AppLayout";
import PageTable from "./pages/page/PageTable";
import PageCreate from "./pages/page/PageCreate";
import PageEdit from "./pages/page/PageEdit";
import AdminUserTable from "./pages/admin-user/AdminUserTable";
import AdminUserCreate from "./pages/admin-user/AdminUserCreate";
import AdminUserEdit from "./pages/admin-user/AdminUserEdit";
import RoleTable from "./pages/role/RoleTable";
import RoleCreate from "./pages/role/RoleCreate";
import RoleEdit from "./pages/role/RoleEdit";
import AssetTable from "./pages/asset/AssetTable";
import ComponentTable from "./pages/component/ComponentTable";
import ComponentCreate from "./pages/component/ComponentCreate";
import ComponentEdit from "./pages/component/ComponentEdit";
import Setup from "./pages/setup/setup";
import Logout from "./pages/auth/Logout";
function App() {
    return (
        <BrowserRouter>
            <Routes>
                {/*<Route path="/" element={<Navigate to="/admin" replace />} />*/}
                <Route path="/" element={<Home />} />

                <Route path="/admin/login"   element={<Login />} />
                <Route path="/setup"   element={<Setup />} />

                <Route element={<AppLayout />}>
                    <Route path="/admin"   element={<Dashboard />} />
                    <Route path="/admin/logout"   element={<Logout />} />
                    <Route path="/admin/asset"   element={<AssetTable />} />
                    <Route path="/admin/component"   element={<ComponentTable />} />
                    <Route path="/admin/component-create"   element={<ComponentCreate />} />
                    <Route path="/admin/component-edit/:component_id"   element={<ComponentEdit />} />
                    <Route path="/admin/role"   element={<RoleTable />} />
                    <Route path="/admin/role-create"   element={<RoleCreate />} />
                    <Route path="/admin/role-edit/:role_id"   element={<RoleEdit />} />
                    <Route path="/admin/page"   element={<PageTable />} />
                    <Route path="/admin/page-create"   element={<PageCreate />} />
                    <Route path="/admin/page-edit/:page_id"   element={<PageEdit />} />
                    <Route path="/admin/admin-user"   element={<AdminUserTable />} />
                    <Route path="/admin/admin-user-create"   element={<AdminUserCreate />} />
                    <Route path="/admin/admin-user-edit/:admin_user_id"   element={<AdminUserEdit />} />
                </Route>

            </Routes>
        </BrowserRouter>
    );
}

export default App;

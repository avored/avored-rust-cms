import './App.css';
import {BrowserRouter, Route, Routes} from 'react-router-dom';
import Dashboard from './pages/Dashboard'
import Login from './pages/auth/Login'
import Home from "./pages/Home";
import AppLayout from "./layouts/AppLayout";
import AdminUserTable from "./pages/admin-user/AdminUserTable";
import AdminUserCreate from "./pages/admin-user/AdminUserCreate";
import AdminUserEdit from "./pages/admin-user/AdminUserEdit";
import RoleTable from "./pages/role/RoleTable";
import RoleCreate from "./pages/role/RoleCreate";
import RoleEdit from "./pages/role/RoleEdit";
import AssetTable from "./pages/asset/AssetTable";
import Setup from "./pages/setup/setup";
import Logout from "./pages/auth/Logout";
import ForgotPassword from "./pages/auth/ForgotPassword";
import ResetPassword from "./pages/auth/ResetPassword";
import AvoRedApiTesting from "./pages/setting/AvoRedApiTesting";
import SettingPage from "./pages/setting/SettingPage";
import NotFoundPage from "./pages/NotFoundPage";
import ChangePassword from './pages/admin-user/ChangePassword';
import {CollectionTable} from "./pages/collection/CollectionTable";
import {CollectionEdit} from "./pages/collection/CollectionEdit";
import {CollectionCreate} from "./pages/collection/CollectionCreate";
import {ContentTable} from "./pages/content/ContentTable";
import {ContentCreate} from "./pages/content/ContentCreate";
import {ContentEdit} from "./pages/content/ContentEdit";

function App() {
    return (
        <BrowserRouter>
            <Routes>
                {/*<Route path="/" element={<Navigate to="/admin" replace />} />*/}
                <Route path="/" element={<Home />} />

                <Route path="/admin/login"   element={<Login />} />
                <Route path="/admin/forgot-password"   element={<ForgotPassword />} />
                <Route path="/admin/reset-password/:token"   element={<ResetPassword />} />
                <Route path="/setup"   element={<Setup />} />

                <Route element={<AppLayout />}>
                    <Route path="/admin"   element={<Dashboard />} />
                    <Route path="/admin/logout"   element={<Logout />} />
                    <Route path="/admin/asset/:asset_id?"   element={<AssetTable />} />
                    <Route path="/admin/role"   element={<RoleTable />} />
                    <Route path="/admin/role-create"   element={<RoleCreate />} />
                    <Route path="/admin/role-edit/:role_id"   element={<RoleEdit />} />

                    <Route path="/admin/content"   element={<ContentTable />} />
                    <Route path="/admin/content-create"   element={<ContentCreate />} />
                    <Route path="/admin/content-edit/:content_id"   element={<ContentEdit />} />

                    <Route path="/admin/collections"   element={<CollectionTable />} />
                    <Route path="/admin/collection-create"   element={<CollectionCreate />} />
                    <Route path="/admin/collection-edit/:collection_id"   element={<CollectionEdit />} />
                    <Route path="/admin/admin-user"   element={<AdminUserTable />} />
                    <Route path="/admin/admin-user-create"   element={<AdminUserCreate />} />
                    <Route path="/admin/admin-user-edit/:admin_user_id"   element={<AdminUserEdit />} />
                    <Route path="/admin/admin-user-change-password" element={<ChangePassword/>}/>
                    <Route path="/admin/api-test"   element={<AvoRedApiTesting />} />
                    <Route path="/admin/setting"   element={<SettingPage />} />
                </Route>

                <Route path="*"   element={<NotFoundPage />} />
            </Routes>
        </BrowserRouter>
    );
}

export default App;

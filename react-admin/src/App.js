import './App.css';
import {BrowserRouter, Navigate, Route, Routes} from 'react-router-dom';
import Dashboard from './pages/Dashboard'
import Login from './pages/auth/Login'
import Home from "./pages/Home";
import AppLayout from "./layouts/AppLayout";
import PageTable from "./pages/page/PageTable";
import PageCreate from "./pages/page/PageCreate";
import PageEdit from "./pages/page/PageEdit";
import AdminUserTable from "./pages/admin-user/AdminUserTable";
import AdminUserCreate from "./pages/admin-user/AdminUserCreate";

function App() {
    return (
        <BrowserRouter>
            <Routes>
                {/*<Route path="/" element={<Navigate to="/admin" replace />} />*/}
                <Route path="/" element={<Home />} />

                <Route path="/admin/login"   element={<Login />} />

                <Route element={<AppLayout />}>
                    <Route path="/admin"   element={<Dashboard />} />
                    <Route path="/admin/page"   element={<PageTable />} />
                    <Route path="/admin/page-create"   element={<PageCreate />} />
                    <Route path="/admin/page-edit/:page_id"   element={<PageEdit />} />
                    <Route path="/admin/admin-user"   element={<AdminUserTable />} />
                    <Route path="/admin/admin-user-create"   element={<AdminUserCreate />} />
                </Route>

            </Routes>
        </BrowserRouter>
    );
}

export default App;

import './App.css';
import {BrowserRouter, Navigate, Route, Routes} from 'react-router-dom';
import Dashboard from './pages/Dashboard'
import Login from './pages/auth/Login'
import Home from "./pages/Home";
import AppLayout from "./layouts/AppLayout";
import PageTable from "./pages/page/PageTable";
import PageCreate from "./pages/page/PageCreate";

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
                    <Route path="/admin/page/create"   element={<PageCreate />} />
                </Route>

            </Routes>
        </BrowserRouter>
    );
}

export default App;

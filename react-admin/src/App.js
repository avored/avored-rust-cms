import './App.css';
import {BrowserRouter, Navigate, Route, Routes} from 'react-router-dom';
import Dashboard from './pages/Dashboard'
import Login from './pages/auth/Login'
import Home from "./pages/Home";

function App() {
    return (
        <BrowserRouter>
            <Routes>
                {/*<Route path="/" element={<Navigate to="/admin" replace />} />*/}
                <Route path="/" element={<Home />} />
                <Route path="/admin"   element={<Dashboard />} />
                <Route path="/admin/login"   element={<Login />} />
            </Routes>
        </BrowserRouter>
    );
}

export default App;

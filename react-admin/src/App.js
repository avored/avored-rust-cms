import './App.css';
import {BrowserRouter, Route, Routes} from 'react-router-dom';
import Dashboard from './pages/Dashboard'
import Login from './pages/auth/Login'

function App() {
    return (
        <BrowserRouter>
            <Routes>
                <Route path="/"   element={<Dashboard />} />
                <Route path="/admin/login"   element={<Login />} />
            </Routes>
        </BrowserRouter>
    );
}

export default App;

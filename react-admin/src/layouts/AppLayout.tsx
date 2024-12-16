import AppHeader from "./partials/AppHeader";
import AppSidebar from "./partials/AppSidebar";
import {useEffect, useState} from "react";
import {useNavigate} from "react-router-dom";
import {isEmpty} from "lodash";
import {ThemeContext} from "../context/ThemeContext";


function AppLayout() {

    const redirect = useNavigate()
    const [isSidebarOpen, setIsSidebarOpen] = useState(false);

    const toggleSidebar = () => {
        setIsSidebarOpen((prev) => !prev)
    }
    const value = {
        isSidebarOpen,
        toggleSidebar,
    }

    useEffect(() => {
        // @todo permission check here
        const token = localStorage.getItem("AUTH_TOKEN")
        if (isEmpty(token)) {
            return redirect("/admin/login")
        }
    })

    return (
        <ThemeContext.Provider value={value}>
            <div
                className="min-h-screen">
                <AppHeader/>
                <AppSidebar/>
            </div>
        </ThemeContext.Provider>

    );
}

export default AppLayout;

import AppHeader from "./partials/AppHeader";
import AppSidebar from "./partials/AppSidebar";
import { useState } from "react";
import { AvoredAdminContext } from "../context/AvoredAdminContext";
import { LoggedInUserRequest } from "../grpc_generated/general_pb";
import { UseLoggedInUserHook } from "../hooks/general/UseLoggedInUserHook";
import { AdminUserType } from "../types/admin_user/AdminUserType";


function AppLayout() {
    const [isSidebarOpen, setIsSidebarOpen] = useState(false);
    const request = new LoggedInUserRequest();
    const auth_user_model = UseLoggedInUserHook(request);
    const adminUser: AdminUserType = auth_user_model?.data?.data as unknown as AdminUserType;

    const toggleSidebar = () => {
        setIsSidebarOpen((prev) => !prev)
    }
    const value = {
        isSidebarOpen,
        toggleSidebar,
        adminUser
    }

    return (
        <AvoredAdminContext.Provider value={value}>
            <div
                className="min-h-screen">
                <AppHeader />
                <AppSidebar />
            </div>
        </AvoredAdminContext.Provider>

    );
}

export default AppLayout;

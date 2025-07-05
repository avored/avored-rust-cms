import {createContext} from "react";
import {AdminUserType} from '../types/admin_user/AdminUserType'

export type AvoredAdminContextType = {
    isSidebarOpen: boolean,
    toggleSidebar: () => void,
    adminUser: AdminUserType
}
export const AvoredAdminContext = createContext<AvoredAdminContextType>({
    isSidebarOpen: false,
    toggleSidebar: () => {},
    adminUser: {} as AdminUserType
});


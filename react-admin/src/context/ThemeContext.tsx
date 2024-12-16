import {createContext} from "react";

export type ThemeContextType = {
    isSidebarOpen: boolean,
    toggleSidebar: () => void,
}
export const ThemeContext = createContext<ThemeContextType>({
    isSidebarOpen: false,
    toggleSidebar: () => {},
});


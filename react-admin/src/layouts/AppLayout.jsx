import AppHeader from "./partials/AppHeader";
import AppSidebar from "./partials/AppSidebar";

function AppLayout() {
    return (
        <div
            className="min-h-screen">
            <AppHeader />
            <AppSidebar />
        </div>
    );
}

export default AppLayout;

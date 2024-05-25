import AppHeader from "./partials/AppHeader";
import AppSidebar from "./partials/AppSidebar";
import {useEffect} from "react";
import {useNavigate} from "react-router-dom";
import {isEmpty} from "lodash";

function AppLayout() {
    const redirect = useNavigate()

    useEffect(() => {
        // @todo permission check here
        const token = localStorage.getItem("AUTH_TOKEN")
        if (isEmpty(token)) {
            return redirect("/admin/login")
        }
    })

    return (
        <div
            className="min-h-screen">
            <AppHeader />
            <AppSidebar />
        </div>
    );
}

export default AppLayout;

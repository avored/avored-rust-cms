import { Outlet } from "react-router-dom";
import AppHeader from "./partials/AppHeader";
import AppFooter from "./partials/AppFooter";

const AppLayout = () => {
  return (
    <div className="bg-white max-w-screen-xl mx-auto">
      <AppHeader />
      <Outlet />
      <AppFooter />
    </div>
  );
};

export default AppLayout;

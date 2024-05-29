import AppHeader from "./partials/AppHeader";
import AppSidebar from "./partials/AppSidebar";
import { useEffect, FC } from "react";
import { useNavigate } from "react-router-dom";
import { isEmpty } from "lodash";

const AppLayout: FC = () => {
  const redirect = useNavigate();

  useEffect(() => {
    // @todo permission check here
    const token = localStorage.getItem("AUTH_TOKEN");
    if (isEmpty(token)) {
      return redirect("/admin/login");
    }
  }, [redirect]); // Added dependency array to prevent unnecessary re-renders

  return (
    <div className="min-h-screen">
      <AppHeader />
      <AppSidebar />
    </div>
  );
};

export default AppLayout;

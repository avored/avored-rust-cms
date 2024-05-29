// import logo from "../../assets/logo_only.svg";
// import { Menu } from "@headlessui/react";
import _ from "lodash";
import { useNavigate } from "react-router-dom"; // Removed 'Link' import
import { useEffect } from "react";
// import { useTranslation } from "react-i18next";

interface AdminUser {
  id?: string;
}

function AppHeader() {
  const adminUser: AdminUser = JSON.parse(
    localStorage.getItem("AUTH_ADMIN_USER") || "{}"
  );
  const navigate = useNavigate();

  useEffect(() => {
    if (!_.get(adminUser, "id")) {
      localStorage.clear();
      navigate("/admin/login");
    }
  });

  // Return null or some JSX element here
  return null;
}
export default AppHeader;

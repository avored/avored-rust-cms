import { useEffect } from "react";
import { useNavigate } from "react-router-dom";

function Logout() {
  const redirect = useNavigate();
  useEffect(() => {
    localStorage.clear();
    redirect("/admin/login");
  }, [redirect]); // Added "redirect" to the dependency array
}

export default Logout;

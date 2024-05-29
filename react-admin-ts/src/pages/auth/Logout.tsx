import { useEffect } from "react";
import { useNavigate } from "react-router-dom";

const Logout: React.FC = () => {
  const redirect = useNavigate();
  useEffect(() => {
    localStorage.clear();
    redirect("/admin/login");
  }, [redirect]); // Added "redirect" to the dependency array

  return null; // Added a return statement to return a valid JSX element
};

export default Logout;

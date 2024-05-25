import {useEffect} from "react"
import { useNavigate } from "react-router-dom"

function Logout() {
  const redirect = useNavigate();
  useEffect(() => {
    localStorage.clear()
    redirect("/admin/login")
  }, []);

}

export default Logout;

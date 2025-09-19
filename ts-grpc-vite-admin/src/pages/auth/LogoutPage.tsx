import {useNavigate} from "react-router-dom";
import {useEffect} from "react";

export const LogoutPage = () => {
    const redirect = useNavigate();
    useEffect(() => {
        localStorage.clear();
        redirect("/admin/login");
    }, [redirect]); // Added "redirect" to the dependency array


    return(
        <></>
    )
}

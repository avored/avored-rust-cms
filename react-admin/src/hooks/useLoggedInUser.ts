import LoggedInUserType from "../types/admin-user/LoggedInUserType";

export const useLoggedInUser = () : LoggedInUserType => {
    return JSON.parse(localStorage.getItem("AUTH_ADMIN_USER") ?? '{}');
}

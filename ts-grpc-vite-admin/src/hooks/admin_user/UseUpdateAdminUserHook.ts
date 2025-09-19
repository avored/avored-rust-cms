import {useMutation} from "@tanstack/react-query";
import {useNavigate} from "react-router-dom";
import {AdminUserClient} from "grpc-avored/Admin_userServiceClientPb";
import {UpdateAdminUserRequest} from "grpc-avored/admin_user_pb";

export const UseUpdateAdminUserHook = () => {
    const backend_url: string = import.meta.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new AdminUserClient(backend_url);
    const redirect = useNavigate();

    return useMutation({
        mutationFn: (request: UpdateAdminUserRequest) => {
            return client.updateAdminUser(request, {
                'Authorization': `Bearer ${localStorage.getItem('token')}`
            })
        },
        onSuccess: (res) => {
            if (res.getStatus()) {
                // localStorage.setItem("token", token);
                redirect("/admin/admin-user");
            }
        }
    })
}
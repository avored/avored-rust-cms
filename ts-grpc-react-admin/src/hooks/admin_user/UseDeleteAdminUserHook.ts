import {useMutation} from "@tanstack/react-query";
import {useNavigate} from "react-router-dom";
import {AdminUserClient} from "../../grpc_generated/Admin_userServiceClientPb";
import {DeleteAdminUserRequest, DeleteRoleRequest} from "../../grpc_generated/admin_user_pb";

export const UseDeleteAdminUserHook = () => {
    const backend_url: string = process.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new AdminUserClient(backend_url);
    const redirect = useNavigate();

    return useMutation({
        mutationFn: (request: DeleteAdminUserRequest) => {
            return client.deleteAdminUser(request, {
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
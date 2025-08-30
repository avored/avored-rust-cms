import {useMutation} from "@tanstack/react-query";
import {useNavigate} from "react-router-dom";
import {AdminUserClient} from "../../grpc_generated/Admin_userServiceClientPb";
import {ChangeAdminUserPasswordRequest} from "../../grpc_generated/admin_user_pb";

export const UseChangePasswordHook = () => {
    const backend_url: string = process.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new AdminUserClient(backend_url);
    const redirect = useNavigate();

    return useMutation({
        mutationFn: (request: ChangeAdminUserPasswordRequest) => {
            /// todo change this to change password
            return client.changeAdminUserPassword(request, {
                'Authorization': `Bearer ${localStorage.getItem('token')}`
            })
        },
        onSuccess: (res) => {
            console.log("res", res)
            if (res.getStatus()) {
                localStorage.removeItem("token");
                redirect("/admin/login");
            }
        }
    })
}
import {AuthClient} from "../../grpc_generated/AuthServiceClientPb";
import {useNavigate} from "react-router-dom";
import {useMutation} from "@tanstack/react-query";
import {ResetPasswordRequest} from "../../grpc_generated/auth_pb";

export const UseResetPasswordHook = () => {
    const backend_url: string = process.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new AuthClient(backend_url);
    const redirect = useNavigate();

    return useMutation({
        mutationFn: (request: ResetPasswordRequest) => {
            return client.resetPassword(request)
        },
        onSuccess: (res) => {
            // console.log(res, "forgot password response");
            if (res.getStatus()) {
                redirect("/admin/login");
            }
        },
        onError: (err) => {
            console.log(err, "forgot password error");
        }
    })
}
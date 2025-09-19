import {useMutation} from "@tanstack/react-query";
import {useNavigate} from "react-router-dom";
import {AuthClient} from "grpc-avored/AuthServiceClientPb";
import {ForgotPasswordRequest} from "grpc-avored/auth_pb";

export const UseForgotPasswordHook = () => {
    const backend_url: string = import.meta.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new AuthClient(backend_url);
    const redirect = useNavigate();

    return useMutation({
        mutationFn: (request: ForgotPasswordRequest) => {
            return client.forgotPassword(request)
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
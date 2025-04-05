import {useMutation} from "@tanstack/react-query";
import {useNavigate} from "react-router-dom";
import {AuthClient} from "../../grpc_generated/AuthServiceClientPb";
import {LoginRequest} from "../../grpc_generated/auth_pb";

export const UseLoginHook = () => {
    const backend_url: string = process.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new AuthClient(backend_url);
    const redirect = useNavigate();

    return useMutation({
        mutationFn: (request: LoginRequest) => {
            return client.login(request)
        },
        onSuccess: (res) => {
            if (res.getStatus()) {
                let token = res.getData();
                localStorage.setItem("token", token);
                redirect("/admin/dashboard");
            }
        }
    })
}
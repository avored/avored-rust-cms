import {useMutation} from "@tanstack/react-query";
import {useNavigate} from "react-router-dom";
import {AuthClient} from "grpc-avored/AuthServiceClientPb";
import {LoginRequest} from "grpc-avored/auth_pb";

export const UseLoginHook = () => {
    const backend_url: string = import.meta.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
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

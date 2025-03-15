import {MiscClient} from "../../grpc_generated/MiscServiceClientPb";
import {useMutation} from "@tanstack/react-query";
import {SetupRequest} from "../../grpc_generated/misc_pb";
import {useNavigate} from "react-router-dom";

export const UseLoginHook = () => {
    const backend_url: string = process.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new MiscClient(backend_url);
    const redirect = useNavigate();

    return useMutation({
        mutationFn: (request: SetupRequest) => client.setup(request),
        onSuccess: (res) => {
            if (res.getStatus()) {
                redirect("/admin/login")
            }
        }
    })
}
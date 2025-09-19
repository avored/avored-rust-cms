import {MiscClient} from "grpc-avored/MiscServiceClientPb";
import {useMutation} from "@tanstack/react-query";
import {SetupRequest} from "grpc-avored/misc_pb";
import {useNavigate} from "react-router-dom";

export const useSetupPost = () => {
    const backend_url: string = import.meta.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
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
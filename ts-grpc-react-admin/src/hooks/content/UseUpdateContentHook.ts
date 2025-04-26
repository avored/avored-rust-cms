import {useMutation} from "@tanstack/react-query";
import {useNavigate} from "react-router-dom";
import {contentClient} from "../../grpc_generated/ContentServiceClientPb";
import {StoreContentRequest, UpdateContentRequest} from "../../grpc_generated/content_pb";

export const UseUpdateContentHook = () => {
    const backend_url: string = process.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new contentClient(backend_url);
    const redirect = useNavigate();

    return useMutation({
        mutationFn: (request: UpdateContentRequest) => {
            return client.updateContent(request, {
                'Authorization': `Bearer ${localStorage.getItem('token')}`
            })
        },
        onSuccess: (res) => {
            if (res.getStatus()) {
                // localStorage.setItem("token", token);
                redirect("/admin/content");
            }
        }
    })
}
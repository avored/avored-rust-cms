import {useMutation} from "@tanstack/react-query";
import {useNavigate} from "react-router-dom";
import {contentClient} from "grpc-avored/ContentServiceClientPb";
import {PutContentIdentifierRequest} from "grpc-avored/content_pb";

export const UsePutContentIdentifierHook = () => {
    const backend_url: string = import.meta.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new contentClient(backend_url);
    const redirect = useNavigate();

    return useMutation({
        mutationFn: (request: PutContentIdentifierRequest) => {
            return client.putContentIdentifier(request, {
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

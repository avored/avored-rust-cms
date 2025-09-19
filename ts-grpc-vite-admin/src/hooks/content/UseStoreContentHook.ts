import {useMutation} from "@tanstack/react-query";
import {useNavigate} from "react-router-dom";
import {contentClient} from "grpc-avored/ContentServiceClientPb";
import {StoreContentRequest} from "grpc-avored/content_pb";

export const UseStoreContentHook = () => {
    const backend_url: string = import.meta.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new contentClient(backend_url);
    const redirect = useNavigate();

    return useMutation({
        mutationFn: (request: StoreContentRequest) => {
            return client.storeContent(request, {
                'Authorization': `Bearer ${localStorage.getItem('token')}`
            })
        },
        onSuccess: (res, request: StoreContentRequest) => {
            if (res.getStatus()) {
                redirect("/admin/content?type=" + request.getContentType());
            }
        }
    })
}

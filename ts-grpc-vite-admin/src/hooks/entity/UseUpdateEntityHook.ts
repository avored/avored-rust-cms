import {useMutation} from "@tanstack/react-query";
import {useNavigate} from "react-router-dom";
import {EnttyServiceClient} from "grpc-avored/EntityServiceClientPb";
import {UpdateEntityRequest} from "grpc-avored/entity_pb";

export const UseUpdateEntityHook = () => {
    const backend_url: string = import.meta.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new EnttyServiceClient(backend_url);
    const redirect = useNavigate();

    return useMutation({
        mutationFn: (request: UpdateEntityRequest) => {
            return client.updateEntity(request, {
                'Authorization': `Bearer ${localStorage.getItem('token')}`
            })
        },
        onSuccess: (res) => {
            if (res.getStatus()) {
                // localStorage.setItem("token", token);
                redirect("/admin/entity");
            }
        }
    })
}
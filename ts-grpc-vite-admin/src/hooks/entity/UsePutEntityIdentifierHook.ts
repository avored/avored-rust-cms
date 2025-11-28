import {useMutation} from "@tanstack/react-query";
import {useNavigate} from "react-router-dom";
import {EnttyServiceClient} from "grpc-avored/EntityServiceClientPb";
import {PutEntityIdentifierRequest} from "grpc-avored/entity_pb";

//UsePutEntityIdentifierHook
export const UsePutEntityIdentifierHook = () => {
    const backend_url: string = import.meta.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new EnttyServiceClient(backend_url);
    const redirect = useNavigate();

    return useMutation({
        mutationFn: (request: PutEntityIdentifierRequest) => {
            return client.putEntityIdentifier(request, {
                'Authorization': `Bearer ${localStorage.getItem('token')}`
            })
        },
        onSuccess: (res) => {
            if (res.getStatus()) {
                // localStorage.setItem("token", token);
                const role_model = res.getData();
                const id = role_model?.getId() ?? '';

                redirect("/admin/entity-edit/" + id);
            }
        }
    })
}
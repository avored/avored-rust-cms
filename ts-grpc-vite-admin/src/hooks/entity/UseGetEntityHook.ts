import {useQuery} from "@tanstack/react-query";
import {EnttyServiceClient} from "grpc-avored/EntityServiceClientPb";
import {GetEntityRequest} from "grpc-avored/entity_pb";

export const UseGetEntityHook = (request: GetEntityRequest) => {
    const backend_url: string = import.meta.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new EnttyServiceClient(backend_url);

    return useQuery({
        queryKey: ['entity-id', request.getEntityId()],
        queryFn: async () => {
            let response = await client.getEntity(request, {
                'Authorization': `Bearer ${localStorage.getItem('token')}`
            })
            if (response.getStatus()) {
                // may be map a type and return a proper type
                return response.toObject();
            }
            console.log('feel like error thrown... ')
        },
    })
}


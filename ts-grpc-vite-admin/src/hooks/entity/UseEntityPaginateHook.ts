import {useQuery} from "@tanstack/react-query";
import {EnttyServiceClient} from "grpc-avored/EntityServiceClientPb";
import {EntityPaginateRequest} from "grpc-avored/entity_pb";
import {PaginateType} from "../../types/misc/PaginateType";

export const UseEntityPaginateHook = (request: EntityPaginateRequest, query: PaginateType) => {
    const backend_url: string = import.meta.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new EnttyServiceClient(backend_url);

    return useQuery({
        queryKey: ['entity-table', query],
        queryFn: async () => {
            request.setPage(query.page ?? 0);
            request.setOrder(query.order as string)

            let response = await client.paginate(request, {
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


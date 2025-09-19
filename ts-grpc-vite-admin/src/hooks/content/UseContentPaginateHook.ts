import {useQuery} from "@tanstack/react-query";
import {contentClient} from "grpc-avored/ContentServiceClientPb";
import {ContentPaginateRequest} from "grpc-avored/content_pb";
import {PaginateType} from "../../types/misc/PaginateType";

export const UseContentPaginateHook = (request: ContentPaginateRequest, query: PaginateType) => {
    const backend_url: string = import.meta.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new contentClient(backend_url);

    return useQuery({
        queryKey: ['content-table-paginate', 'content-' + request.getContentType(), query],
        queryFn: async () => {
            request.setPage(query.page ?? 0);
            request.setOrder(query.order as string)

            let response = await client.contentPaginate(request, {
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


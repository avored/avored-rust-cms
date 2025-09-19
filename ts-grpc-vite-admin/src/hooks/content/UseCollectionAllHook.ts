import {useQuery} from "@tanstack/react-query";
import {CollectionAllRequest} from "grpc-avored/content_pb";
import {contentClient} from "grpc-avored/ContentServiceClientPb";

export const UseCollectionAllHook = (request: CollectionAllRequest) => {
    const backend_url: string = import.meta.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new contentClient(backend_url);

    return useQuery({
        queryKey: ['collection-all'],
        queryFn: async () => {
            let response = await client.collectionAll(request, {
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


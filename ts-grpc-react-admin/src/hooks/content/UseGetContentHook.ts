import {useQuery} from "@tanstack/react-query";
import {GetContentRequest} from "../../grpc_generated/content_pb";
import {contentClient} from "../../grpc_generated/ContentServiceClientPb";

export const UseGetContentHook = (request: GetContentRequest) => {
    const backend_url: string = process.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new contentClient(backend_url);

    return useQuery({
        queryKey: ['get-content', request.getContentId()],
        queryFn: async () => {
            let response = await client.getContent(request, {
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


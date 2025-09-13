import {useQuery} from "@tanstack/react-query";
import { GetCmsContentRequest } from "grpc-avored/cms_pb";
import { CmsClient } from "grpc-avored/CmsServiceClientPb";


export const useHomeCmsPage = (request: GetCmsContentRequest) => {
    const backend_url: string = "http://localhost:50051";
    const client = new CmsClient(backend_url);

    return useQuery({
        queryKey: ['home-cms-page'],
        queryFn: async () => {
            let response = await client.getCmsContent(request, {
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


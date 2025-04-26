import {useQuery} from "@tanstack/react-query";
import {AdminUserClient} from "../../grpc_generated/Admin_userServiceClientPb";
import {AdminUserPaginateRequest} from "../../grpc_generated/admin_user_pb";
import {contentClient} from "../../grpc_generated/ContentServiceClientPb";
import {ContentPaginateRequest} from "../../grpc_generated/content_pb";

export const UseContentPaginateHook = (request: ContentPaginateRequest) => {
    const backend_url: string = process.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new contentClient(backend_url);

    return useQuery({
        queryKey: ['content-paginate'],
        queryFn: async () => {
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


import {useQuery} from "@tanstack/react-query";
import {AdminUserClient} from "../../grpc_generated/Admin_userServiceClientPb";
import {RolePaginateRequest} from "../../grpc_generated/admin_user_pb";

export const UseRolePaginateHook = (request: RolePaginateRequest) => {
    const backend_url: string = process.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new AdminUserClient(backend_url);

    return useQuery({
        queryKey: ['role'],
        queryFn: async () => {
            let response = await client.rolePaginate(request, {
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


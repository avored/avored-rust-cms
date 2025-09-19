import {useQuery} from "@tanstack/react-query";
import {AdminUserClient} from "grpc-avored/Admin_userServiceClientPb";
import {GetRoleRequest} from "grpc-avored/admin_user_pb";

export const UseGetRoleHook = (request: GetRoleRequest) => {
    const backend_url: string = import.meta.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new AdminUserClient(backend_url);

    return useQuery({
        queryKey: ['role-id', request.getRoleId()],
        queryFn: async () => {
            let response = await client.getRole(request, {
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


import {useQuery} from "@tanstack/react-query";
import {LoggedInUserRequest} from "grpc-avored/general_pb";
import {GeneralServiceClient} from "grpc-avored/GeneralServiceClientPb";

export const UseLoggedInUserHook = (request: LoggedInUserRequest) => {
    const backend_url: string = import.meta.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new GeneralServiceClient(backend_url);

    return useQuery({
        queryKey: ['logged-in-user'],
        queryFn: async () => {
            let response = await client.loggedInUser(request, {
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


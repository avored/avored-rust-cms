import {useQuery} from "@tanstack/react-query";
import { VisitByContentTypeRequest } from "grpc-avored/dashboard_pb";
import { DashboardClient } from "grpc-avored/DashboardServiceClientPb";

export const UseGetVisitByContentTypeHook = (request: VisitByContentTypeRequest) => {
    const backend_url: string = import.meta.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new DashboardClient(backend_url);

    return useQuery({
        queryKey: ['visit-by-content-type', request.getYear(), request.getContentType()],
        queryFn: async () => {
            let response = await client.getVisitByContentType(request, {
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


import {MiscClient} from "grpc-avored/MiscServiceClientPb";
import {useQuery} from "@tanstack/react-query";
import {HealthCheckRequest} from "grpc-avored/misc_pb";

export const useHealthCheckHook = (request: HealthCheckRequest) => {
    const backend_url: string = import.meta.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new MiscClient(backend_url);

    return useQuery({
        queryKey: ['user'],
        queryFn: () => client.healthCheck(request),
    })
};

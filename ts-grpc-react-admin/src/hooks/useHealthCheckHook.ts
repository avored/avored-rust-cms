import {MiscClient} from "../grpc_generated/MiscServiceClientPb";
import {useQuery} from "@tanstack/react-query";
import {HealthCheckRequest} from "../grpc_generated/misc_pb";

export const useHealthCheckHook = (request: HealthCheckRequest) => {
    const client = new MiscClient('http://localhost:50051');

    return useQuery({
        queryKey: ['user'],
        queryFn: () => client.healthCheck(request),
    })
};

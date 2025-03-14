import {MiscClient} from "../../grpc_generated/MiscServiceClientPb";
import {useQuery} from "@tanstack/react-query";
import {HealthCheckRequest} from "../../grpc_generated/misc_pb";

export const useHealthCheckHook = (request: HealthCheckRequest) => {
    const backend_url: string = process.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new MiscClient(backend_url);

    return useQuery({
        queryKey: ['user'],
        queryFn: () => client.healthCheck(request),
    })
};

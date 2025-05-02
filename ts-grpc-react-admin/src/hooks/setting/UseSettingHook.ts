import {useQuery} from "@tanstack/react-query";
import {SettingClient} from "../../grpc_generated/SettingServiceClientPb";
import {GetSettingRequest} from "../../grpc_generated/setting_pb";

export const UseSettingHook = (request: GetSettingRequest) => {
    const backend_url: string = process.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new SettingClient(backend_url);

    return useQuery({
        queryKey: ['settings'],
        queryFn: async () =>  {
            let response = await client.getSetting(request, {
                'Authorization': `Bearer ${localStorage.getItem('token')}`
            })
            if (response.getStatus()) {
                // may be map a type and return a proper type
                return response.toObject();
            }
            console.log('feel like error thrown... ')
        },
    })
};

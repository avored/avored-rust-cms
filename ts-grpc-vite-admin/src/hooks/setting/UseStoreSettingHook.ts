import {useMutation} from "@tanstack/react-query";
import {useNavigate} from "react-router-dom";
import {StoreSettingRequest} from "grpc-avored/setting_pb";
import {SettingClient} from "grpc-avored/SettingServiceClientPb";

export const UseStoreSettingHook = () => {
    const backend_url: string = import.meta.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new SettingClient(backend_url);
    const redirect = useNavigate();

    return useMutation({
        mutationFn: (request: StoreSettingRequest) => {
            return client.storeSetting(request, {
                'Authorization': `Bearer ${localStorage.getItem('token')}`
            })
        },
        onSuccess: (res) => {
            if (res.getStatus()) {
                redirect("/admin/setting")
            }
        }
    })
}
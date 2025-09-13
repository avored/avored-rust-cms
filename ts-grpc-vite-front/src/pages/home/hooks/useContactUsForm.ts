import {useMutation} from "@tanstack/react-query";
import { SentContactFormRequest } from "grpc-avored/cms_pb";
import { CmsClient } from "grpc-avored/CmsServiceClientPb";
import {useNavigate} from "react-router-dom";

export const useContactUsForm = () => {
    const backend_url: string = import.meta.env.VITE_BACKEND_BASE_URL ??  "http://localhost:50051";
    const client = new CmsClient(backend_url);
    const redirect = useNavigate();

    return useMutation({
        mutationFn: async (request: SentContactFormRequest) => {
            let response = await client.sentContactForm(request, {
                'Authorization': `Bearer ${localStorage.getItem('token')}`
            })
            return response;
        },
        onSuccess: (res) => {
            if (res.getStatus()) {
                // localStorage.setItem("token", token);
                redirect("/");
            }
        }
    });
};

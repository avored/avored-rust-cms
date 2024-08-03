import {useAxios} from "../../../hooks/useAxios.ts";
import {useMutation} from "@tanstack/react-query";
import {ContactUsType} from "../../../types/ContactUsType.ts";

export const useContactUsForm = () => {
    const client = useAxios();
    return useMutation({
        mutationFn: async (data: ContactUsType) => {
            return await client.post("/sent-contact-us-email", data);
        }
    });
};

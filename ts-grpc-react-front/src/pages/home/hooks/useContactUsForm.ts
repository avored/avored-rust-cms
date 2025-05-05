import {useMutation} from "@tanstack/react-query";
import {grpc} from "grpc-web-client";

export const useContactUsForm = () => {
    // const client = useAxios();
    return useMutation({
        mutationFn: async (data: any) => {

            return await {};
        }
    });
};

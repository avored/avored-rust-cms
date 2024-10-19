import {useMutation, useQueryClient} from "@tanstack/react-query";
import {useAxios} from "./useAxios";

export const useInstallDemoData = () => {
    const client = useAxios();
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: async () => {
            return await client.post(`/install-demo-data`);
        },
        onSuccess: (res) => {
            queryClient.invalidateQueries({ queryKey: ["page-table", "component-table"] });
        },
    });
};

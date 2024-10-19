import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
import {RenameFolderType} from "../../../types/asset/RenameFolderType";

export const useRenameFolder = () => {
    const client = useAxios();
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: async (data: RenameFolderType) => {
            return await client.post(`/rename-asset/${data.id}`, {name: data.name});
        },
        onSuccess: (res) => {
            queryClient.invalidateQueries({ queryKey: ["asset-table"] });
        },
    });
};

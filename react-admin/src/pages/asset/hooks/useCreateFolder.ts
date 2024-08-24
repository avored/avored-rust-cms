import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
import IAssetSave from "../../../types/asset/IAssetSave";
import {CreateFolderType} from "../../../types/asset/CreateFolderType";

export const useCreateFolder = () => {
    const client = useAxios();
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: async (data: CreateFolderType) => {
            return await client.post("/create-folder", data);
        },
        onSuccess: (res) => {
            queryClient.invalidateQueries({ queryKey: ["asset-table"] });
        },
    });
};

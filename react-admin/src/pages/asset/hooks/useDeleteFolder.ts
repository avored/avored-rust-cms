import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
import IAssetSave from "../../../types/asset/IAssetSave";
import {CreateFolderType} from "../../../types/asset/CreateFolderType";
import {DeleteFolderType} from "../../../types/asset/DeleteFolderType";

export const useDeleteFolder = () => {
    const client = useAxios();
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: async (data: DeleteFolderType) => {
            return await client.delete(`/delete-folder/${data.asset_id}`);
        },
        onSuccess: (res) => {
            queryClient.invalidateQueries({ queryKey: ["asset-table"] });
        },
    });
};

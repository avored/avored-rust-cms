import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
import IAssetSave from "../../../types/asset/IAssetSave";
import {CreateFolderType} from "../../../types/asset/CreateFolderType";
import {isEmpty} from "lodash";

export const useCreateFolder = () => {
    const client = useAxios();
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: async (data: CreateFolderType) => {
            if (isEmpty(data.parent_id)) {
                delete data.parent_id
            }
            return await client.post("/create-folder", data);
        },
        onSuccess: (res) => {
            queryClient.invalidateQueries({ queryKey: ["asset-table"] });
        },
    });
};

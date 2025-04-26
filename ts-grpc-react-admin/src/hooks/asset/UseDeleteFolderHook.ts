import {UseAxiosHook} from "../misc/UseAxiosHook";
import {useMutation, useQueryClient} from "@tanstack/react-query";
import {DeleteFolderType} from "../../types/asset/AssetType";

export const UseDeleteFolderHook = () => {
    const client = UseAxiosHook();
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: async (data: DeleteFolderType) => {
            return await client.delete(`/delete-folder/${data.asset_id}`);
        },
        onSuccess: (res) => {
            queryClient.invalidateQueries({ queryKey: ["asset-table"] });
        },
    });
}
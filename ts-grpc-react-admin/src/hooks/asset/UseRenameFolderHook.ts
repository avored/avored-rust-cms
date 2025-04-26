import {useMutation, useQueryClient} from "@tanstack/react-query";
import {RenameFolderType} from "../../types/asset/AssetType";
import {UseAxiosHook} from "../misc/UseAxiosHook";

export const UseRenameFolderHook = () => {
    const client = UseAxiosHook();
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: async (data: RenameFolderType) => {
            return await client.post(`/rename-asset/${data.id}`, {name: data.name});
        },
        onSuccess: (res) => {
            queryClient.invalidateQueries({ queryKey: ["asset-table"] });
        },
    });
}
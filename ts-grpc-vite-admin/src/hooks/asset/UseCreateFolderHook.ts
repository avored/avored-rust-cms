import {useMutation, useQueryClient} from "@tanstack/react-query";
import {CreateFolderType} from "../../types/asset/AssetType";
import {isEmpty} from "lodash";
import {UseAxiosHook} from "../misc/UseAxiosHook";

export const UseCreateFolderHook = () => {
    const client = UseAxiosHook();
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
}
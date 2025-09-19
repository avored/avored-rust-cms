import {useMutation, useQueryClient} from "@tanstack/react-query";
import {AssetSaveType} from "../../types/asset/AssetType";
import {UseAxiosHook} from "../misc/UseAxiosHook";
import _ from "lodash";

export const UseStoreAssetHook = (parent_id: string) => {
    const client = UseAxiosHook();
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: async (data: AssetSaveType) => {
            const assetUrl: string = _.isEmpty(parent_id) ? '/asset' : '/asset?parent_id=' + parent_id;
            return await client.post(assetUrl, data, {
                headers: {
                    "Content-Type": "multipart/form-data; boundary=----",
                    Authorization: "Bearer " + localStorage.getItem("token"),
                },
            });
        },
        onSuccess: (res) => {
            queryClient.invalidateQueries({ queryKey: ["asset-table"] });
        },
    });
}
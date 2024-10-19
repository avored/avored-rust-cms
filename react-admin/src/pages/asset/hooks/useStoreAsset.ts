import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
import IAssetSave from "../../../types/asset/IAssetSave";
import _ from "lodash";

export const useStoreAsset = (parent_id: string) => {
    const client = useAxios();
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: async (data: IAssetSave) => {
            const assetUrl: string = _.isEmpty(parent_id) ? '/asset' : '/asset?parent_id=' + parent_id;
            return await client.post(assetUrl, data, {
                headers: {
                    "Content-Type": "multipart/form-data; boundary=----",
                    Authorization: "Bearer " + localStorage.getItem("AUTH_TOKEN"),
                },
            });
        },
        onSuccess: (res) => {
            queryClient.invalidateQueries({ queryKey: ["asset-table"] });
        },
    });
};

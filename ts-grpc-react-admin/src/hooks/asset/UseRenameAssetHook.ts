import {useMutation, useQueryClient} from "@tanstack/react-query";
import {AssetClient} from "../../grpc_generated/AssetServiceClientPb";
import {RenameAssetRequest} from "../../grpc_generated/asset_pb";
import {useNavigate} from "react-router-dom";

export const UseRenameAssetHook = () => {
    const backend_url: string = process.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new AssetClient(backend_url);
    const redirect = useNavigate();
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: async (request: RenameAssetRequest) => {
            return await client.renameAsset(request, {
                'Authorization': `Bearer ${localStorage.getItem('token')}`
            });
        },
        onSuccess: async (res) => {
            if (res.getStatus()) {
                await queryClient.invalidateQueries({ queryKey: ['asset-table'] });
                redirect("/admin/asset")
            }
        },
    });
}

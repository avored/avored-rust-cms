import {useMutation, useQueryClient} from "@tanstack/react-query";
import {AssetClient} from "../../grpc_generated/AssetServiceClientPb";
import {DeleteFolderRequest} from "../../grpc_generated/asset_pb";

export const UseDeleteFolderHook = () => {
    const backend_url: string = process.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new AssetClient(backend_url);
    const queryClient = useQueryClient();

    return useMutation({
        mutationFn: async (request: DeleteFolderRequest) => {
            return await client.deleteFolder(request, {
                'Authorization': `Bearer ${localStorage.getItem('token')}`
            });
        },
        onSuccess: (res) => {
            queryClient.invalidateQueries({ queryKey: ["asset-table"] });
        },
    });
}

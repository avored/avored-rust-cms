import {QueryClient, useMutation} from "@tanstack/react-query";
import {useNavigate} from "react-router-dom";
import {contentClient} from "../../grpc_generated/ContentServiceClientPb";
import {UpdateCollectionRequest} from "../../grpc_generated/content_pb";


export const UseUpdateCollectionHook = (refetchCollectionAll: any) => {
    const backend_url: string = process.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new contentClient(backend_url);
    const redirect = useNavigate();

    return useMutation({
        mutationFn: (request: UpdateCollectionRequest) => {
            return client.updateCollection(request, {
                'Authorization': `Bearer ${localStorage.getItem('token')}`
            })
        },
        onSuccess: async (res) => {
            if (res.getStatus()) {
                const queryClient = new QueryClient();
                await queryClient.invalidateQueries({ queryKey: ['collection-all'] });
                redirect("/admin/content?type=" + res.getData()?.getIdentifier());
                await refetchCollectionAll()
            }
        }
    })
}

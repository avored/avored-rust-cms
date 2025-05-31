import {useQuery} from "@tanstack/react-query";
import {AssetClient} from "../../grpc_generated/AssetServiceClientPb";
import {AssetPaginateRequest} from "../../grpc_generated/asset_pb";


export const UseAssetTableHook = (request: AssetPaginateRequest) => {
    const backend_url: string = process.env.REACT_APP_BACKEND_BASE_URL ?? "http://localhost:50051";
    const client = new AssetClient(backend_url);


    return useQuery({
        queryKey: ['asset-table'],
        queryFn: (async () => {
            try {
                let response = await client.paginate(request, {
                    'Authorization': `Bearer ${localStorage.getItem('token')}`
                })
                if (response.getStatus()) {
                    // may be map a type and return a proper type
                    return response.toObject();
                }
                console.log('feel like error thrown... ')
            } catch (error) {
                console.log(error)
            }
        })
    })
}

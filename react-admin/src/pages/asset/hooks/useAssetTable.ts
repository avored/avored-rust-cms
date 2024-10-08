import {useQuery} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'

export const useAssetTable = (asset_id: string) => {
    const client = useAxios();
    const redirect = useNavigate();
    return useQuery({
        queryKey: ['asset-table', {asset_id}],
        queryFn: (async () => {
            try {
                const assetUrl: string = _.isEmpty(asset_id) ? '/asset' : '/asset?parent_id=' + asset_id;
                return await client.get(assetUrl)
            } catch (error) {
                if (_.get(error, 'response.status') === 401) {
                    localStorage.removeItem('AUTH_TOKEN')
                    redirect("/admin/login")
                }
            }
        })
    })
}
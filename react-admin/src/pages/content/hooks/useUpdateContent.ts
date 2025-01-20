import {useMutation} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'
import {SaveContentType} from "../../../types/content/ContentType"

export const useUpdateContent = (content_id: string, collection_type: string) => {
    const client = useAxios();
    const redirect = useNavigate();
    return useMutation({
        mutationFn: async (data: SaveContentType) => {
            const url = `/content/'${collection_type}/${content_id}`;
            return await client.put(url , JSON.stringify(data));
        },
        onSuccess: (res) => {
            if (_.get(res, 'data.status') === true) {
                redirect("/admin/page")
            }
        }
    })
}
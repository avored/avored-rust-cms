import {useMutation} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'
import {PutCollectionIdentifierType} from "../../../types/collection/PutCollectionIdentifierType";

export const usePutCollectionIdentifier = (collection_id: string) => {
    const client = useAxios();
    const redirect = useNavigate();
    return useMutation({
        mutationFn: async (data: PutCollectionIdentifierType) => {
            const url = '/put-collection-identifier/'  + collection_id;
            return await client.put(url , JSON.stringify(data));
        },
        onSuccess: (res) => {
            if (_.get(res, 'data.status') === true) {
                redirect("/admin/collection-edit/" + collection_id)
            }
        }
    })
}
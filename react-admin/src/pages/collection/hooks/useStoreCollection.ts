import {useMutation} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'
import {CreatableCollectionType} from "../../../types/collection/CreatableCollectionType";

export const useStoreCollection = () => {
    const client = useAxios();
    const redirect = useNavigate();
    return useMutation({
        mutationFn: async (data: CreatableCollectionType) => {
            return await client.post('/collection', JSON.stringify(data));
        },
        onSuccess: (res) => {
            if (_.get(res, 'data.status') === true) {
                redirect("/admin/collections")
            }
        }
    })
}
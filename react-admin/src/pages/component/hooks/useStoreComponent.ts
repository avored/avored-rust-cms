import {useMutation} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'
import CreatableFieldStoreData from "../../../types/field/CreatableFieldStoreData";

export const useStoreComponent = () => {
    const client = useAxios();
    const redirect = useNavigate();
    return useMutation({
        mutationFn: async (data: CreatableFieldStoreData) => {
            return await client.post('/component', JSON.stringify(data));
        },
        onSuccess: (res) => {
            if (_.get(res, 'data.status') === true) {
                redirect("/admin/component")
            }
        }
    })
}
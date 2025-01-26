import {useMutation} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'
import IEditableModel from "../../../types/model/IEditableModel";
import {SavableCollectionType} from "../../../types/collection/CreatableCollectionType";

export const useUpdateCollection = (role_id: string) => {
    const client = useAxios();
    const redirect = useNavigate();
    return useMutation({
        mutationFn: async (data: SavableCollectionType) => {
            const url = '/collection/'  + role_id;
            return await client.put(url , JSON.stringify(data));
        },
        onSuccess: (res) => {
            if (_.get(res, 'data.status') === true) {
                redirect("/admin/collections")
            }
        }
    })
}
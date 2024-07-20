import {useMutation} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'
import {PutModelIdentifierType} from "../../../types/model/PutModelIdentifierType";

export const usePutModelIdentifier = (model_id: string) => {
    const client = useAxios();
    const redirect = useNavigate();
    return useMutation({
        mutationFn: async (data: PutModelIdentifierType) => {
            const url = '/put-model-identifier/'  + model_id;
            return await client.put(url , JSON.stringify(data));
        },
        onSuccess: (res) => {
            if (_.get(res, 'data.status') === true) {
                redirect("/admin/model-edit/" + model_id)
            }
        }
    })
}
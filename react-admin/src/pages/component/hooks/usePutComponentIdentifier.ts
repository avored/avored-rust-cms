import {useMutation} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'
import {PutComponentIdentifierType} from "../../../types/component/PutComponentIdentifierType";

export const usePutComponentIdentifier = (component_id: string) => {
    const client = useAxios();
    const redirect = useNavigate();
    return useMutation({
        mutationFn: async (data: PutComponentIdentifierType) => {
            const url = '/put-component-identifier/'  + component_id;
            return await client.put(url , JSON.stringify(data));
        },
        onSuccess: (res) => {
            if (_.get(res, 'data.status') === true) {
                redirect("/admin/component-edit/" + component_id)
            }
        }
    })
}
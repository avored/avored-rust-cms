import {useMutation} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'
import {PutRoleIdentifierType} from "../../../types/role/PutRoleIdentifierType";

export const usePutRoleIdentifier = (role_id: string) => {
    const client = useAxios();
    const redirect = useNavigate();
    return useMutation({
        mutationFn: async (data: PutRoleIdentifierType) => {
            const url = '/put-role-identifier/'  + role_id;
            return await client.put(url , JSON.stringify(data));
        },
        onSuccess: (res) => {
            if (_.get(res, 'data.status') === true) {
                redirect("/admin/role-edit/" + role_id)
            }
        }
    })
}
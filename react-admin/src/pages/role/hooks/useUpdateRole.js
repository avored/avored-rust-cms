import {useMutation} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'

export const useUpdateRole = (role_id) => {
    const client = useAxios();
    const redirect = useNavigate();
    return useMutation({
        mutationFn: async (data) => {
            const url = '/role/'  + role_id;
            return await client.put(url , JSON.stringify(data));
        },
        onSuccess: (res) => {
            if (_.get(res, 'data.status') === true) {
                redirect("/admin/role")
            }
        }
    })
}
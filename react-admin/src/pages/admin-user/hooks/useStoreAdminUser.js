import {useMutation, useQuery} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'

export const useStoreAdminUser = () => {
    const client = useAxios();
    const redirect = useNavigate();
    return useMutation({
        mutationFn: async (data) => {
            return await client.post('/admin-user', data, {
                headers: {
                    'Content-Type': 'multipart/form-data; boundary=----',
                    'Authorization': 'Bearer ' + localStorage.getItem("AUTH_TOKEN"),
                },
            });
        },
        onSuccess: (res) => {
            if (_.get(res, 'data.status') === true) {
                redirect("/admin/admin-user")
            }
        }
    })
}
import {useMutation} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import {useNavigate} from 'react-router-dom'
import _ from 'lodash'

export const useLogin = () => {
    const client = useAxios();
    const redirect = useNavigate();
    return useMutation({
        mutationFn: async (data) => {
            return await client.post('/login', data);
        },
        onSuccess: (res) => {
            // will set the res.data.data (which is the access token)
            localStorage.setItem("AUTH_TOKEN", res.data.data);
            localStorage.setItem("AUTH_ADMIN_USER", JSON.stringify(_.get(res, 'data.admin_user')));

            // and redirect to the admin.
            redirect("/admin");
        }
   })
}
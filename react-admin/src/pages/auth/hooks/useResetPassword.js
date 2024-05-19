import {useMutation} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import {useNavigate} from 'react-router-dom'

export const useResetPassword = () => {
    const client = useAxios();
    const redirect = useNavigate();
    return useMutation({
        mutationFn: async (data) => {
            return await client.post('/reset-password', data);
        },
        onSuccess: (res) => {
            redirect("/admin/login");
        }
   })
}
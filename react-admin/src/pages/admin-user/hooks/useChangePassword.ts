import {useMutation} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import {useNavigate} from 'react-router-dom'
import IChangePasswordPost from "../../../types/auth/ChangePasswordPostType";

export const useChangePassword = () => {
    const client = useAxios();
    const redirect = useNavigate();
    return useMutation({
        mutationFn: async (data: IChangePasswordPost) => {
            return await client.post('/change-password', data);
        },
        onSuccess: (res) => {
            redirect("/admin/login");
        }
   })
}

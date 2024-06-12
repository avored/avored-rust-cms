import {useMutation} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import {useNavigate} from 'react-router-dom'
import IForgotPasswordPost from "../../../types/auth/IForgotPasswordPost";


export const useForgotPassword = () => {
    const client = useAxios();
    const redirect = useNavigate();
    return useMutation({
        mutationFn: async (data: IForgotPasswordPost) => {
            return await client.post('/forgot-password', data);
        },
        onSuccess: (res) => {
            redirect("/admin/login");
        }
   })
}
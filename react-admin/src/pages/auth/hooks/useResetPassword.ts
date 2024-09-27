import {useMutation} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import {useNavigate} from 'react-router-dom'
import IResetPasswordPost from "../../../types/auth/IResetPasswordPost";
import {AxiosError} from "axios";

export const useResetPassword = () => {
    const client = useAxios();
    const redirect = useNavigate();
    return useMutation({
        mutationFn: async (data: IResetPasswordPost) => {
            return await client.post('/reset-password', data);
        },
        onSuccess: (res) => {
            redirect("/admin/login");
        },
        onError: (error: AxiosError) => {
            if (error.status === 404) {
                // @todo improve with snackbar / toast notification
                alert(error.response?.data)
            }

            throw error
        }
   })
}
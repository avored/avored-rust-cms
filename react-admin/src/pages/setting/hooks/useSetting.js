import {useQuery} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'

export const useSetting = () => {
    const client = useAxios();
    const redirect = useNavigate();

    return useQuery({
        queryKey: ['setting'],
        queryFn: (async () => {
            try {
                return await client.get("/setting")
            } catch (error) {
                if (_.get(error, 'response.status') === 401) {
                    localStorage.removeItem('AUTH_TOKEN')
                    redirect("/admin/login")
                }
            }
        })
    })
}
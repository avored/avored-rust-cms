import {useQuery} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'

export const useOpenapi = () => {
    const client = useAxios();
    const redirect = useNavigate();
    return useQuery({
        queryKey: ['openapi'],
        queryFn: (async () => {
            try {
                return await client.get("/openapi.json")
            } catch (error) {
                if (_.get(error, 'response.status') === 401) {
                    localStorage.removeItem('AUTH_TOKEN')
                    redirect("/admin/login")
                }
            }
        })
    })
}
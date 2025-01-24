import {useQuery} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'

export const useModelAll = () => {

    const client = useAxios();
    const redirect = useNavigate();
    return useQuery({
        queryKey: ['model-all'],
        queryFn: (async () => {
            try {
                return await client.get("/model-all")
            } catch (error) {
                if (_.get(error, 'response.status') === 401) {
                    localStorage.removeItem('AUTH_TOKEN')
                    redirect("/admin/login")
                }
                
                console.error(error)
            }
        })
    })
}
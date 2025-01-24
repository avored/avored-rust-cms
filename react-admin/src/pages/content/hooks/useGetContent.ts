import {useQuery} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'

export const useGetContent = (content_id: string, collection_type: string) => {
    const client = useAxios()
    const redirect = useNavigate()

    return useQuery({
        queryKey: ['content', content_id],
        queryFn: (async () => {
            try {
                return await client.get(`/content/${collection_type}/${content_id}`)
            } catch (error) {
                if (_.get(error, 'response.status') === 401) {
                    localStorage.removeItem('AUTH_TOKEN')
                    redirect("/admin/login")
                }
            }
        })
    })
}
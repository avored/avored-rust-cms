import {useQuery} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'

export const useGetCollectionByIdentifier = (collection_identifier: string) => {
    const client = useAxios()
    const redirect = useNavigate()

    return useQuery({
        queryKey: ['collection-identifier', collection_identifier],
        queryFn: (async () => {
            try {
                return await client.get(`/collection-identifier/${collection_identifier}`)
            } catch (error) {
                if (_.get(error, 'response.status') === 401) {
                    localStorage.removeItem('AUTH_TOKEN')
                    redirect("/admin/login")
                }
            }
        })
    })
}
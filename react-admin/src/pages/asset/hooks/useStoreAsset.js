import {useMutation, useQuery, useQueryClient} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'

export const useStoreAsset = () => {
    const client = useAxios()
    const redirect = useNavigate()
    const queryClient = useQueryClient()

    return useMutation({
        mutationFn: async (data) => {
            return await client.post('/asset', data, {
                headers: {
                    'Content-Type': 'multipart/form-data; boundary=----',
                    'Authorization': 'Bearer ' + localStorage.getItem("AUTH_TOKEN"),
                },
            });
        },
        onSuccess: (res) => {
            queryClient.invalidateQueries({queryKey: ['asset-table']})
        }
    })
}
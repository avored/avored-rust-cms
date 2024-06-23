import {useQuery, useQueryClient} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'

type PaginateType = {
    order?: string;
}

export const useAdminUserTable = (query: PaginateType) => {
    let query_string = "";
    if (query.order !== "") {
        const queryKey = query.order
        query_string = "?" + new URLSearchParams(query).toString()
    }

    const client = useAxios();
    const redirect = useNavigate();
    return useQuery({
        queryKey: ['admin-user-table'],
        queryFn: (async () => {
            try {
                return await client.get("/admin-user" + query_string)
            } catch (error) {
                if (_.get(error, 'response.status') === 401) {
                    localStorage.removeItem('AUTH_TOKEN')
                    redirect("/admin/login")
                }
            }
        }),

    })
}
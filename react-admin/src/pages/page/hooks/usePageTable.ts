import {useQuery} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'
import PaginateType from "../../../types/misc/PaginateType";

export const usePageTable = (query: PaginateType) => {
    let query_string = "";
    if (query.order !== "") {
        const queryKey = query.order
        query_string = "?" + new URLSearchParams(query).toString()
    }
    const client = useAxios();
    const redirect = useNavigate();
    return useQuery({
        queryKey: ['page-table'],
        queryFn: (async () => {
            try {
                return await client.get("/page" + query_string)
            } catch (error) {
                if (_.get(error, 'response.status') === 401) {
                    localStorage.removeItem('AUTH_TOKEN')
                    redirect("/admin/login")
                }
            }
        })
    })
}
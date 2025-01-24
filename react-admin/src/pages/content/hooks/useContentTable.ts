import {useQuery} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'
import PaginateType from "../../../types/misc/PaginateType";

export const useContentTable = (query: PaginateType, collectionType: string | null) => {
    let params: URLSearchParams = new URLSearchParams();
    if (query.page && query.page > 0) {
        params.append("page", query.page.toString())
    }
    if (query.order && query.order !== "") {
        params.append("order", query.order)
    }
    let query_string = "";
    if (params.toString() !== "") {
        query_string = "?" + params.toString()
    }

    const client = useAxios();
    const redirect = useNavigate();
    return useQuery({
        queryKey: ['content-table', query, collectionType],
        queryFn: (async () => {
            // early exit
            if (_.isEmpty(collectionType)) {
                return { data: undefined }
            }

            try {

                return await client.get(`/content/${collectionType?.trim()}${query_string}`)
            } catch (error) {
                if (_.get(error, 'response.status') === 401) {
                    localStorage.removeItem('AUTH_TOKEN')
                    redirect("/admin/login")
                }
            }
        })
    })
}

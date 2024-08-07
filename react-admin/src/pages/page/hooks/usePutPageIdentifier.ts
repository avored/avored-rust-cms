import {useMutation} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'
import IEditableRole from "../../../types/role/IEditableRole";
import {PutRoleIdentifierType} from "../../../types/role/PutRoleIdentifierType";

export const usePutPageIdentifier = (page_id: string) => {
    const client = useAxios();
    const redirect = useNavigate();
    return useMutation({
        mutationFn: async (data: PutRoleIdentifierType) => {
            const url = '/put-page-identifier/'  + page_id;
            return await client.put(url , JSON.stringify(data));
        },
        onSuccess: (res) => {
            if (_.get(res, 'data.status') === true) {
                redirect("/admin/page-edit/" + page_id)
            }
        }
    })
}
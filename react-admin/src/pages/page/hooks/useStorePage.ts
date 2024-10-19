import {useMutation} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'
import {SavePageType} from "../../../types/page/CreatablePageType";

export const useStorePage = () => {
    const client = useAxios();
    const redirect = useNavigate();
    return useMutation({
        mutationFn: async (data: SavePageType) => {
            // @todo possible fixed this None(String) | Object union type issue
            const postdata : any = data;

            postdata.page_fields.map((field: any) => {
                if (typeof field.field_data === 'undefined') {
                    // We do pass none as to map with the rust None enum type(basically it is null)
                    field.field_data = "None"
                }
            })

            return await client.post('/page', JSON.stringify(postdata));
        },
        onSuccess: (res) => {
            if (_.get(res, 'data.status') === true) {
                redirect("/admin/page")
            }
        }
    })
}
import {useMutation} from '@tanstack/react-query'
import { useAxios } from '../../../hooks/useAxios'
import _ from 'lodash'
import {useNavigate} from 'react-router-dom'
import SaveSettingType from '../../../types/settings/SaveSettingType'

export const useStoreSetting = () => {
    const client = useAxios();
    const redirect = useNavigate();
    return useMutation({
        mutationFn: async (data: SaveSettingType) => {
            return await client.post('/setting', JSON.stringify(data));
        },
        onSuccess: (res) => {
            if (_.get(res, 'data.status') === true) {
                //@todo display some kind a flash message
                redirect("/admin/setting")
            }
        }
    })
}

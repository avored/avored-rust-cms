import {useAxios} from "../../../hooks/useAxios";
import {useMutation} from '@tanstack/react-query'
import _ from "lodash";

export const useDeletePage = () => {
  const client = useAxios();
  return useMutation({
    mutationFn: async (pageID: string) => {
      if (!pageID) throw new Error('Page id required');
      const url = `/page/${pageID}`
      return await client.delete(url)
    },
    onSuccess: (res) => {
      if (_.get(res, 'data.status') === true) {
        console.info('Page removed successfully')
      }
    },
  })
}

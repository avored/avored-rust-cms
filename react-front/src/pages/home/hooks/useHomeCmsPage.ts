import { useQuery, UseQueryResult } from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
import { RepositoryInformationType } from "../../../types/RepositoryInformationType"
import { AxiosResponse } from "axios";
import {CmsPageType} from "../../../types/CmsPageType";

export const useHomeCmsPage = (): UseQueryResult<AxiosResponse<CmsPageType>> => {
    const client = useAxios();
    return useQuery({
        queryKey: ['home-cms-page'],
        queryFn: (async () => {
            try {
                return await client.get<CmsPageType>("/cms/page/wvb4100904eaf3ykz64c")
            } catch (error) {
                //@todo display error
            }
        })
    })
}

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
                return await client.get<CmsPageType>("/cms/page/ym2z7ls4kpb4gp5kh7d2")
            } catch (error) {
                //@todo display error
            }
        })
    })
}

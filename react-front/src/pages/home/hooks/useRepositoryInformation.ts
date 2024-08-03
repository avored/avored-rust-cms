import { useQuery, UseQueryResult } from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
import { RepositoryInformationType } from "../../../types/RepositoryInformationType"
import { AxiosResponse } from "axios";

export const useRepositoryInformation = (): UseQueryResult<AxiosResponse<RepositoryInformationType>> => {
    const client = useAxios();
    return useQuery({
        queryKey: ['repository-information'],
        queryFn: (async () => {
            try {
                return await client.get<RepositoryInformationType>("/repository-information")
            } catch (error) {
                //@todo display error
            }
        })
    })
}

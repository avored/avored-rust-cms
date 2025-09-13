import { useQuery } from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
import { RepositoryInformationType } from "../../../types/RepositoryInformationType"

export const useRepositoryInformation = () => {
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

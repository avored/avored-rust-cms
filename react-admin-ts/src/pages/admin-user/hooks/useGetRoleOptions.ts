import { useQuery, UseQueryResult } from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
import _ from "lodash";
import { useNavigate } from "react-router-dom";
import { AxiosInstance } from "axios";

interface RoleOption {
  id: string;
  name: string;
}

export const useGetRoleOptions = (): UseQueryResult<RoleOption[], unknown> => {
  const client: AxiosInstance = useAxios();
  const redirect = useNavigate();

  return useQuery<RoleOption[], unknown>({
    queryKey: ["role-options"],
    queryFn: async () => {
      try {
        const response = await client.get<RoleOption[]>("/role-options");
        return response.data;
      } catch (error) {
        if (_.get(error, "response.status") === 401) {
          localStorage.removeItem("AUTH_TOKEN");
          redirect("/admin/login");
        }
        throw error;
      }
    },
  });
};

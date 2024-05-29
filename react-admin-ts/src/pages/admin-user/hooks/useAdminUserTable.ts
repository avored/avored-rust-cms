import { useQuery, UseQueryResult } from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
import _ from "lodash";
import { useNavigate } from "react-router-dom";
import { AxiosResponse } from "axios";

type AdminUser = {
  id: string;
  full_name: string;
  email: string;
  created_at: string;
  updated_at: string;
  created_by: string;
  updated_by: string;
};

interface AdminUserTableResponse {
  data: AdminUser[];
}

export const useAdminUserTable = (): UseQueryResult<
  AxiosResponse<AdminUserTableResponse>
> => {
  const client = useAxios();
  const redirect = useNavigate();

  return useQuery<AxiosResponse<AdminUserTableResponse>>({
    queryKey: ["admin-user-table"],
    queryFn: async () => {
      try {
        return await client.get<AdminUserTableResponse>("/admin-user");
      } catch (error: string | unknown) {
        if (_.get(error, "response.status") === 401) {
          localStorage.removeItem("AUTH_TOKEN");
          redirect("/admin/login");
        }
        throw error; // Re-throw the error to ensure the query shows an error state
      }
    },
  });
};

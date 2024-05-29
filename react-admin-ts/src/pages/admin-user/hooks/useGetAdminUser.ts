import { useQuery, UseQueryResult } from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
import _ from "lodash";
import { useNavigate } from "react-router-dom";
import { AxiosInstance } from "axios";

interface AdminUser {
  id: string;
  full_name: string;
  email: string;
  created_at: string;
  updated_at: string;
  created_by: string;
  updated_by: string;
}

export const useGetAdminUser = (
  adminUserId: string
): UseQueryResult<AdminUser, unknown> => {
  const client: AxiosInstance = useAxios();
  const redirect = useNavigate();

  return useQuery<AdminUser, unknown>({
    queryKey: ["admin-user", adminUserId],
    queryFn: async () => {
      try {
        const response = await client.get<AdminUser>(
          "/admin-user/" + adminUserId
        );
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

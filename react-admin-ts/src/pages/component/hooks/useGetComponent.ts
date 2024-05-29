import { useQuery, UseQueryResult } from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
import _ from "lodash";
import { useNavigate } from "react-router-dom";

export interface Component {
  id: number;
  name: string;
  description: string;
  created_at: string;
  updated_at: string;
}

export interface ApiResponse {
  data: {
    component: Component;
  };
}

export const useGetComponent = (
  component_id: string
): UseQueryResult<ApiResponse, unknown> => {
  const client = useAxios();
  const redirect = useNavigate();

  return useQuery<ApiResponse, unknown>({
    queryKey: ["component", component_id],
    queryFn: async () => {
      try {
        const response = await client.get<ApiResponse>(
          "/component/" + component_id
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

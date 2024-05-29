import {
  QueryObserverResult,
  useQuery,
  UseQueryOptions,
} from "@tanstack/react-query";
import { AxiosInstance } from "axios";
import { useAxios } from "../../../hooks/useAxios";
import _ from "lodash";
import { useNavigate, NavigateFunction } from "react-router-dom";

interface SettingResponse {
  data: any;
}

export const useSetting = (): QueryObserverResult<SettingResponse, Error> => {
  const client: AxiosInstance = useAxios();
  const redirect: NavigateFunction = useNavigate();

  const queryOptions: UseQueryOptions<SettingResponse, Error> = {
    queryKey: ["setting"],
    queryFn: async () => {
      try {
        const response = await client.get<SettingResponse>("/setting");
        return response.data;
      } catch (error) {
        if (_.get(error, "response.status") === 401) {
          localStorage.removeItem("AUTH_TOKEN");
          redirect("/admin/login");
        }
        throw error;
      }
    },
  };

  return useQuery(queryOptions);
};

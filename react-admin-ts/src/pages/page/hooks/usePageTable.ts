import {
  useQuery,
  UseQueryOptions,
  UseQueryResult,
} from "@tanstack/react-query";
import { AxiosInstance } from "axios";
import { useAxios } from "../../../hooks/useAxios";
import _ from "lodash";
import { useNavigate, NavigateFunction } from "react-router-dom";

interface PageResponse {
  data: any;
}

export const usePageTable = (): UseQueryResult<PageResponse, Error> => {
  const client: AxiosInstance = useAxios();
  const redirect: NavigateFunction = useNavigate();

  const queryFn = async (): Promise<PageResponse> => {
    try {
      return await client.get<PageResponse>("/page");
    } catch (error) {
      if (_.get(error, "response.status") === 401) {
        localStorage.removeItem("AUTH_TOKEN");
        redirect("/admin/login");
      }
      throw error;
    }
  };

  const queryOptions: UseQueryOptions<PageResponse, Error> = {
    queryKey: ["page-table"],
    queryFn,
  };

  return useQuery(queryOptions);
};

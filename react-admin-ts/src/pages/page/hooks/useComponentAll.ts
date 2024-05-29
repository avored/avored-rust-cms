import {
  useQuery,
  UseQueryOptions,
  UseQueryResult,
} from "@tanstack/react-query";
import { AxiosInstance } from "axios";
import { useAxios } from "../../../hooks/useAxios";
import _ from "lodash";
import { useNavigate, NavigateFunction } from "react-router-dom";

type ComponentResponse = any; // Replace 'any' with the type of response you expect from '/component-all' endpoint

export const useComponentAll = (): UseQueryResult<ComponentResponse, Error> => {
  const client: AxiosInstance = useAxios();
  const redirect: NavigateFunction = useNavigate();

  const queryFn = async (): Promise<ComponentResponse> => {
    try {
      return await client.get<ComponentResponse>("/component-all");
    } catch (error) {
      if (_.get(error, "response.status") === 401) {
        localStorage.removeItem("AUTH_TOKEN");
        redirect("/admin/login");
      }
      throw error;
    }
  };

  const queryOptions: UseQueryOptions<ComponentResponse, Error> = {
    queryKey: ["component-all"],
    queryFn,
  };

  return useQuery(queryOptions);
};

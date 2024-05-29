import {
  MutationFunction,
  useMutation,
  UseMutationResult,
} from "@tanstack/react-query";
import { AxiosInstance } from "axios";
import { useAxios } from "../../../hooks/useAxios";
import _ from "lodash";
import { useNavigate, NavigateFunction } from "react-router-dom";

interface PageData {
  title: string;
  content: string;
}

interface PageResponse {
  status: boolean;
  data: {
    status: boolean;
  };
}

export const useUpdatePage = (
  page_id: string
): UseMutationResult<PageResponse, Error, PageData> => {
  const client: AxiosInstance = useAxios();
  const redirect: NavigateFunction = useNavigate();

  const mutationFn: MutationFunction<PageResponse, PageData> = async (
    data: PageData
  ) => {
    const url = "/page/" + page_id;
    const response = await client.put<PageResponse>(url, JSON.stringify(data));
    return response.data;
  };

  const onSuccess = (res: PageResponse) => {
    if (_.get(res, "data.status") === true) {
      redirect("/admin/page");
    }
  };

  return useMutation<PageResponse, Error, PageData>({
    mutationFn,
    onSuccess,
  });
};

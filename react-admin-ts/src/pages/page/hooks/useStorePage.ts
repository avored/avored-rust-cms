import {
  useMutation,
  MutationFunction,
  UseMutationResult,
} from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
import _ from "lodash";
import { useNavigate } from "react-router-dom";

interface PageData {
  title: string;
  content: string;
}

interface ResponseData {
  status: boolean;
  data: {
    status: boolean;
  };
}

export const useStorePage = (): UseMutationResult<
  ResponseData,
  unknown,
  PageData,
  unknown
> => {
  const client = useAxios();
  const redirect = useNavigate();
  const mutationFn: MutationFunction<ResponseData, PageData> = async (
    data: PageData
  ): Promise<ResponseData> => {
    const response = await client.post<ResponseData>(
      "/page",
      JSON.stringify(data)
    );
    return { status: response.data.status, data: response.data };
  };

  return useMutation({
    mutationFn,
    onSuccess: (res: ResponseData) => {
      if (_.get(res, "data.status") === true) {
        redirect("/admin/page");
      }
    },
  });
};

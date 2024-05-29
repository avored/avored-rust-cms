import { useMutation, UseMutationResult } from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
import _ from "lodash";
import { useNavigate } from "react-router-dom";

interface SetupData {
  email: string;
  password: string;
}

interface ApiResponse {
  data: {
    status: boolean;
  };
}

export const useStoreSetup = (): UseMutationResult<
  ApiResponse,
  unknown,
  SetupData,
  unknown
> => {
  const client = useAxios();
  const redirect = useNavigate();
  return useMutation<ApiResponse, unknown, SetupData, unknown>({
    mutationFn: async (data: SetupData) => {
      const response = await client.post<ApiResponse>(
        "/setup",
        JSON.stringify(data)
      );
      return response.data;
    },
    onSuccess: (res: ApiResponse) => {
      if (_.get(res, "data.status") === true) {
        redirect("/admin/login");
      }
    },
  });
};

import {
  MutationFunction,
  useMutation,
  UseMutationResult,
} from "@tanstack/react-query";
import { AxiosInstance } from "axios";
import { useAxios } from "../../../hooks/useAxios";
import _ from "lodash";
import { useNavigate, NavigateFunction } from "react-router-dom";

interface RoleData {
  name: string;
  description: string;
  identifier: string;
  permissions: string[];
}

interface RoleResponse {
  status: boolean;
  data: {
    status: boolean;
  };
}

export const useStoreRole = (): UseMutationResult<
  RoleResponse,
  Error,
  RoleData
> => {
  const client: AxiosInstance = useAxios();
  const redirect: NavigateFunction = useNavigate();

  const mutationFn: MutationFunction<RoleResponse, RoleData> = async (
    data: RoleData
  ): Promise<RoleResponse> => {
    const url = "/role";
    const response = await client.post<RoleResponse>(url, JSON.stringify(data));
    return {
      status: response.data.status,
      data: { status: response.data.status },
    };
  };

  const onSuccess = (res: RoleResponse) => {
    if (_.get(res, "data.status") === true) {
      redirect("/admin/role");
    } else {
      // Handle the case when status is not true
      console.error("Failed to store role:", res);
    }
  };

  const onError = (error: Error) => {
    // Handle the error
    console.error("Error occurred while storing role:", error);
  };

  return useMutation<RoleResponse, Error, RoleData>({
    mutationFn,
    onSuccess,
    onError,
  });
};

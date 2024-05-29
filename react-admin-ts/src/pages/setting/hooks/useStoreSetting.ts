import {
  useMutation,
  MutationFunction,
  MutationOptions,
  UseMutationResult,
} from "@tanstack/react-query";
import { AxiosInstance } from "axios";
import { useAxios } from "../../../hooks/useAxios";
import _ from "lodash";
import { useNavigate, NavigateFunction } from "react-router-dom";

interface SettingData {
  settings: any;
  email: string;
  password: string;
}

interface SettingResponse {
  data: {
    status: boolean;
  };
}

export const useStoreSetting = (): UseMutationResult<
  SettingResponse,
  Error,
  SettingData,
  unknown
> => {
  const client: AxiosInstance = useAxios();
  const redirect: NavigateFunction = useNavigate();

  const mutationFn: MutationFunction<SettingResponse, SettingData> = async (
    data: SettingData
  ) => {
    const response = await client.post<SettingResponse>(
      "/setting",
      JSON.stringify(data)
    );
    return response.data;
  };

  const mutationOptions: MutationOptions<
    SettingResponse,
    Error,
    SettingData,
    unknown
  > = {
    mutationFn,
    onSuccess: (res: SettingResponse) => {
      if (_.get(res, "data.status") === true) {
        //@todo display some kind a flash message
        redirect("/admin/setting");
      }
    },
  };

  return useMutation(mutationOptions);
};

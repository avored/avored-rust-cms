import {
  useMutation,
  MutationFunction,
  MutationOptions,
} from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
import { useNavigate } from "react-router-dom";

interface ResetPasswordData {
  // Define the structure of the reset password data here
  [key: string]: any;
}

interface ResponseData {
  // Define the structure of the response data here
  [key: string]: any;
}

export const useResetPassword = () => {
  const client = useAxios();
  const redirect = useNavigate();

  const mutationFn: MutationFunction<ResponseData, ResetPasswordData> = async (
    data
  ) => {
    return await client.post<ResponseData>("/reset-password", data);
  };

  const mutationOptions: MutationOptions<
    ResponseData,
    unknown,
    ResetPasswordData
  > = {
    mutationFn,
    onSuccess: () => {
      redirect("/admin/login");
    },
  };

  return useMutation(mutationOptions);
};

import {
  useMutation,
  MutationFunction,
  MutationOptions,
} from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
import { useNavigate } from "react-router-dom";

interface ForgotPasswordData {
  // Define the structure of the forgot password data here
  [key: string]: any;
}

interface ResponseData {
  // Define the structure of the response data here
  [key: string]: any;
}

export const useForgotPassword = () => {
  const client = useAxios();
  const redirect = useNavigate();

  const mutationFn: MutationFunction<ResponseData, ForgotPasswordData> = async (
    data
  ) => {
    return await client.post<ResponseData>("/forgot-password", data);
  };

  const mutationOptions: MutationOptions<
    ResponseData,
    unknown,
    ForgotPasswordData
  > = {
    mutationFn,
    onSuccess: () => {
      redirect("/admin/login");
    },
  };

  return useMutation(mutationOptions);
};

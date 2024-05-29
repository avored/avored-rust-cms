import {
  useMutation,
  MutationFunction,
  MutationOptions,
} from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
import { useNavigate } from "react-router-dom";
import _ from "lodash";

interface LoginData {
  // Define the structure of the login data here
  [key: string]: any;
}

interface ResponseData {
  data: {
    data: string;
    admin_user: any; // Replace with the actual type of admin_user
  };
}

export const useLogin = () => {
  const client = useAxios();
  const redirect = useNavigate();

  const mutationFn: MutationFunction<ResponseData, LoginData> = async (
    data
  ) => {
    const response = await client.post<ResponseData>("/login", data);
    return response.data;
  };

  const mutationOptions: MutationOptions<ResponseData, unknown, LoginData> = {
    mutationFn,
    onSuccess: (res) => {
      // will set the res.data.data (which is the access token)
      localStorage.setItem("AUTH_TOKEN", res.data.data);
      localStorage.setItem(
        "AUTH_ADMIN_USER",
        JSON.stringify(_.get(res, "data.admin_user"))
      );

      // and redirect to the admin.
      redirect("/admin");
    },
  };

  return useMutation(mutationOptions);
};

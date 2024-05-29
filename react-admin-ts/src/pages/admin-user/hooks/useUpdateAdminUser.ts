import {
  useMutation,
  MutationFunction,
  UseMutationResult,
} from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
import _ from "lodash";
import { useNavigate } from "react-router-dom";
import { AxiosInstance } from "axios";

interface AdminUserUpdateData {
  // Define the properties of the AdminUserUpdateData here
}

export const useUpdateAdminUser = (
  adminUserId: string
): UseMutationResult<void, unknown, AdminUserUpdateData, unknown> => {
  const client: AxiosInstance = useAxios();
  const redirect = useNavigate();

  const mutationFn: MutationFunction<void, AdminUserUpdateData> = async (
    data
  ) => {
    const url = "/admin-user/" + adminUserId;
    return await client.put(url, data, {
      headers: {
        "Content-Type": "multipart/form-data; boundary=----",
        Authorization: "Bearer " + localStorage.getItem("AUTH_TOKEN"),
      },
    });
  };

  return useMutation({
    mutationFn,
    onSuccess: (res) => {
      if (_.get(res, "data.status") === true) {
        redirect("/admin/admin-user");
      }
    },
  });
};

import {
  useMutation,
  MutationFunction,
  UseMutationResult,
} from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
import _ from "lodash";
import { useNavigate } from "react-router-dom";

interface Res {
  data: {
    status: boolean;
    // Define the other properties of the response here
  };
}

const mutationFn: MutationFunction<Res, FormData> = async (data) => {
  const client = useAxios();
  return await client.post("/admin-user", data, {
    headers: {
      "Content-Type": "multipart/form-data; boundary=----",
      Authorization: "Bearer " + localStorage.getItem("AUTH_TOKEN"),
    },
  });
};

const onSuccess = (res: Res) => {
  const redirect = useNavigate();
  if (_.get(res, "data.status") === true) {
    redirect("/admin/admin-user");
  }
};

export const useStoreAdminUser = (): UseMutationResult<
  Res,
  unknown,
  FormData,
  unknown
> => {
  return useMutation({
    mutationFn,
    onSuccess,
  });
};

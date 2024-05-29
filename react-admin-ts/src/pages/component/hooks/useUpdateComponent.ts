import {
  useMutation,
  MutationFunction,
  MutationOptions,
} from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
import _ from "lodash";
import { useNavigate } from "react-router-dom";

interface ComponentData {
  // Define the structure of the component data here
  [key: string]: any;
}

interface ResponseData {
  data: {
    status: boolean;
  };
}

export const useUpdateComponent = (component_id: string) => {
  const client = useAxios();
  const redirect = useNavigate();

  const mutationFn: MutationFunction<ResponseData, ComponentData> = async (
    data
  ) => {
    const url = "/component/" + component_id;
    const response = await client.put<ResponseData>(url, JSON.stringify(data));
    return response.data;
  };

  const mutationOptions: MutationOptions<ResponseData, unknown, ComponentData> =
    {
      mutationFn,
      onSuccess: (res) => {
        if (_.get(res, "data.status") === true) {
          redirect("/admin/component");
        }
      },
    };

  return useMutation(mutationOptions);
};

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

export const useStoreComponent = () => {
  const client = useAxios();
  const redirect = useNavigate();

  const mutationFn: MutationFunction<ResponseData, ComponentData> = async (
    data
  ) => {
    const response = await client.post<ResponseData>(
      "/component",
      JSON.stringify(data)
    );
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

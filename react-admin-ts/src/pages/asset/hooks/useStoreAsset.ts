import {
  useMutation,
  useQueryClient,
  MutationFunction,
  MutationOptions,
} from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
// import _ from 'lodash' // Removed unused import
// import { useNavigate } from "react-router-dom"; // Removed unused import

interface FormData {
  file: File;
}

interface ResponseData {
  data: {
    data: {
      data: any[];
    };
  };
}

export const useStoreAsset = () => {
  const client = useAxios();
  /* const redirect = useNavigate() */ // Removed unused variable
  const queryClient = useQueryClient();

  const mutationFn: MutationFunction<ResponseData, FormData> = async (data) => {
    const response = await client.post<ResponseData>("/asset", data, {
      headers: {
        "Content-Type": "multipart/form-data; boundary=----",
        Authorization: "Bearer " + localStorage.getItem("AUTH_TOKEN"),
      },
    });
    return response.data; // Return the actual data instead of the entire AxiosResponse object
  };

  const mutationOptions: MutationOptions<ResponseData, unknown, FormData> = {
    mutationFn,
    onSuccess: (_res) => {
      queryClient.invalidateQueries({ queryKey: ["asset-table"] });
    },
  };

  return useMutation(mutationOptions);
};

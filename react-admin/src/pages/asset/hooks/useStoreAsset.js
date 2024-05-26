import { useMutation, useQueryClient } from "@tanstack/react-query"; // Removed "useQuery" from the import
import { useAxios } from "../../../hooks/useAxios";
// import _ from 'lodash' // Removed unused import
// import { useNavigate } from "react-router-dom"; // Removed unused import

export const useStoreAsset = () => {
  const client = useAxios();
  /* const redirect = useNavigate() */ // Removed unused variable
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (data) => {
      return await client.post("/asset", data, {
        headers: {
          "Content-Type": "multipart/form-data; boundary=----",
          Authorization: "Bearer " + localStorage.getItem("AUTH_TOKEN"),
        },
      });
    },
    onSuccess: (res) => {
      queryClient.invalidateQueries({ queryKey: ["asset-table"] });
    },
  });
};

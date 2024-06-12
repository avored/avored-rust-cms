import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useAxios } from "../../../hooks/useAxios";
import IAssetSave from "../../../types/asset/IAssetSave";


export const useStoreAsset = () => {
  const client = useAxios();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: async (data: IAssetSave) => {
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

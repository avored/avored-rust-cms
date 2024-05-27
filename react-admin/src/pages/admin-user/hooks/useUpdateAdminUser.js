import { useMutation } from "@tanstack/react-query"; // Removed "useQuery"
import { useAxios } from "../../../hooks/useAxios";
import _ from "lodash";
import { useNavigate } from "react-router-dom";

export const useUpdateAdminUser = (adminUserId) => {
  const client = useAxios();
  const redirect = useNavigate();
  return useMutation({
    mutationFn: async (data) => {
      const url = "/admin-user/" + adminUserId;
      return await client.put(url, data, {
        headers: {
          "Content-Type": "multipart/form-data; boundary=----",
          Authorization: "Bearer " + localStorage.getItem("AUTH_TOKEN"),
        },
      });
    },
    onSuccess: (res) => {
      if (_.get(res, "data.status") === true) {
        redirect("/admin/admin-user");
      }
    },
  });
};

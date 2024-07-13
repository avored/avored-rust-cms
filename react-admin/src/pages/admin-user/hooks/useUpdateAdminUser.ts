import { useMutation } from "@tanstack/react-query"; // Removed "useQuery"
import { useAxios } from "../../../hooks/useAxios";
import _ from "lodash";
import { useNavigate } from "react-router-dom";
import IAdminUserUpdate from "../../../types/admin-user/IAdminUserUpdate";

export const useUpdateAdminUser = (adminUserId: string) => {
    const client = useAxios();
    const redirect = useNavigate();
    return useMutation({
        mutationFn: async (data: IAdminUserUpdate) => {
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

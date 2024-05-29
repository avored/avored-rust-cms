import { useMemo, useState } from "react";
import { Link, useParams } from "react-router-dom";
import InputField from "../../components/InputField";
import { Switch } from "@headlessui/react";
import AvoRedMultiSelectField from "../../components/AvoRedMultiSelectField";
import _ from "lodash";
import { useGetAdminUser } from "./hooks/useGetAdminUser";
import { useGetRoleOptions } from "./hooks/useGetRoleOptions";
import { useUpdateAdminUser } from "./hooks/useUpdateAdminUser";
import { useTranslation } from "react-i18next";

interface Params {
  admin_user_id: string;
  [key: string]: string | undefined;
}

const AdminUserEdit: React.FC = () => {
  const [selectedOption, setSelectedOption] = useState<string[]>([]);
  const [full_name, setFullName] = useState<string | undefined>();
  const [current_profile_image, setCurrentProfileImage] = useState<
    string | undefined
  >();
  const [is_super_admin, setIsSuperAdmin] = useState<boolean>(false);
  const [image, setImage] = useState<File | undefined>();
  const params = useParams<Params>();
  const [t] = useTranslation("global");

  const { data } = useGetAdminUser(params.admin_user_id || "");
  useMemo(() => {
    setFullName(_.get(data, "data.admin_user_model.full_name"));

    setIsSuperAdmin(_.get(data, "data.admin_user_model.is_super_admin", false));
    setCurrentProfileImage(_.get(data, "data.admin_user_model.profile_image"));
    const role_ids: string[] = [];
    _.get(data, "data.admin_user_model.roles", []).forEach(
      (role: { id: string }) => {
        role_ids.push(role.id);
      }
    );
    setSelectedOption(role_ids);
  }, [data]);

  const roleOptionResult = useGetRoleOptions();
  const { mutate } = useUpdateAdminUser(params.admin_user_id ?? "");

  const roles = _.get(roleOptionResult, "data.data.options", []);

  const handleProfileImageChange = (files: FileList | []) => {
    const file = files[0];
    setImage(file);
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    const formData = new FormData();

    if (full_name) {
      formData.append("full_name", full_name);
    }
    formData.append("is_super_admin", is_super_admin.toString());

    selectedOption.forEach((option) => {
      formData.append("role_ids[]", option);
    });

    if (image) {
      formData.append("image", image);
    }

    mutate(formData);
  };

  return (
    <div className="flex-1 bg-white">
      <div className="px-5 pl-64">
        <div className="w-full">
          <div className="block rounded-lg p-6">
            <h1 className="text-xl font-semibold mb-4 text-gray-900">
              {t("admin_user.information")}
            </h1>
            <form onSubmit={handleSubmit}>
              <div className="mb-4">
                <InputField
                  label={t("common.fullname")}
                  type="text"
                  name="full_name"
                  value={full_name || ""}
                  onChange={(e) => setFullName(e.target.value)}
                  autoFocus
                  errorMessages={[]}
                />
              </div>
              <div className="mb-4">
                <div className="relative z-10">
                  <AvoRedMultiSelectField
                    label="Roles"
                    options={roles}
                    selectedOption={selectedOption}
                    onChangeSelectedOption={setSelectedOption}
                  />
                </div>
              </div>
              <div className="mb-4 flex items-center">
                <label
                  htmlFor="is_super_admin_switch"
                  className="text-sm text-gray-600"
                >
                  Is Super Admin
                </label>
                <Switch
                  checked={is_super_admin}
                  onChange={setIsSuperAdmin}
                  id="is_super_admin_switch"
                  className={`${
                    is_super_admin ? "bg-primary-500" : "bg-gray-200"
                  } relative ml-5 inline-flex h-6 w-11 items-center rounded-full`}
                >
                  <span className="sr-only">Enable notifications</span>
                  <span
                    className={`${
                      is_super_admin ? "translate-x-6" : "translate-x-1"
                    } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                  />
                </Switch>
              </div>
              <div className="flex items-center mt-3">
                <div className="ring-1 ring-gray-300 rounded">
                  <div className="p-3">
                    <img
                      className="h-48 w-48 rounded"
                      alt={full_name || "Profile Image"}
                      src={`${current_profile_image}`}
                    />
                  </div>
                </div>
                <div className="ml-5">
                  <div className="mb-4">
                    <InputField
                      accept="image/*"
                      label="New Profile Photo"
                      type="file"
                      name="image"
                      onChange={(e) =>
                        handleProfileImageChange(e.target.files || [])
                      }
                    />
                  </div>
                </div>
              </div>
              <div className="flex items-center mt-5">
                <button
                  type="submit"
                  className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                >
                  {t("common.save")}
                </button>
                <Link
                  to="/admin/admin-user"
                  className="ml-auto font-medium text-gray-600 hover:text-gray-500"
                >
                  {t("common.cancel")}
                </Link>
              </div>
            </form>
          </div>
        </div>
      </div>
    </div>
  );
};

export default AdminUserEdit;

import { useState } from "react";
import { Link } from "react-router-dom";
import { Switch } from "@headlessui/react";
import _ from "lodash";
import { useStoreRole } from "./hooks/useStoreRole";
import { useTranslation } from "react-i18next";

interface RoleData {
  name: string;
  identifier: string;
  permissions: string[];
  description: string; // Add the missing 'description' property
}

function RoleCreate() {
  const [name, setName] = useState<string>("Editor");
  const [identifier, setIdentifier] = useState<string>("editor");
  const [permissions, setPermissions] = useState<string[]>([]);
  const { mutate } = useStoreRole();
  const [t] = useTranslation("global");

  const switchOnChange = (e: boolean, key: string) => {
    if (e) {
      setPermissions((perm) => [...perm, key]);
    } else {
      const array = [...permissions]; // make a separate copy of the array
      const newArray = array.filter((ele) => ele !== key);
      setPermissions(newArray);
    }
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const data: RoleData = { name, identifier, permissions, description: "" }; // Provide a default value for the 'description' property
    mutate(data);
  };

  function permissionAllowed(_arg0: string): boolean | undefined {
    throw new Error("Function not implemented.");
  }

  return (
    <div className="flex-1 bg-white">
      <div className="px-5 pl-64 ">
        <div className="w-full">
          <div className="block rounded-lg p-6">
            <h1 className="text-xl font-semibold mb-4 text-gray-900">
              {t("roles.information")}
            </h1>
            {/*<p className="text-gray-600 dark:text-gray-300 mb-6">Use a permanent address where you can*/}
            {/*    receive mail.</p>*/}
            <form onSubmit={handleSubmit}>
              <div className="mb-4">
                <input
                  type="text"
                  placeholder="Name"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  className="border p-2 rounded w-full"
                />
              </div>
              <div className="mb-4">
                <input
                  type="text"
                  placeholder="Identifier"
                  value={identifier}
                  onChange={(e) => setIdentifier(e.target.value)}
                  className="border p-2 rounded w-full"
                />
              </div>

              <div className="mb-4 flex">
                <div className="border w-1/3 border-gray-200 rounded">
                  <div className="p-3 font-semibold border-b">
                    {t("roles.generics")}
                  </div>
                  <div className="p-3">
                    <div className="mb-4 flex items-center">
                      <label
                        htmlFor="dashboard_permission"
                        className="text-sm text-gray-600"
                      >
                        {t("sidebar.dashboard")}
                      </label>
                      <Switch
                        checked={permissionAllowed("dashboard")}
                        onChange={(e) => switchOnChange(e, "dashboard")}
                        id="dashboard_permission"
                        className={`${
                          permissionAllowed("dashboard")
                            ? "bg-primary-500"
                            : "bg-gray-200"
                        } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                          className={`${
                            permissionAllowed("dashboard")
                              ? "translate-x-6"
                              : "translate-x-1"
                          } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>

                    <div className="mb-4 flex items-center">
                      <label
                        htmlFor="setting_permission"
                        className="text-sm text-gray-600"
                      >
                        {t("sidebar.setting")}
                      </label>
                      <Switch
                        checked={permissionAllowed("setting")}
                        onChange={(e) => switchOnChange(e, "setting")}
                        id="setting_permission"
                        className={`${
                          permissionAllowed("setting")
                            ? "bg-primary-500"
                            : "bg-gray-200"
                        } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                          className={`${
                            permissionAllowed("setting")
                              ? "translate-x-6"
                              : "translate-x-1"
                          } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>
                  </div>
                </div>

                <div className="border w-1/3 ml-3 border-gray-200 rounded">
                  <div className="p-3 font-semibold border-b">
                    {`${t("common.page")} ${t("common.permissions")}`}
                  </div>
                  <div className="p-3">
                    <div className="mb-4 flex items-center">
                      <label
                        htmlFor="page_table_permission"
                        className="text-sm text-gray-600"
                      >
                        {`${t("common.page")} ${t("common.table")}`}
                      </label>
                      <Switch
                        checked={permissionAllowed("page_table")}
                        onChange={(e) => switchOnChange(e, "page_table")}
                        id="page_table_permission"
                        className={`${
                          permissionAllowed("page_table")
                            ? "bg-primary-500"
                            : "bg-gray-200"
                        } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                          className={`${
                            permissionAllowed("page_table")
                              ? "translate-x-6"
                              : "translate-x-1"
                          } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>
                    <div className="mb-4 flex items-center">
                      <label
                        htmlFor="page_create_permission"
                        className="text-sm text-gray-600"
                      >
                        {`${t("common.page")} ${t("common.create")}`}
                      </label>
                      <Switch
                        checked={permissionAllowed("page_create")}
                        onChange={(e) => switchOnChange(e, "page_create")}
                        id="page_create_permission"
                        className={`${
                          permissionAllowed("page_create")
                            ? "bg-primary-500"
                            : "bg-gray-200"
                        } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                          className={`${
                            permissionAllowed("page_create")
                              ? "translate-x-6"
                              : "translate-x-1"
                          } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>
                    <div className="mb-4 flex items-center">
                      <label
                        htmlFor="page_edit_permission"
                        className="text-sm text-gray-600"
                      >
                        {`${t("common.page")} ${t("common.edit")}`}
                      </label>
                      <Switch
                        checked={permissionAllowed("page_edit")}
                        onChange={(e) => switchOnChange(e, "page_edit")}
                        id="page_edit_permission"
                        className={`${
                          permissionAllowed("page_edit")
                            ? "bg-primary-500"
                            : "bg-gray-200"
                        } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                          className={`${
                            permissionAllowed("page_edit")
                              ? "translate-x-6"
                              : "translate-x-1"
                          } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>
                    <div className="mb-4 flex items-center">
                      <label
                        htmlFor="page_delete_permission"
                        className="text-sm text-gray-600"
                      >
                        {`${t("common.page")} ${t("common.delete")}`}
                      </label>
                      <Switch
                        checked={permissionAllowed("page_delete")}
                        onChange={(e) => switchOnChange(e, "page_delete")}
                        id="page_delete_permission"
                        className={`${
                          permissionAllowed("page_delete")
                            ? "bg-primary-500"
                            : "bg-gray-200"
                        } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                          className={`${
                            permissionAllowed("page_delete")
                              ? "translate-x-6"
                              : "translate-x-1"
                          } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>
                  </div>
                </div>

                <div className="border w-1/3 ml-3 border-gray-200 rounded">
                  <div className="p-3 font-semibold border-b">
                    {`${t("common.component")} ${t("common.permissions")}`}
                  </div>
                  <div className="p-3">
                    <div className="mb-4 flex items-center">
                      <label
                        htmlFor="component_table_permission"
                        className="text-sm text-gray-600"
                      >
                        {`${t("common.component")} ${t("common.table")}`}
                      </label>
                      <Switch
                        checked={permissionAllowed("component_table")}
                        onChange={(e) => switchOnChange(e, "component_table")}
                        id="component_table_permission"
                        className={`${
                          permissionAllowed("component_table")
                            ? "bg-primary-500"
                            : "bg-gray-200"
                        } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                          className={`${
                            permissionAllowed("component_table")
                              ? "translate-x-6"
                              : "translate-x-1"
                          } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>
                    <div className="mb-4 flex items-center">
                      <label
                        htmlFor="component_create_permission"
                        className="text-sm text-gray-600"
                      >
                        {`${t("common.component")} ${t("common.create")}`}
                      </label>
                      <Switch
                        checked={permissionAllowed("component_create")}
                        onChange={(e) => switchOnChange(e, "component_create")}
                        id="component_create_permission"
                        className={`${
                          permissionAllowed("component_create")
                            ? "bg-primary-500"
                            : "bg-gray-200"
                        } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                          className={`${
                            permissionAllowed("component_create")
                              ? "translate-x-6"
                              : "translate-x-1"
                          } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>
                    <div className="mb-4 flex items-center">
                      <label
                        htmlFor="component_edit_permission"
                        className="text-sm text-gray-600"
                      >
                        {`${t("common.component")} ${t("common.edit")}`}
                      </label>
                      <Switch
                        checked={permissionAllowed("component_edit")}
                        onChange={(e) => switchOnChange(e, "component_edit")}
                        id="component_edit_permission"
                        className={`${
                          permissionAllowed("component_edit")
                            ? "bg-primary-500"
                            : "bg-gray-200"
                        } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                          className={`${
                            permissionAllowed("component_edit")
                              ? "translate-x-6"
                              : "translate-x-1"
                          } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>
                    <div className="mb-4 flex items-center">
                      <label
                        htmlFor="component_delete_permission"
                        className="text-sm text-gray-600"
                      >
                        {`${t("common.component")} ${t("common.delete")}`}
                      </label>
                      <Switch
                        checked={permissionAllowed("component_delete")}
                        onChange={(e) => switchOnChange(e, "component_delete")}
                        id="component_delete_permission"
                        className={`${
                          permissionAllowed("component_delete")
                            ? "bg-primary-500"
                            : "bg-gray-200"
                        } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                          className={`${
                            permissionAllowed("component_delete")
                              ? "translate-x-6"
                              : "translate-x-1"
                          } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>
                  </div>
                </div>
              </div>

              <div className="mb-4 flex">
                <div className="border w-1/3 border-gray-200 rounded">
                  <div className="p-3 font-semibold border-b">{`${t(
                    "common.asset"
                  )} ${t("common.permissions")}`}</div>
                  <div className="p-3">
                    <div className="mb-4 flex items-center">
                      <label
                        htmlFor="asset_table_permission"
                        className="text-sm text-gray-600"
                      >
                        {`${t("common.asset")} ${t("common.table")}`}
                      </label>
                      <Switch
                        checked={permissionAllowed("asset_table")}
                        onChange={(e) => switchOnChange(e, "asset_table")}
                        id="asset_table_permission"
                        className={`${
                          permissionAllowed("asset_table")
                            ? "bg-primary-500"
                            : "bg-gray-200"
                        } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                          className={`${
                            permissionAllowed("asset_table")
                              ? "translate-x-6"
                              : "translate-x-1"
                          } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>
                    <div className="mb-4 flex items-center">
                      <label
                        htmlFor="asset_create_permission"
                        className="text-sm text-gray-600"
                      >
                        {`${t("common.asset")} ${t("common.create")}`}
                      </label>
                      <Switch
                        checked={permissionAllowed("asset_create")}
                        onChange={(e) => switchOnChange(e, "asset_create")}
                        id="asset_create_permission"
                        className={`${
                          permissionAllowed("asset_create")
                            ? "bg-primary-500"
                            : "bg-gray-200"
                        } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                          className={`${
                            permissionAllowed("asset_create")
                              ? "translate-x-6"
                              : "translate-x-1"
                          } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>

                    <div className="mb-4 flex items-center">
                      <label
                        htmlFor="asset_edit_permission"
                        className="text-sm text-gray-600"
                      >
                        {`${t("common.asset")} ${t("common.edit")}`}
                      </label>
                      <Switch
                        checked={permissionAllowed("asset_edit")}
                        onChange={(e) => switchOnChange(e, "asset_edit")}
                        id="asset_edit_permission"
                        className={`${
                          permissionAllowed("asset_edit")
                            ? "bg-primary-500"
                            : "bg-gray-200"
                        } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                          className={`${
                            permissionAllowed("asset_edit")
                              ? "translate-x-6"
                              : "translate-x-1"
                          } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>

                    <div className="mb-4 flex items-center">
                      <label
                        htmlFor="asset_delete_permission"
                        className="text-sm text-gray-600"
                      >
                        {`${t("common.asset")} ${t("common.delete")}`}
                      </label>
                      <Switch
                        checked={permissionAllowed("asset_delete")}
                        onChange={(e) => switchOnChange(e, "asset_delete")}
                        id="asset_delete_permission"
                        className={`${
                          permissionAllowed("asset_delete")
                            ? "bg-primary-500"
                            : "bg-gray-200"
                        } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                          className={`${
                            permissionAllowed("asset_delete")
                              ? "translate-x-6"
                              : "translate-x-1"
                          } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>
                  </div>
                </div>

                <div className="border w-1/3 ml-3 border-gray-200 rounded">
                  <div className="p-3 font-semibold border-b">
                    {`${t("common.admin_user")} ${t("common.permissions")}`}
                  </div>

                  <div className="p-3">
                    <div className="mb-4 flex items-center">
                      <label
                        htmlFor="admin_user_table_permission"
                        className="text-sm text-gray-600"
                      >
                        {`${t("common.admin_user")} ${t("common.table")}`}
                      </label>
                      <Switch
                        checked={permissionAllowed("admin_user_table")}
                        onChange={(e) => switchOnChange(e, "admin_user_table")}
                        id="admin_user_table_permission"
                        className={`${
                          permissionAllowed("admin_user_table")
                            ? "bg-primary-500"
                            : "bg-gray-200"
                        } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                          className={`${
                            permissionAllowed("admin_user_table")
                              ? "translate-x-6"
                              : "translate-x-1"
                          } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>
                    <div className="mb-4 flex items-center">
                      <label
                        htmlFor="admin_user_create_permission"
                        className="text-sm text-gray-600"
                      >
                        {`${t("common.admin_user")} ${t("common.create")}`}
                      </label>
                      <Switch
                        checked={permissionAllowed("admin_user_create")}
                        onChange={(e) => switchOnChange(e, "admin_user_create")}
                        id="admin_user_create_permission"
                        className={`${
                          permissionAllowed("admin_user_create")
                            ? "bg-primary-500"
                            : "bg-gray-200"
                        } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                          className={`${
                            permissionAllowed("admin_user_create")
                              ? "translate-x-6"
                              : "translate-x-1"
                          } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>

                    <div className="mb-4 flex items-center">
                      <label
                        htmlFor="admin_user_edit_permission"
                        className="text-sm text-gray-600"
                      >
                        {`${t("common.admin_user")} ${t("common.edit")}`}
                      </label>
                      <Switch
                        checked={permissionAllowed("admin_user_edit")}
                        onChange={(e) => switchOnChange(e, "admin_user_edit")}
                        id="admin_user_edit_permission"
                        className={`${
                          permissionAllowed("admin_user_edit")
                            ? "bg-primary-500"
                            : "bg-gray-200"
                        } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                          className={`${
                            permissionAllowed("admin_user_edit")
                              ? "translate-x-6"
                              : "translate-x-1"
                          } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>

                    <div className="mb-4 flex items-center">
                      <label
                        htmlFor="admin_user_delete_permission"
                        className="text-sm text-gray-600"
                      >
                        {`${t("common.admin_user")} ${t("common.delete")}`}
                      </label>
                      <Switch
                        checked={permissionAllowed("admin_user_delete")}
                        onChange={(e) => switchOnChange(e, "admin_user_delete")}
                        id="admin_user_delete_permission"
                        className={`${
                          permissionAllowed("admin_user_delete")
                            ? "bg-primary-500"
                            : "bg-gray-200"
                        } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                          className={`${
                            permissionAllowed("admin_user_delete")
                              ? "translate-x-6"
                              : "translate-x-1"
                          } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>
                  </div>
                </div>

                <div className="border w-1/3 ml-3 border-gray-200 rounded">
                  <div className="p-3 font-semibold border-b">{`${t(
                    "roles.roles"
                  )} ${t("common.permissions")}`}</div>

                  <div className="p-3">
                    <div className="mb-4 flex items-center">
                      <label
                        htmlFor="role_table_permission"
                        className="text-sm text-gray-600"
                      >
                        {`${t("common.role")} ${t("common.table")}`}
                      </label>
                      <Switch
                        checked={permissionAllowed("role_table")}
                        onChange={(e) => switchOnChange(e, "role_table")}
                        id="role_table_permission"
                        className={`${
                          permissionAllowed("role_table")
                            ? "bg-primary-500"
                            : "bg-gray-200"
                        } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                          className={`${
                            permissionAllowed("role_table")
                              ? "translate-x-6"
                              : "translate-x-1"
                          } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>
                    <div className="mb-4 flex items-center">
                      <label
                        htmlFor="role_create_permission"
                        className="text-sm text-gray-600"
                      >
                        {`${t("common.role")} ${t("common.create")}`}
                      </label>
                      <Switch
                        checked={permissionAllowed("role_create")}
                        onChange={(e) => switchOnChange(e, "role_create")}
                        id="role_create_permission"
                        className={`${
                          permissionAllowed("role_create")
                            ? "bg-primary-500"
                            : "bg-gray-200"
                        } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                          className={`${
                            permissionAllowed("role_create")
                              ? "translate-x-6"
                              : "translate-x-1"
                          } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>

                    <div className="mb-4 flex items-center">
                      <label
                        htmlFor="role_edit_permission"
                        className="text-sm text-gray-600"
                      >
                        {`${t("common.role")} ${t("common.edit")}`}
                      </label>
                      <Switch
                        checked={permissionAllowed("role_edit")}
                        onChange={(e) => switchOnChange(e, "role_edit")}
                        id="role_edit_permission"
                        className={`${
                          permissionAllowed("role_edit")
                            ? "bg-primary-500"
                            : "bg-gray-200"
                        } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                          className={`${
                            permissionAllowed("role_edit")
                              ? "translate-x-6"
                              : "translate-x-1"
                          } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>

                    <div className="mb-4 flex items-center">
                      <label
                        htmlFor="role_delete_permission"
                        className="text-sm text-gray-600"
                      >
                        {`${t("common.role")} ${t("common.delete")}`}
                      </label>
                      <Switch
                        checked={permissionAllowed("role_delete")}
                        onChange={(e) => switchOnChange(e, "role_delete")}
                        id="role_delete_permission"
                        className={`${
                          permissionAllowed("role_delete")
                            ? "bg-primary-500"
                            : "bg-gray-200"
                        } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                          className={`${
                            permissionAllowed("role_delete")
                              ? "translate-x-6"
                              : "translate-x-1"
                          } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>
                  </div>
                </div>
              </div>

              <div className="flex items-center">
                <button
                  type="submit"
                  className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                >
                  {t("common.save")}
                </button>
                <Link
                  to="/admin/page"
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
}

export default RoleCreate;

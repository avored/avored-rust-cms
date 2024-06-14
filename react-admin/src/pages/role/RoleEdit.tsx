import {Link, useParams} from "react-router-dom"
import {Switch} from "@headlessui/react"
import {useTranslation} from "react-i18next";
import {Controller, useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import InputField from "../../components/InputField";
import IEditableRole from "../../types/role/IEditableRole";
import {RoleEditSchema} from "./schemas/role.edit.schema";
import {useUpdateRole} from "./hooks/useUpdateRole";
import {useGetRole} from "./hooks/useGetRole";

function RoleEdit() {
  const params = useParams();
  const role_id = params.role_id ?? ''
  const { mutate } = useUpdateRole(role_id);
  const [t] = useTranslation("global")
  const {data} = useGetRole(role_id)
  const values = data?.data.role_model


  const {
    control,
    register,
    handleSubmit,
    formState: {errors},
    setValue,
    getValues
  } = useForm<IEditableRole>({
    resolver: joiResolver(RoleEditSchema, {allowUnknown: true}),
    values
  });

  const switchOnChange = ((key: string) => {
    let permissions = getValues('permissions') ?? [];
    let index = permissions.findIndex(ele => ele === key)
    if (index < 0) {
      permissions.push(key)
    } else {
      permissions.splice(index, 1)
    }
    setValue("permissions", permissions)
  })

  const permissionAllowed = ((key: string) => {
    let permissions = getValues('permissions') ?? [];
    let index = permissions.findIndex(ele => ele === key)

    return (index >= 0)
  })

  const submitHandler = ((data: IEditableRole) => {
    mutate(data)
  })

  const renderSwitch = ((switchKey: string) => {
    return (
        <Controller
            control={control}
            name="permissions"
            render={({field}) => {
              return (
                  <>
                    <div className="mb-4 flex items-center">
                      <label
                          htmlFor={switchKey}
                          className="text-sm text-gray-600"
                      >
                        {t(`pages.role.permissions.${switchKey}`)}
                      </label>
                      <Switch
                          checked={permissionAllowed('setting')}
                          onChange={(e) => switchOnChange(switchKey)}
                          id="dashboard_permission"
                          className={`${
                              permissionAllowed(switchKey)
                                  ? "bg-primary-500"
                                  : "bg-gray-200"
                          } relative ml-auto inline-flex h-6 w-11 items-center rounded-full`}
                      >
                        <span className="sr-only">Enable notifications</span>
                        <span
                            className={`${
                                permissionAllowed(switchKey)
                                    ? "translate-x-6"
                                    : "translate-x-1"
                            } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                        />
                      </Switch>
                    </div>
                  </>
              )
            }}
        />
    )
  })

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
              <form onSubmit={handleSubmit(submitHandler)}>
                <div className="mb-4">
                  <InputField
                      label={t('common.name')}
                      placeholder={t('common.name')}
                      name="name"
                      register={register("name")}
                      autoFocus={true}
                  />
                </div>
                <div className="mb-4">
                  <InputField
                      label={t('common.identifier')}
                      placeholder={t('common.identifier')}
                      name="identifier"
                      register={register("identifier")}
                  />
                </div>

                <div className="mb-4 flex">
                  <div className="border w-1/3 border-gray-200 rounded">
                    <div className="p-3 font-semibold border-b">
                      {t("roles.generics")}
                    </div>
                    <div className="p-3">
                      {renderSwitch('dashboard')}

                      {renderSwitch('setting')}
                    </div>
                  </div>

                  <div className="border w-1/3 ml-3 border-gray-200 rounded">
                    <div className="p-3 font-semibold border-b">
                      {`${t("common.page")} ${t("common.permissions")}`}
                    </div>
                    <div className="p-3">
                      {renderSwitch('page_table')}
                      {renderSwitch('page_create')}
                      {renderSwitch('page_edit')}
                      {renderSwitch('page_delete')}
                    </div>
                  </div>

                  <div className="border w-1/3 ml-3 border-gray-200 rounded">
                    <div className="p-3 font-semibold border-b">
                      {`${t("common.component")} ${t("common.permissions")}`}
                    </div>
                    <div className="p-3">
                      {renderSwitch('component_table')}
                      {renderSwitch('component_create')}
                      {renderSwitch('component_edit')}
                      {renderSwitch('component_delete')}
                    </div>
                  </div>
                </div>

                <div className="mb-4 flex">
                  <div className="border w-1/3 border-gray-200 rounded">
                    <div className="p-3 font-semibold border-b">{`${t(
                        "common.asset"
                    )} ${t("common.permissions")}`}</div>
                    <div className="p-3">
                      {renderSwitch('asset_table')}
                      {renderSwitch('asset_create')}
                      {renderSwitch('asset_edit')}
                      {renderSwitch('asset_delete')}
                    </div>
                  </div>

                  <div className="border w-1/3 ml-3 border-gray-200 rounded">
                    <div className="p-3 font-semibold border-b">
                      {`${t("common.admin_user")} ${t("common.permissions")}`}
                    </div>

                    <div className="p-3">
                      {renderSwitch('admin_user_table')}
                      {renderSwitch('admin_user_create')}
                      {renderSwitch('admin_user_edit')}
                      {renderSwitch('admin_user_delete')}
                    </div>
                  </div>

                  <div className="border w-1/3 ml-3 border-gray-200 rounded">
                    <div className="p-3 font-semibold border-b">{`${t(
                        "roles.roles"
                    )} ${t("common.permissions")}`}</div>

                    <div className="p-3">
                      {renderSwitch('role_table')}
                      {renderSwitch('role_create')}
                      {renderSwitch('role_edit')}
                      {renderSwitch('role_delete')}
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
                      to="/admin/role"
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

export default RoleEdit

import InputField from "../../components/InputField";
import ErrorMessage from "../../components/ErrorMessage";
import { useTranslation } from "react-i18next";
import {Controller, useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import {Switch} from "@headlessui/react";
import {CreatableRoleType} from "../../types/admin_user/AdminUserType";
import {UseRoleCreateSchema} from "../../schemas/admin_user/UseRoleCreateSchema";
import { Link } from "react-router-dom";
import {UseStoreRoleHook} from "../../hooks/admin_user/UseStoreRoleHook";
import slug from "slug";
import { StoreRoleRequest } from "grpc-avored/admin_user_pb";

export const RoleCreatePage = () => {
    const [t] = useTranslation("global")
    const {mutate, error} = UseStoreRoleHook()

    const {
        control,
        register,
        handleSubmit,
        formState: {errors},
        setValue,
        getValues
    } = useForm<CreatableRoleType>({
        resolver: joiResolver(UseRoleCreateSchema(), {allowUnknown: true}),
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

    const submitHandler = ((data: CreatableRoleType) => {
        const store_role = new StoreRoleRequest();
        store_role.setName(data.name);
        store_role.setIdentifier(data.identifier);
        store_role.setPermissionsList(data.permissions);

        mutate(store_role)
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
                                    htmlFor="dashboard_permission"
                                    className="text-sm text-gray-600"
                                >
                                    {t(switchKey)}
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

    const onNameChange = (e: React.KeyboardEvent<HTMLInputElement>) => {
        setValue('identifier', slug(e.currentTarget.value || ''))
    }

    return(
        <>
            <div className="px-5">
                <div className="w-full">
                    <div className="block rounded-lg p-6">
                        <h1 className="text-xl font-semibold mb-4 text-gray-900">
                            {t("role_information")}
                        </h1>

                        <form onSubmit={handleSubmit(submitHandler)}>
                            <div className="mb-4">
                                <InputField
                                    label={t("name")}
                                    placeholder={t("name")}
                                    name="name"
                                    onKeyUp={e => onNameChange(e)}
                                    register={register("name")}
                                    autoFocus={true}
                                />
                                <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="name" />
                            </div>
                            <div className="mb-4">
                                <InputField
                                    label={t("identifier")}
                                    placeholder={t("identifier")}
                                    name="identifier"
                                    register={register("identifier")}
                                />
                                <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="identifier" />
                            </div>

                            <div className="block">
                                <div className="mb-4 flex">
                                    <div className="border w-1/3 border-gray-200 rounded">
                                        <div className="p-3 font-semibold border-b">
                                            {t("generics")}
                                        </div>
                                        <div className="p-3">
                                            {renderSwitch('dashboard')}

                                            {renderSwitch('get_setting')}
                                            {renderSwitch('store_setting')}
                                        </div>
                                    </div>

                                    <div className="border w-1/3 ml-3 border-gray-200 rounded">
                                        <div className="p-3 font-semibold border-b">
                                            {t("content_permission")}
                                        </div>
                                        <div className="p-3">
                                            {renderSwitch('content_paginate')}
                                            {renderSwitch('store_content')}
                                            {renderSwitch('content_edit')}
                                            {renderSwitch('content_delete')}
                                            {renderSwitch('get_content')}
                                            {renderSwitch('collection_all')}
                                            {renderSwitch('get_collection')}
                                            {renderSwitch('store_collection')}
                                            {renderSwitch('update_collection')}
                                        </div>
                                    </div>

                                    <div className="border w-1/3 ml-3 border-gray-200 rounded">
                                        <div className="p-3 font-semibold border-b">
                                            {t("cms_permission")}
                                        </div>
                                        <div className="p-3">
                                            {renderSwitch('get_cms_content')}
                                        </div>
                                    </div>

                                </div>

                                <div className="mb-4 flex">
                                    <div className="border w-1/3 border-gray-200 rounded">
                                        <div className="p-3 font-semibold border-b">
                                            {t("asset_permission")}
                                        </div>
                                        <div className="p-3">
                                            {renderSwitch('paginate_asset')}
                                            {renderSwitch('asset_create')}
                                            {renderSwitch('rename_asset')}
                                            {renderSwitch('asset_delete')}
                                            {renderSwitch('create_folder')}
                                            {renderSwitch('delete_folder')}
                                        </div>
                                    </div>

                                    <div className="border w-1/3 ml-3 border-gray-200 rounded">
                                        <div className="p-3 font-semibold border-b">
                                            {t("admin_user_permission")}
                                        </div>

                                        <div className="p-3">
                                            {renderSwitch('paginate_admin_user')}
                                            {renderSwitch('admin_user_create')}
                                            {renderSwitch('admin_user_edit')}
                                            {renderSwitch('admin_user_delete')}
                                            {renderSwitch('get_admin_user')}
                                        </div>
                                    </div>

                                    <div className="border w-1/3 ml-3 border-gray-200 rounded">
                                        <div className="p-3 font-semibold border-b">
                                            {t("role_permission")}
                                        </div>

                                        <div className="p-3">
                                            {renderSwitch('role_paginate')}
                                            {renderSwitch('store_role')}
                                            {renderSwitch('update_role')}
                                            {renderSwitch('role_option')}
                                            {renderSwitch('get_role')}
                                            {renderSwitch('put_role_identifier')}
                                        </div>
                                    </div>
                                </div>
                            </div>

                            <div className="flex items-center">
                                <button
                                    type="submit"
                                    className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                                >
                                    {t("save")}
                                </button>
                                <Link
                                    to="/admin/role"
                                    className="ml-auto font-medium text-gray-600 hover:text-gray-500"
                                >
                                    {t("cancel")}
                                </Link>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </>
    )
}
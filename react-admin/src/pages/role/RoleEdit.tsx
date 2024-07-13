import {Link, useParams} from "react-router-dom"
import {Switch} from "@headlessui/react"
import {useTranslation} from "react-i18next";
import {Controller, useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import IEditableRole from "../../types/role/IEditableRole";
import {useRoleEditSchema} from "./schemas/role.edit.schema";
import {useUpdateRole} from "./hooks/useUpdateRole";
import {useGetRole} from "./hooks/useGetRole";
import {RoleFields} from "./RoleFields";
import InputField from "../../components/InputField";
import {useState} from "react";
import {PutRoleIdentifierType} from "../../types/role/PutRoleIdentifierType";
import {useRolePutSchema} from "./schemas/role.put.schema";
import {usePutRoleIdentifier} from "./hooks/usePutRoleIdentifier";

function RoleEdit() {
    const params = useParams();
    const role_id = params.role_id ?? ''
    const { mutate } = useUpdateRole(role_id);
    const [t] = useTranslation("global")
    const {data} = useGetRole(role_id)
    const [isEditableIdentifier, setIsEditableIdentifier] = useState<boolean>(true)
    const values = data?.data.role_model


    const {
        register: putRoleRegister,
        getValues: getRoleIdentifierValue
    } = useForm<PutRoleIdentifierType>({
        resolver: joiResolver(useRolePutSchema(), {allowUnknown: true}),
        values: {
            identifier: data?.data.role_model.identifier
        }
    });

    const {
        control,
        register,
        handleSubmit,
        formState: {errors},
        setValue,
        getValues
    } = useForm<IEditableRole>({
        resolver: joiResolver(useRoleEditSchema(), {allowUnknown: true}),
        values
    })

    const {mutate: putRoleIdentifierMutate} = usePutRoleIdentifier(role_id)


    const editableIdentifierOnClick = (() => {
        setIsEditableIdentifier(false)
    })
    const saveIdentifierOnClick = (() => {
        putRoleIdentifierMutate({identifier: getRoleIdentifierValue('identifier')})
        setIsEditableIdentifier(true)
    })

    const cancelIdentifierOnClick = (() => {
        setIsEditableIdentifier(true)
    })


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
        console.log(data)
        // mutate(data)
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
                            {t("role_information")}
                        </h1>

                        <form onSubmit={handleSubmit(submitHandler)}>
                            <div className="mb-4">
                                <InputField
                                    label={t("name")}
                                    placeholder={t("name")}
                                    name="name"
                                    register={register("name")}
                                    autoFocus={true}
                                />
                            </div>
                            <div className="mb-4">
                                <InputField
                                    label={t("identifier")}
                                    placeholder={t("identifier")}
                                    name="identifier"
                                    register={putRoleRegister("identifier")}
                                    disabled={isEditableIdentifier}
                                />
                                <div
                                    className="mt-2"
                                >
                                    {isEditableIdentifier ? (
                                        <>
                                            <span onClick={editableIdentifierOnClick}
                                                  className="text-xs text-blue-600 cursor-pointer">
                                                {t("edit_identifier")}
                                            </span>
                                        </>
                                    ) : (
                                        <>
                                            <button type="button" onClick={saveIdentifierOnClick}
                                                    className="text-xs text-blue-600 cursor-pointer">
                                                {t('save')}
                                            </button>
                                            <button type="button" onClick={cancelIdentifierOnClick}
                                                    className="ml-3 text-xs text-blue-600 cursor-pointer">
                                                {t('cancel')}
                                            </button>
                                        </>
                                    )}
                                </div>
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
                                            {renderSwitch('save_setting')}
                                        </div>
                                    </div>

                                    <div className="border w-1/3 ml-3 border-gray-200 rounded">
                                        <div className="p-3 font-semibold border-b">
                                            {t("page_permission")}
                                        </div>
                                        <div className="p-3">
                                            {renderSwitch('page_table')}
                                            {renderSwitch('page_create')}
                                            {renderSwitch('page_edit')}
                                            {renderSwitch('page_delete')}
                                            {renderSwitch('get_page')}
                                        </div>
                                    </div>

                                    <div className="border w-1/3 ml-3 border-gray-200 rounded">
                                        <div className="p-3 font-semibold border-b">
                                            {t("component_permission")}
                                        </div>
                                        <div className="p-3">
                                            {renderSwitch('component_table')}
                                            {renderSwitch('component_create')}
                                            {renderSwitch('component_edit')}
                                            {renderSwitch('component_delete')}
                                            {renderSwitch('get_component')}
                                        </div>
                                    </div>
                                </div>

                                <div className="mb-4 flex">
                                    <div className="border w-1/3 border-gray-200 rounded">
                                        <div className="p-3 font-semibold border-b">
                                            {t("asset_permission")}
                                        </div>
                                        <div className="p-3">
                                            {renderSwitch('asset_table')}
                                            {renderSwitch('asset_create')}
                                            {renderSwitch('asset_edit')}
                                            {renderSwitch('asset_delete')}
                                        </div>
                                    </div>

                                    <div className="border w-1/3 ml-3 border-gray-200 rounded">
                                        <div className="p-3 font-semibold border-b">
                                            {t("admin_user_permission")}
                                        </div>

                                        <div className="p-3">
                                            {renderSwitch('admin_user_table')}
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
                                            {renderSwitch('role_table')}
                                            {renderSwitch('role_create')}
                                            {renderSwitch('role_edit')}
                                            {renderSwitch('role_delete')}
                                            {renderSwitch('get_role')}
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
        </div>
    );
}

export default RoleEdit

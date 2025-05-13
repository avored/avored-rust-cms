import {Link, useParams} from "react-router-dom";
import {useTranslation} from "react-i18next";
import {useState} from "react";
import {Controller, useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import InputField from "../../components/InputField";
import {Switch} from "@headlessui/react";
import {UseRoleEditSchema} from "../../schemas/admin_user/UserRoleEditSchema";
import {UseGetRoleHook} from "../../hooks/admin_user/UseGetRoleHook";
import {GetRoleRequest, PutRoleIdentifierRequest, UpdateRoleRequest} from "../../grpc_generated/admin_user_pb";
import {EditRoleType, PutRoleIdentifierType} from "../../types/admin_user/AdminUserType";
import {UseRolePutSchema} from "../../schemas/admin_user/UsePutRoleSchema";
import {UseUpdateRoleHook} from "../../hooks/admin_user/UseUpdateRoleHook";
import {UsePutRoleIdentifierHook} from "../../hooks/admin_user/UsePutRoleIdentifierHook";
import ErrorMessage from "../../components/ErrorMessage";

export const RoleEditPage = () => {
    const params = useParams();
    const [isEditableIdentifier, setIsEditableIdentifier] = useState<boolean>(true)
    const role_id = params.role_id ?? ''
    const { mutate } = UseUpdateRoleHook();
    const [t] = useTranslation("global")

    const req = new GetRoleRequest();
    req.setRoleId(params.role_id ?? '');

    const {data, error} = UseGetRoleHook(req)


    const values: EditRoleType = data?.data as unknown as EditRoleType;
    const role_permissions_list = data?.data?.permissionsList ?? [];
    if (values) {
        values.permissions = role_permissions_list as Array<unknown> as string[];
    }



    const {
        register: putRoleRegister,
        getValues: getRoleIdentifierValue
    } = useForm<PutRoleIdentifierType>({
        resolver: joiResolver(UseRolePutSchema(), {allowUnknown: true}),
        values: {
            identifier: data?.data?.identifier ?? ''
        }
    });

    const {
        control,
        register,
        handleSubmit,
        formState: {errors},
        setValue,
        getValues
    } = useForm<EditRoleType>({
        resolver: joiResolver(UseRoleEditSchema(), {allowUnknown: true}),
        values
    })

    const {mutate: putRoleIdentifierMutate} = UsePutRoleIdentifierHook()


    const editableIdentifierOnClick = (() => {
        setIsEditableIdentifier(false)
    })
    const saveIdentifierOnClick = (() => {
        const putRoleIdentifier = new PutRoleIdentifierRequest();
        putRoleIdentifier.setRoleId(role_id);
        const val = getRoleIdentifierValue('identifier') ?? '';
        putRoleIdentifier.setIdentifier(val.toString())

        putRoleIdentifierMutate(putRoleIdentifier)

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

    const submitHandler = ((data: EditRoleType) => {
        const update_role = new UpdateRoleRequest();
        update_role.setRoleId(data.id);
        update_role.setName(data.name);
        update_role.setPermissionsList(data.permissions);
        mutate(update_role)
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
                                    {t(switchKey)}
                                </label>
                                <Switch
                                    checked={permissionAllowed('setting')}
                                    onChange={(e) => switchOnChange(switchKey)}
                                    id={switchKey}
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
                                            {t("content_permission")}
                                        </div>
                                        <div className="p-3">
                                            {renderSwitch('content_table')}
                                            {renderSwitch('content_create')}
                                            {renderSwitch('content_edit')}
                                            {renderSwitch('content_delete')}
                                            {renderSwitch('get_content')}
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
                                            {renderSwitch('asset_table')}
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
        </>
    )
}

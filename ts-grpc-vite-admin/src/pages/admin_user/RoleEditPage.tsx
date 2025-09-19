import {Link, useParams} from "react-router-dom";
import {useTranslation} from "react-i18next";
import {useState} from "react";
import {Controller, useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import InputField from "../../components/InputField";
import {Switch} from "@headlessui/react";
import {UseRoleEditSchema} from "../../schemas/admin_user/UserRoleEditSchema";
import {UseGetRoleHook} from "../../hooks/admin_user/UseGetRoleHook";
import {DeleteRoleRequest, GetRoleRequest, PutRoleIdentifierRequest, UpdateRoleRequest} from "grpc-avored/admin_user_pb";
import {EditRoleType, PutRoleIdentifierType} from "../../types/admin_user/AdminUserType";
import {UseRolePutSchema} from "../../schemas/admin_user/UsePutRoleSchema";
import {UseUpdateRoleHook} from "../../hooks/admin_user/UseUpdateRoleHook";
import {UsePutRoleIdentifierHook} from "../../hooks/admin_user/UsePutRoleIdentifierHook";
import ErrorMessage from "../../components/ErrorMessage";
import { UseDeleteRoleHook } from "../../hooks/admin_user/UseDeleteRoleHook";
import { ButtonType } from "../../components/AvoRedButton";
import { ExclamationTriangleIcon } from "@heroicons/react/24/solid";
import AvoredModal from "../../components/AvoredModal";

export const RoleEditPage = () => {
    const params = useParams();
    const [isEditableIdentifier, setIsEditableIdentifier] = useState<boolean>(true)
    const [isDeleteConfirmationModalOpen, setIiDeleteConfirmationModalOpen] = useState<boolean>(false)
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
    const {mutate: deleteRoleMutate} = UseDeleteRoleHook()


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
                render={() => {
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
                                    onChange={() => switchOnChange(switchKey)}
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

    const deleteButtonOnClick = (() => {
        setIiDeleteConfirmationModalOpen(true)
    })

    const confirmOnDelete = ((e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
        e.preventDefault()
        const request = new DeleteRoleRequest()
        request.setRoleId(role_id)

        
        deleteRoleMutate(request)        
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


                            <AvoredModal
                                isOpen={isDeleteConfirmationModalOpen}
                                
                                closeModal={() => setIiDeleteConfirmationModalOpen(false)}
                                modal_header=""
                                modal_body={
                                    <div>
                                        <div className="">
                                            <div className="p-6 pt-0 text-center">
                                                <ExclamationTriangleIcon className="w-20 h-20 text-red-600 mx-auto" />
                                                <h3 className="text-xl font-normal text-gray-500 mt-5 mb-6">
                                                    Are you sure you want to delete this role?
                                                </h3>
                                                <button type="button"
                                                    onClick={e => confirmOnDelete(e)} 
                                                    className="text-white bg-red-600 hover:bg-red-800 focus:ring-4 focus:ring-red-300 font-medium rounded-lg text-base inline-flex items-center px-3 py-2.5 text-center mr-2">
                                                    Yes, I'm sure
                                                </button>
                                                <button type="button"
                                                    onClick={e => { e.preventDefault(); setIiDeleteConfirmationModalOpen(false)} } 
                                                    className="text-gray-900 bg-white hover:bg-gray-100 focus:ring-4 focus:ring-cyan-200 border border-gray-200 font-medium inline-flex items-center rounded-lg text-base px-3 py-2.5 text-center"
                                                    data-modal-toggle="delete-user-modal">
                                                    No, cancel
                                                </button>
                                            </div>
    
                                        </div>
                                    </div>
                                }
                            />


                            <div className="flex items-center">
                                <button
                                    type="submit"
                                    className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                                >
                                    {t("save")}
                                </button>
                                <Link
                                    to="/admin/role"
                                    className="ml-5 font-medium text-gray-600 hover:text-gray-500"
                                >
                                    {t("cancel")}
                                </Link>

                                <button
                                    onClick={deleteButtonOnClick}
                                    type={ButtonType.button}
                                    className="ml-auto bg-red-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
                                >
                                    {t("delete")}
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </>
    )
}

import {Link, useParams} from "react-router-dom";
import {useTranslation} from "react-i18next";
import {UseGetAdminUserHook} from "../../hooks/admin_user/UseGetAdminUserHook";
import {EditAdminUserType, RoleOptionType, RoleType} from "../../types/admin_user/AdminUserType";
import InputField from "../../components/InputField";
import {UseAdminUserEditSchema} from "../../schemas/admin_user/UseAdminUserEditSchema";
import {joiResolver} from "@hookform/resolvers/joi";
import {Controller, useForm} from "react-hook-form";
import {DeleteAdminUserRequest, GetAdminUserRequest, RoleOptionRequest, UpdateAdminUserRequest} from "../../grpc_generated/admin_user_pb";
import ErrorMessage from "../../components/ErrorMessage";
import {UseUpdateAdminUserHook} from "../../hooks/admin_user/UseUpdateAdminUserHook";
import _ from "lodash";
import {Switch} from "@headlessui/react";
import AvoRedMultiSelectField from "../../components/AvoRedMultiSelectField";
import {useState} from "react";
import {UseGetRoleOptionHook} from "../../hooks/admin_user/UseGetRoleOption";
import { UseDeleteAdminUserHook } from "../../hooks/admin_user/UseDeleteAdminUserHook";
import AvoredModal from "../../components/AvoredModal";
import { ExclamationTriangleIcon } from "@heroicons/react/24/solid";
import { ButtonType } from "../../components/AvoRedButton";

export const AdminUserEditPage = () => {
    const [selectedOption, setSelectedOption] = useState<Array<string>>([]);
    const [isDeleteConfirmationModalOpen, setIiDeleteConfirmationModalOpen] = useState<boolean>(false)
    const params = useParams();
    const [t] = useTranslation("global")
    const req = new GetAdminUserRequest();
    req.setAdminUserId(params.admin_user_id ?? '');

    const role_option_request = new RoleOptionRequest();
    const role_option_response = UseGetRoleOptionHook(role_option_request);
    const role_option_data_list = role_option_response?.data?.dataList ?? [];
    const roles: Array<RoleOptionType> = role_option_data_list as Array<unknown> as RoleOptionType[];


    const {data} = UseGetAdminUserHook(req);

    const values: EditAdminUserType = data?.data as unknown as EditAdminUserType;
    const admin_user_role_list = data?.data?.rolesList ?? [];
    if (values) {
        values.roles = admin_user_role_list as Array<unknown> as RoleType[];
        values.roles = _.uniqBy(values.roles, 'id')

        values.roles.map((role) => {
            if (!_.includes(selectedOption, role.id)) {
                selectedOption.push(role.id)
            }

            return role;
        })
    }

    const {
        register,
        control,
        trigger,
        handleSubmit,
        formState: {errors},
        setValue
    } = useForm<EditAdminUserType>({
        resolver: joiResolver(UseAdminUserEditSchema(), {allowUnknown: true}),
        values
    })
    const {mutate, error} = UseUpdateAdminUserHook();
    const { mutate: deleteAdminUserMutate } = UseDeleteAdminUserHook()


    const isSuperAdminSwitchOnChange = (async (is_checked: boolean) => {
        if (is_checked) {
            setSelectedOption([])
        }

        setValue("isSuperAdmin", is_checked)
        await trigger('isSuperAdmin')
    })

    const deleteButtonOnClick = (() => {
        setIiDeleteConfirmationModalOpen(true)
    })

    const confirmOnDelete = ((e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
        e.preventDefault()
        const request = new DeleteAdminUserRequest()
        request.setAdminUserId(params.admin_user_id ?? '')


        deleteAdminUserMutate(request)
    })

    const submitHandler = async (data: EditAdminUserType) => {
        const update_admin_user = new UpdateAdminUserRequest();
        update_admin_user.setFullName(data.fullName);
        update_admin_user.setAdminUserId(params.admin_user_id ?? '');
        update_admin_user.setRoleIdsList(selectedOption);
        update_admin_user.setIsSuperAdmin(data.isSuperAdmin)

        var profile_image_file_name = ""
        const file: File = data.profile_image[0];

        if (file) {
            profile_image_file_name = _.get(data, "profile_image.0.name", "user_profile_image_name.jpg");

            const reader = new FileReader()

            reader.onloadend = () => {
                const content = reader.result as ArrayBuffer;

                const bytesData = new Uint8Array(content)
                update_admin_user.setProfileImageContent(bytesData)
                update_admin_user.setProfileImageFileName(profile_image_file_name)

                mutate(update_admin_user)
            }
            reader.readAsArrayBuffer(file)
        } else {

            mutate(update_admin_user)
        }

    };

    return (
        <>
            <div className="px-5 pl-64">
                <div className="w-full">
                    <div className="block rounded-lg p-6">
                        <h1 className="text-xl font-semibold mb-4 text-gray-900">
                            {t("admin_user_information")}
                        </h1>
                        <form onSubmit={handleSubmit(submitHandler)}>
                            <div className="mb-4">
                                <InputField
                                    label={t("full_name")}
                                    type="text"
                                    name="full_name"
                                    register={register("fullName")}
                                    autoFocus
                                />
                                <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="fullName" />
                            </div>
                            <div className="mb-4">
                                <InputField
                                    label={t("email")}
                                    type="text"
                                    disabled={true}
                                    register={register("email")}
                                />
                            </div>

                            <Controller
                                control={control}
                                name="isSuperAdmin"
                                render={({field}) => {
                                    return (
                                        <>
                                            <div className="mb-4 flex items-center">
                                                <label
                                                    htmlFor="is_super_admin_switch"
                                                    className="text-sm text-gray-600"
                                                >
                                                    {t("is_super_admin")}
                                                </label>

                                                <Switch
                                                    onChange={e => isSuperAdminSwitchOnChange(e)}
                                                    checked={field.value}
                                                    id="is_super_admin_switch"
                                                    className={`${
                                                        field.value ? "bg-primary-500" : "bg-gray-200"
                                                    } relative ml-5 inline-flex h-6 w-11 items-center rounded-full`}
                                                >
                                                    <span className="sr-only">Enable notifications</span>
                                                    <span
                                                        className={`${
                                                            field.value ? "translate-x-6" : "translate-x-1"
                                                        } inline-block h-4 w-4 transform rounded-full bg-white transition`}
                                                    />
                                                </Switch>
                                            </div>
                                            {(!field.value) ?
                                                <div className="mb-4">
                                                    <div className="relative z-10">
                                                        <AvoRedMultiSelectField
                                                            label={t("roles")}
                                                            options={roles}
                                                            selectedOption={selectedOption}
                                                            onChangeSelectedOption={setSelectedOption}
                                                        ></AvoRedMultiSelectField>
                                                    </div>
                                                </div>
                                                : <></>}

                                        </>
                                    )
                                }}
                            />




                            <div className="flex items-center mt-3">
                                <div className="ring-1 ring-gray-300 rounded">
                                    <div className="p-3">
                                        <img
                                            className="h-48 w-48 rounded"
                                            alt={values?.fullName}
                                            src={`${values?.profileImage}`}
                                        />
                                    </div>
                                </div>
                                <div className="ml-5">
                                    <div className="mb-4">
                                        <InputField
                                            accept="image/*"
                                            label={t("new_profile_photo")}
                                            type="file"
                                            name="profile_image"
                                            register={register("profile_image")}
                                        />
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
                                                    onClick={e => { e.preventDefault(); setIiDeleteConfirmationModalOpen(false) }}
                                                    className="text-gray-900 bg-white hover:bg-gray-100 focus:ring-4 focus:ring-cyan-200 border border-gray-200 font-medium inline-flex items-center rounded-lg text-base px-3 py-2.5 text-center"
                                                    data-modal-toggle="delete-user-modal">
                                                    No, cancel
                                                </button>
                                            </div>

                                        </div>
                                    </div>
                                }
                            />


                            <div className="mt-5  flex items-center">
                                <button
                                    type="submit"
                                    className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                                >
                                    {t("save")}
                                </button>
                                <Link
                                    to="/admin/admin-user"
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

import React, {useState} from "react";
import {Link, useParams} from "react-router-dom";
import InputField from "../../components/InputField";
import {Switch} from "@headlessui/react";
import AvoRedMultiSelectField from "../../components/AvoRedMultiSelectField";
import _ from "lodash";
import {useGetRoleOptions} from "./hooks/useGetRoleOptions";
import {useTranslation} from "react-i18next";
import {Controller, useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import IAdminUserCreate from "../../types/admin-user/IAdminUserCreate";
import {useStoreAdminUser} from "./hooks/useStoreAdminUser";
import {useAdminUserCreateSchema} from "./schemas/admin.user.create.schema";
import ErrorMessage from "../../components/ErrorMessage";

function AdminUserCreate() {
    const [selectedOption, setSelectedOption] = useState([]);
    const [t] = useTranslation("global")


    const {
        control,
        trigger,
        register,
        handleSubmit,
        formState: {errors},
        setValue
    } = useForm<IAdminUserCreate>({
        resolver: joiResolver(useAdminUserCreateSchema(), {allowUnknown: true})
    })

    const roleOptionResult = useGetRoleOptions();
    const {mutate, error} = useStoreAdminUser()
    const roles = _.get(roleOptionResult, "data.data.options", []);

    const isSuperAdminSwitchOnChange = ((e: boolean) => {
        if (!e) {
            setSelectedOption([])
        }
        setValue("is_super_admin", e)
        trigger('is_super_admin')
    })

    const submitHandler = async (data: IAdminUserCreate) => {
        data.role_ids = selectedOption;
        (typeof data.profile_image === 'object') ? data.image = data.profile_image[0]: delete data.image
        mutate(data);
    };

    return (
      <>
            <div className="px-5">
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
                                    register={register("full_name")}
                                    autoFocus
                                />
                                <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="full_name" />
                            </div>
                            <div className="mb-4">
                                <InputField
                                    label={t("email")}
                                    type="text"
                                    name="email"
                                    register={register("email")}
                                />
                                <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="email" />
                            </div>

                            <div className="mb-4">
                                <InputField
                                    label={t("password")}
                                    type="password"
                                    name="password"
                                    register={register("password")}
                                />
                                <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="password" />
                            </div>
                            <div className="mb-4">
                                <InputField
                                    label={t("confirmation_password")}
                                    type="password"
                                    name="confirmation_password"
                                    register={register("confirmation_password")}
                                />
                                <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="confirmation_password" />
                            </div>

                            <Controller
                                control={control}
                                name="is_super_admin"
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
                                <div className="mb-4">
                                    <InputField
                                        label="Profile Photo"
                                        accept="image/*"
                                        type="file"
                                        name="profile_image"
                                        register={register("profile_image")}
                                    />
                                </div>

                            </div>
                            <div className="flex items-center mt-5">
                                <button
                                    type="submit"
                                    className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                                >
                                    {t("save")}
                                </button>
                                <Link
                                    to="/admin/admin-user"
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
    );
}

export default AdminUserCreate;

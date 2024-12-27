import React, {useMemo, useState} from "react";
import {Link, useParams} from "react-router-dom";
import InputField from "../../components/InputField";
import {Switch} from "@headlessui/react";
import AvoRedMultiSelectField from "../../components/AvoRedMultiSelectField";
import _ from "lodash";
import {useGetAdminUser} from "./hooks/useGetAdminUser";
import {useGetRoleOptions} from "./hooks/useGetRoleOptions";
import {useUpdateAdminUser} from "./hooks/useUpdateAdminUser";
import {useTranslation} from "react-i18next";
import {Controller, useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import {useAdminUserEditSchema} from "./schemas/admin.user.edit.schema";
import IAdminUserUpdate from "../../types/admin-user/IAdminUserUpdate";

function AdminUserEdit() {
    const [selectedOption, setSelectedOption] = useState<Array<string>>([]);
    const params = useParams();
    const [t] = useTranslation("global")
    const {data} = useGetAdminUser(params.admin_user_id ?? '');
    const values: IAdminUserUpdate = data?.data.admin_user_model
    const {
        control,
        trigger,
        register,
        handleSubmit,
        formState: {errors},
        setValue
    } = useForm<IAdminUserUpdate>({
        resolver: joiResolver(useAdminUserEditSchema(), {allowUnknown: true}),
        values
    })
    const roleOptionResult = useGetRoleOptions();
    const {mutate} = useUpdateAdminUser(params.admin_user_id ?? '');
    const roles = _.get(roleOptionResult, "data.data.options", []);

    useMemo(() => {
        let role_ids: Array<string> = []
        _.get(values, 'roles', []).map((role) => role_ids.push(role.id))
        setSelectedOption(role_ids)
    }, [values])

    const isSuperAdminSwitchOnChange = ((is_checked: boolean) => {
        if (is_checked) {
            setSelectedOption([])
        }

        setValue("is_super_admin", is_checked)
        trigger('is_super_admin')
    })

    const submitHandler = async (data: IAdminUserUpdate) => {
        (typeof data.profile_image === 'object') ? data.image = data.profile_image[0]: delete data.image
        data.role_ids = selectedOption
        mutate(data);
    };

    return (
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
                                    register={register("full_name")}
                                    autoFocus
                                />
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
                                <div className="ring-1 ring-gray-300 rounded">
                                    <div className="p-3">
                                        <img
                                            className="h-48 w-48 rounded"
                                            alt={values?.full_name}
                                            src={`${values?.profile_image}`}
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
                            <div className="flex border-t border-1 pt-5 items-center mt-5">
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
    );
}

export default AdminUserEdit;

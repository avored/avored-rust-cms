import InputField from "../../components/InputField";
import {useTranslation} from "react-i18next";
import ErrorMessage from "../../components/ErrorMessage";
import {Link} from "react-router-dom";
import {useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import {UseAdminUserCreateSchema} from "../../schemas/admin_user/UseAdminUserCreateSchema";
import {UseStoreAdminUserHook} from "../../hooks/admin_user/UseStoreAdminUserHook";
import {CreateAdminUserType} from "../../types/admin_user/AdminUserType";
import _ from "lodash";
import { StoreAdminUserRequest } from "grpc-avored/admin_user_pb";

export const AdminUserCreatePage = () => {

    const [t] = useTranslation("global")
    const {mutate, error} = UseStoreAdminUserHook()

    const {
        register,
        handleSubmit,
        formState: {errors},
    } = useForm<CreateAdminUserType>({
        resolver: joiResolver(UseAdminUserCreateSchema(), {allowUnknown: true, abortEarly: false})
    })

    const submitHandler = async (data: CreateAdminUserType) => {

        const store_admin_user = new StoreAdminUserRequest();

        var profile_image_file_name = ""
        const file: File = data.profile_image[0];

        if (file) {
            profile_image_file_name = _.get(data, "profile_image.0.name", "user_profile_image_name.jpg");

            const reader = new FileReader()

            reader.onloadend = () => {
                const content = reader.result as ArrayBuffer;

                const bytesData = new Uint8Array(content)
                store_admin_user.setProfileImageContent(bytesData)


                store_admin_user.setFullName(data.full_name);
                store_admin_user.setEmail(data.email);
                store_admin_user.setPassword(data.password);
                store_admin_user.setConfirmPassword(data.confirmation_password)
                store_admin_user.setIsSuperAdmin(false)

                store_admin_user.setProfileImageFileName(profile_image_file_name)

                mutate(store_admin_user)
            }
            reader.readAsArrayBuffer(file)
        } else {
            store_admin_user.setFullName(data.full_name);
            store_admin_user.setEmail(data.email);
            store_admin_user.setPassword(data.password);
            store_admin_user.setConfirmPassword(data.confirmation_password)
            store_admin_user.setIsSuperAdmin(false)

            mutate(store_admin_user)
        }

    }

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

                            {/*<Controller*/}
                            {/*    control={control}*/}
                            {/*    name="is_super_admin"*/}
                            {/*    render={({field}) => {*/}
                            {/*        return (*/}
                            {/*            <>*/}
                            {/*                <div className="mb-4 flex items-center">*/}
                            {/*                    <label*/}
                            {/*                        htmlFor="is_super_admin_switch"*/}
                            {/*                        className="text-sm text-gray-600"*/}
                            {/*                    >*/}
                            {/*                        {t("is_super_admin")}*/}
                            {/*                    </label>*/}

                            {/*                    <Switch*/}
                            {/*                        onChange={e => isSuperAdminSwitchOnChange(e)}*/}
                            {/*                        checked={field.value}*/}
                            {/*                        id="is_super_admin_switch"*/}
                            {/*                        className={`${*/}
                            {/*                            field.value ? "bg-primary-500" : "bg-gray-200"*/}
                            {/*                        } relative ml-5 inline-flex h-6 w-11 items-center rounded-full`}*/}
                            {/*                    >*/}
                            {/*                        <span className="sr-only">Enable notifications</span>*/}
                            {/*                        <span*/}
                            {/*                            className={`${*/}
                            {/*                                field.value ? "translate-x-6" : "translate-x-1"*/}
                            {/*                            } inline-block h-4 w-4 transform rounded-full bg-white transition`}*/}
                            {/*                        />*/}
                            {/*                    </Switch>*/}
                            {/*                </div>*/}
                            {/*                {(!field.value) ?*/}
                            {/*                    <div className="mb-4">*/}
                            {/*                        <div className="relative z-10">*/}
                            {/*                            <AvoRedMultiSelectField*/}
                            {/*                                label={t("roles")}*/}
                            {/*                                options={roles}*/}
                            {/*                                selectedOption={selectedOption}*/}
                            {/*                                onChangeSelectedOption={setSelectedOption}*/}
                            {/*                            ></AvoRedMultiSelectField>*/}
                            {/*                        </div>*/}
                            {/*                    </div>*/}
                            {/*                    : <></>}*/}

                            {/*            </>*/}
                            {/*        )*/}
                            {/*    }}*/}
                            {/*/>*/}

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
    )
}

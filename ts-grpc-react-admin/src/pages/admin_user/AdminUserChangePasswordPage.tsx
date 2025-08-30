import _ from "lodash";
import { UseChangePasswordAdminUserSchema } from "../../schemas/admin_user/UseChangePasswordAdminUserSchema";
import { useTranslation } from "react-i18next";
import { ChangePasswordType } from "../../types/admin_user/AdminUserType";
import { useForm } from "react-hook-form";
import { joiResolver } from "@hookform/resolvers/joi";
import InputField from "../../components/InputField";
import ErrorMessage from "../../components/ErrorMessage";
import { Link } from "react-router-dom";
import { UseChangePasswordHook } from "../../hooks/admin_user/UseChangePasswordHook";
import { ChangeAdminUserPasswordRequest } from "../../grpc_generated/admin_user_pb";

export const AdminUserChangePasswordPage = () => {
    
    const [t] = useTranslation("global")
    const {
        register,
        handleSubmit,
        formState: {errors},
    } = useForm<ChangePasswordType>({
        resolver: joiResolver(UseChangePasswordAdminUserSchema(), {allowUnknown: true}),
    })
    const {mutate, error} = UseChangePasswordHook();

    const submitHandler = async (data: any) => {
        const request = new ChangeAdminUserPasswordRequest()
        request.setPassword(data.password)
        request.setNewPassword(data.new_password)
        request.setConfirmPassword(data.confirm_password)

        mutate(request);
    };

    return (
        <>
            <div className="px-5 pl-64">
                <div className="w-full">
                    <div className="block rounded-lg p-6">
                        <h1 className="text-xl font-semibold mb-4 text-gray-900">
                            {t("change_password")}
                        </h1>
                        <form onSubmit={handleSubmit(submitHandler)}>
                            <div className="mb-4">
                                <InputField
                                    label={t("password")}
                                    type="password"
                                    name="password"
                                    register={register("password")}
                                    autoFocus
                                />
                                <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="password" />
                            </div>
                            <div className="mb-4">
                                <InputField
                                    label={t("new_password")}
                                    type="password"
                                    register={register("new_password")}
                                />
                            </div>
                            <div className="mb-4">
                                <InputField
                                    label={t("confirm_password")}
                                    type="password"
                                    register={register("confirm_password")}
                                />
                            </div>

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
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </>
    )
}

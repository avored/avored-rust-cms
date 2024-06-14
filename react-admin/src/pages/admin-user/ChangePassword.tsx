import { useEffect } from "react"
import logo from "../../assets/logo_only.svg"
import { Link, useNavigate, useParams } from "react-router-dom"
import InputField from "../../components/InputField"
import { useForm } from 'react-hook-form'
import { joiResolver } from '@hookform/resolvers/joi'
import _ from 'lodash'
// import { ErrorMessage } from "../../components/ErrorMessage"
import { useTranslation } from "react-i18next"
import IChangePasswordPost from "../../types/auth/ChangePasswordPostType";
import IErrorMessage from "../../types/common/IError";
import { changePasswordSchema } from "./schemas/changePassword.schema"
import { useChangePassword } from "./hooks/useChangePassword"

function ChangePassword() {
    const { register, handleSubmit, formState: { errors } } = useForm<IChangePasswordPost>({
        resolver: joiResolver(changePasswordSchema)
    });
    const [t] = useTranslation("global");
    const { mutate, isPending, error } = useChangePassword();

    const isErrorExist = (key: string) => {
        console.log(errors)
        let message = _.get(errors, key + '.message');
        if (message) {
            return 1;
        }
        return _.findIndex(_.get(error, 'response.data.errors', []), ((err: IErrorMessage) => err.key === key))
    }

    const getErrorMessage = (key: string) => {
        let message = _.get(errors, key + '.message');
        if (message) {
            return message;
        }
        return _.get(error, "response.data.errors." + isErrorExist('email') + ".message")
    }
    const submitHandler = (data: IChangePasswordPost) => {
        console.log(errors)
        mutate(data);
    };

    return (
        <div className="flex-1 bg-white">
            <div className="px-5 pl-64">
                <div className="w-full">
                    <div className="block rounded-lg p-6">
                        <h1 className="text-xl font-semibold mb-4 text-gray-900">
                            {t("admin_user.information")}
                        </h1>
                        <form onSubmit={handleSubmit(submitHandler)} className="space-y-5">
                            <div>
                                <InputField
                                    label={t("Current password")}
                                    type="password"
                                    name="current_password"
                                    register={register("current_password")}
                                />
                                {/*{isErrorExist("current_password") >= 0 && (*/}
                                {/*    <ErrorMessage message={getErrorMessage("current_password")} />*/}
                                {/*)}*/}
                            </div>
                            <div>
                                <InputField
                                    label={t("new_password")}
                                    type="password"
                                    name="password"
                                    register={register("password")}
                                />
                                {/*{isErrorExist("password") >= 0 && (*/}
                                {/*    <ErrorMessage message={getErrorMessage("password")} />*/}
                                {/*)}*/}
                            </div>
                            <div>
                                <InputField
                                    label="Confirm Password"
                                    type="password"
                                    name="confirm_password"
                                    register={register("confirm_password")}
                                />
                                {/*{isErrorExist("confirm_password") >= 0 && (*/}
                                {/*    <ErrorMessage message={getErrorMessage("confirm_password")} />*/}
                                {/*)}*/}
                            </div>

                            <div className="flex items-center mt-5">
                                <button
                                    type="submit"
                                    className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                                >
                                    {isPending ? "Loading..." : t("Change password")}
                                </button>
                                <Link
                                    to="/admin/admin-user"
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

export default ChangePassword;

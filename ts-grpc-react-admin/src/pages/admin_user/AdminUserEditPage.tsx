import {Link, useParams} from "react-router-dom";
import {useTranslation} from "react-i18next";
import {UseGetAdminUserHook} from "../../hooks/admin_user/UseGetAdminUserHook";
import {AdminUserType, EditAdminUserType} from "../../types/admin_user/AdminUserType";
import InputField from "../../components/InputField";
import {UseAdminUserEditSchema} from "../../schemas/admin_user/UseAdminUserEditSchema";
import {joiResolver} from "@hookform/resolvers/joi";
import {useForm} from "react-hook-form";
import {GetAdminUserRequest, StoreAdminUserRequest, UpdateAdminUserRequest} from "../../grpc_generated/admin_user_pb";
import ErrorMessage from "../../components/ErrorMessage";
import {UseUpdateAdminUserHook} from "../../hooks/admin_user/UseUpdateAdminUserHook";
import _ from "lodash";

export const AdminUserEditPage = () => {
    // const [selectedOption, setSelectedOption] = useState<Array<string>>([]);
    const params = useParams();
    const [t] = useTranslation("global")
    const req = new GetAdminUserRequest();
    req.setAdminUserId(params.admin_user_id ?? '');
    const {data} = UseGetAdminUserHook(req);
    const values: EditAdminUserType = data?.data as unknown as EditAdminUserType;
    const {
        register,
        handleSubmit,
        formState: {errors},
    } = useForm<EditAdminUserType>({
        resolver: joiResolver(UseAdminUserEditSchema(), {allowUnknown: true}),
        values
    })
    const {mutate, error} = UseUpdateAdminUserHook();


    const isSuperAdminSwitchOnChange = ((is_checked: boolean) => {
        // if (is_checked) {
        //     setSelectedOption([])
        // }

        // setValue("is_super_admin", is_checked)
        // trigger('is_super_admin')
    })

    const submitHandler = async (data: EditAdminUserType) => {
        const update_admin_user = new UpdateAdminUserRequest();
        update_admin_user.setFullName(data.fullName);
        update_admin_user.setAdminUserId(params.admin_user_id ?? '');

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
                            <div className="flex border border-x-0 border-b-0 border-gray-200 pt-5 items-center mt-5">
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
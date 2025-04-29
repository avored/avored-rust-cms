import InputField from "../../components/InputField";
import {useTranslation} from "react-i18next";
import {joiResolver} from "@hookform/resolvers/joi";
import {useForm} from "react-hook-form";
import AvoRedButton, {ButtonType} from "../../components/AvoRedButton";
import {SaveSettingType, SettingType} from "../../types/setting/SettingType";
import {SettingSaveSchema} from "../../schemas/setting/SettingSaveSchema";
import {UseSettingHook} from "../../hooks/setting/UseSettingHook";
import {UseStoreSettingHook} from "../../hooks/setting/UseStoreSettingHook";
import {GetSettingRequest, SettingSaveModel, StoreSettingRequest} from "../../grpc_generated/setting_pb";

export const SettingPage = () => {
    const request = new GetSettingRequest();
    const setting_api_all_response = UseSettingHook(request)

    const settin_data_list = setting_api_all_response?.data?.dataList ?? [];
    const existing_settings: Array<SettingType> = settin_data_list as Array<unknown> as SettingType[];

    // console.log(existing_settings)

    const {mutate} = UseStoreSettingHook()
    const [t] = useTranslation("global")


    const values: SaveSettingType = {'settings': existing_settings}
    const {
        register,
        handleSubmit,
        setValue,
        formState: {errors}
    } = useForm<SaveSettingType>({
        resolver: joiResolver(SettingSaveSchema, {allowUnknown: true}),
        values
    });

    const getIdentifierIndex = ((identifier: string): number => {
        return existing_settings.findIndex((setting) => setting.identifier === identifier);
    })
    const submitHandler = ((data: SaveSettingType) => {
        const request = new StoreSettingRequest();
        const setting_models: Array<SettingSaveModel> = [];
        for (const setting of data.settings) {
            const setting_model = new SettingSaveModel();

            setting_model.setIdentifier(setting.identifier);
            setting_model.setValue(setting.value);
            setting_model.setId(setting.id);

            setting_models.push(setting_model);
        }
        // console.log(setting_models)
        request.setDataList(setting_models)
        mutate(request)
    })

    const generateTokenOnClick = ((e: React.MouseEvent<HTMLButtonElement>) => {
        e.preventDefault()
        e.stopPropagation()
        setValue(`settings.${getIdentifierIndex('auth_cms_token')}.value`, "randomstringgoeshere")
    })

    return (
        <>
            <div className="p-5">
                <div className="flex items-center">
                    <div className="p-5 text-2xl font-semibold text-primary-500">
                        {t("settings")}
                    </div>
                </div>

                <div className="grid grid-cols-12 pt-3">
                    <div className="col-span-2 sm:border-0 border-r px-3">
                        <ul>
                            <li
                                className="mt-5 bg-gray-200 rounded cursor-pointer
                                        border-l-2 border-transparent px-2 py-2 font-semibold
                                        transition hover:border-l-primary-600"
                            >
                                {t("general")}
                            </li>
                        </ul>
                    </div>

                    <div className="col-span-10 px-5 rounded py-5">
                        <form onSubmit={handleSubmit(submitHandler)}>
                            <div className="sm:items-center sm:justify-between">
                                <div className="mb-5">
                                    <InputField
                                        label={t("site_name")}
                                        register={register(`settings.${getIdentifierIndex('general_site_name')}.value`)}
                                        autoFocus
                                    />
                                </div>

                                <div className="flex items-end mb-5">

                                    <div className="flex-1">
                                        <InputField
                                            label={t("cms_frontend_auth_token")}
                                            register={register(`settings.${getIdentifierIndex('auth_cms_token')}.value`)}
                                            autoFocus
                                        />
                                    </div>
                                    <div className="ml-3 mr-auto">
                                        <AvoRedButton
                                            className="bg-primary-500"
                                            onClick={generateTokenOnClick}
                                            type={ButtonType.button}
                                            label={t("generate_token")}
                                        />
                                    </div>


                                </div>
                            </div>

                            <div className="flex">

                                <button
                                    type="submit"
                                    className="mt-5 rounded-lg bg-primary-600 px-4 py-2 text-white"
                                >
                                    {t("save")}
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </>
    );

}

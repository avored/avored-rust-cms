import InputField from "../../components/InputField";
import {Link, useParams} from "react-router-dom";
import {useTranslation} from "react-i18next";
import {useState} from "react";
import {useForm} from "react-hook-form";
import {PutModelIdentifierType} from "../../types/model/PutModelIdentifierType";
import {joiResolver} from "@hookform/resolvers/joi";
import IEditableModel from "../../types/model/IEditableModel";
import {useModelPutSchema} from "./schemas/ModelPutSchema";
import {useModelEditSchema} from "./schemas/ModelEditSchema";
import {usePutModelIdentifier} from "./hooks/usePutModelIdentifier";
import {useGetModel} from "./hooks/useGetModel";
import {useUpdateModel} from "./hooks/useUpdateModel";

export const ModelEditPage = (() => {
    const params = useParams();
    const model_id = params.model_id ?? ''
    const {mutate} = useUpdateModel(model_id);
    const [t] = useTranslation("global")
    const {data} = useGetModel(model_id)
    const [isEditableIdentifier, setIsEditableIdentifier] = useState<boolean>(true)
    const values = data?.data.model_model


    const {
        register: putModelRegister,
        getValues: getModelIdentifierValue
    } = useForm<PutModelIdentifierType>({
        resolver: joiResolver(useModelPutSchema(), {allowUnknown: true}),
        values: {
            identifier: data?.data.model_model.identifier
        }
    });

    const {
        register,
        handleSubmit,
        formState: {errors},
    } = useForm<IEditableModel>({
        resolver: joiResolver(useModelEditSchema(), {allowUnknown: true}),
        values
    })

    const {mutate: putModelIdentifierMutate} = usePutModelIdentifier(model_id)


    const editableIdentifierOnClick = (() => {
        setIsEditableIdentifier(false)
    })
    const saveIdentifierOnClick = (() => {
        putModelIdentifierMutate({identifier: getModelIdentifierValue('identifier')})
        setIsEditableIdentifier(true)
    })

    const cancelIdentifierOnClick = (() => {
        setIsEditableIdentifier(true)
    })

    const submitHandler = ((data: IEditableModel) => {
        mutate(data)
    })
    return (
        <>
            <div className="px-5">
                <div className="w-full">
                    <div className="block rounded-lg p-6">
                        <h1 className="text-xl font-semibold mb-4 text-gray-900">
                            {t("model_information")}
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
                            </div>
                            <div className="mb-4">
                                <InputField
                                    label={t("identifier")}
                                    placeholder={t("identifier")}
                                    name="identifier"
                                    register={putModelRegister("identifier")}
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

                            <div className="flex items-center">
                                <button
                                    type="submit"
                                    className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                                >
                                    {t("save")}
                                </button>
                                <Link
                                    to={`/admin/model`}
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
})
import InputField from "../../components/InputField";
import {Link, useParams} from "react-router-dom";
import {useTranslation} from "react-i18next";
import {useState} from "react";
import {useForm} from "react-hook-form";
import {PutCollectionIdentifierType} from "../../types/collection/PutCollectionIdentifierType";
import {joiResolver} from "@hookform/resolvers/joi";
import EditableCollectionType from "../../types/collection/EditableCollectionType";
import {useGetCollection} from "./hooks/useGetCollection";
import {useCollectionPutSchema} from "./schemas/CollectionPutSchema";
import {useCollectionEditSchema} from "./schemas/CollectionEditSchema";
import {usePutCollectionIdentifier} from "./hooks/usePutCollectionIdentifier";
import {useUpdateCollection} from "./hooks/useUpdateCollection";

export const CollectionEdit = (() => {
    const params = useParams();
    const collection_id = params.collection_id ?? ''
    const {mutate} = useUpdateCollection(collection_id);
    const [t] = useTranslation("global")
    const {data} = useGetCollection(collection_id)
    const [isEditableIdentifier, setIsEditableIdentifier] = useState<boolean>(true)
    const values = data?.data.data


    const {
        register: putCollectionRegister,
        getValues: getCollectionIdentifierValue
    } = useForm<PutCollectionIdentifierType>({
        resolver: joiResolver(useCollectionPutSchema(), {allowUnknown: true}),
        values: {
            identifier: data?.data.data.identifier
        }
    });

    const {
        register,
        handleSubmit,
        formState: {errors},
    } = useForm<EditableCollectionType>({
        resolver: joiResolver(useCollectionEditSchema(), {allowUnknown: true}),
        values
    })

    const {mutate: putCollectionIdentifierMutate} = usePutCollectionIdentifier(collection_id)


    const editableIdentifierOnClick = (() => {
        setIsEditableIdentifier(false)
    })
    const saveIdentifierOnClick = (() => {
        putCollectionIdentifierMutate({identifier: getCollectionIdentifierValue('identifier')})
        setIsEditableIdentifier(true)
        console.log("teststes")
    })

    const cancelIdentifierOnClick = (() => {
        setIsEditableIdentifier(true)
    })

    const submitHandler = ((data: EditableCollectionType) => {
        mutate(data)
    })
    return (
        <>
            <div className="px-5">
                <div className="w-full">
                    <div className="block rounded-lg p-6">
                        <h1 className="text-xl font-semibold mb-4 text-gray-900">
                            {t("collection_information")}
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
                                    register={putCollectionRegister("identifier")}
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
                                    to={`/admin/collections`}
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
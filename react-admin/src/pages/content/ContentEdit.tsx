import {useTranslation} from "react-i18next"
import {ContentSidebar} from "./ContentSidebar"
import {Link, useParams, useSearchParams} from "react-router-dom"
import InputField from "../../components/InputField"
import React, { useState } from "react"
import {
    ContentFieldDataType,
    ContentFieldFieldType,
    SaveContentFieldType,
    SaveContentType
} from "../../types/content/ContentType"
import {Controller, useFieldArray, useForm} from "react-hook-form"
import {joiResolver} from "@hookform/resolvers/joi"
import AvoRedButton, { ButtonType } from "../../components/AvoRedButton"
import _ from 'lodash'
import { ContentFieldModal } from "./ContentFieldModal"
import {Cog8ToothIcon, TrashIcon} from "@heroicons/react/24/solid"
import {useContentEditSchema} from "./schemas/useContentEditSchema"
import {useGetContent} from "./hooks/useGetContent";
import {useUpdateContent} from "./hooks/useUpdateContent";
import {usePutContentIdentifier} from "./hooks/usePutContentIdentifier";

export const ContentEdit = (() => {
    const [t] = useTranslation("global")
    const [isContentFieldModalOpen, setIsContentFieldModalOpen] = useState<boolean>(false);
    const [isEditableIdentifier, setIsEditableIdentifier] = useState<boolean>(true);
    const [currentIndex, setCurrentIndex] = useState<number>(0);
    const [searchParams] = useSearchParams()
    const collectionType: string = searchParams.get("type") as string
    const params = useParams()

    const {mutate, error} = useUpdateContent(params.content_id ?? "", collectionType)

    const { data } = useGetContent(params.content_id ?? "", collectionType)
    const values = data?.data.data

    const submitHandler = (async (data: SaveContentType) => {
        mutate(data)
    })

    const {
        register,
        handleSubmit,
        setValue,
        getValues,
        formState: {errors},
        control,
        trigger
    } = useForm<SaveContentType>({
        resolver: joiResolver(useContentEditSchema(), {allowUnknown: true}),
        values
    })

    const { fields, append, remove } = useFieldArray({
        control,
        name: "content_fields", //rename fields
    });

    const { mutate: putContentIdentifierMutate } = usePutContentIdentifier(
        params.content_id ?? "", collectionType
    );


    const editableIdentifierOnClick = () => {
        setIsEditableIdentifier(false);
    };
    const saveIdentifierOnClick = () => {
        putContentIdentifierMutate({
            identifier: getValues("identifier"),
        });
        setIsEditableIdentifier(true);
    };

    const cancelIdentifierOnClick = () => {
        setIsEditableIdentifier(true);
    };

    const deleteContentFieldOnClick = (e: any, index: number) => {
        e.preventDefault();
        remove(index);
        setCurrentIndex(0);
    };

    const addFieldButtonOnClick = (async (e: React.MouseEvent<HTMLButtonElement>, max_index: number) => {
        e.preventDefault()
        append({
            name: '',
            identifier: '',
            data_type: ContentFieldDataType.TEXT,
            field_type: ContentFieldFieldType.TEXT,
            field_content: {
                text_value: {
                    text_value: ""
                }
            }
        })
        await trigger("content_fields");
        setCurrentIndex(max_index);
        setIsContentFieldModalOpen(true)
    })

    const renderField = (field: SaveContentFieldType, index: number) => {
        switch (field.field_type) {
            case ContentFieldFieldType.TEXT:
                return (
                    <div className="mb-4">
                        <InputField
                            label={t("field_content")}
                            placeholder={t("field_content")}
                            register={register(`content_fields.${index}.field_content.text_value.text_value`)}
                        />
                    </div>
                );
        }

    }

    return (
        <div className="flex w-full">
            <div className="p-5 w-64 bg-gray-50 min-h-screen">
                <ContentSidebar />
            </div>
            <div className="p-5 flex-1">
                <form onSubmit={handleSubmit(submitHandler)}>
                    {_.size(fields) > 0 ? (
                        <ContentFieldModal
                            register={register}
                            currentIndex={currentIndex}
                            getValues={getValues}
                            setValue={setValue}
                            trigger={trigger}
                            setIsOpen={setIsContentFieldModalOpen}
                            isOpen={isContentFieldModalOpen}
                            collectionType={collectionType}
                        />
                    ) : (
                        <></>
                    )}


                    <div className="mb-4">
                        <InputField
                            placeholder={t("name")}
                            label={t("name")}
                            name="name"
                            register={register("name")}
                        />
                    </div>

                    <div className="mb-4">
                        <InputField
                            placeholder={t("identifier")}
                            name="identifier"
                            register={register("identifier")}
                            disabled={isEditableIdentifier}
                        />
                        <div className="mt-2">
                            {isEditableIdentifier ? (
                                <>
                          <span
                              onClick={editableIdentifierOnClick}
                              className="text-xs text-blue-600 cursor-pointer"
                          >
                            {t("edit_identifier")}
                          </span>
                                </>
                            ) : (
                                <>
                                    <button
                                        type="button"
                                        onClick={saveIdentifierOnClick}
                                        className="text-xs text-blue-600 cursor-pointer"
                                    >
                                        {t("save")}
                                    </button>
                                    <button
                                        type="button"
                                        onClick={cancelIdentifierOnClick}
                                        className="ml-3 text-xs text-blue-600 cursor-pointer"
                                    >
                                        {t("cancel")}
                                    </button>
                                </>
                            )}
                        </div>
                    </div>

                    {fields.map((field, index) => {
                        return (
                            <div
                                key={field.id}
                                className="hover:ring-1 ring-primary-300 rounded mb-5 flex mt-5 py-3 w-full"
                            >
                                <Controller
                                    name={`content_fields.${index}`}
                                    render={({field}) => {
                                        return (
                                            <>
                                                <div className="flex mt-3 w-full justify-center">
                                                    <div className="flex-1 p-3">
                                                        <div className="p-3 bg-gray-200 rounded">
                                                            <div
                                                                className="flex text-sm w-full border-gray-300 border-b py-2">
                                                                <div className="flex-1">
                                                                                <span>
                                                                                    {field.value.name}
                                                                                </span>
                                                                    <span
                                                                        className="ml-1 text-xs text-gray-500">
                                                                                    ({field.value.identifier})
                                                                                </span>
                                                                </div>
                                                                <div className="ml-auto flex items-center">
                                                                    <div>
                                                                        <button
                                                                            type="button"
                                                                            className="outline-none"
                                                                            onClick={() => setIsContentFieldModalOpen(true)}
                                                                        >
                                                                            <Cog8ToothIcon className="w-5 h-5"/>
                                                                        </button>
                                                                    </div>
                                                                    <div
                                                                        onClick={(e) =>
                                                                            deleteContentFieldOnClick(e, index)
                                                                        }
                                                                        className="ml-3"
                                                                    >
                                                                        <TrashIcon className="w-4 h-4"/>
                                                                    </div>
                                                                </div>
                                                            </div>

                                                            <InputField
                                                                type="hidden"
                                                                placeholder={t("data_type")}
                                                                register={register(
                                                                    `content_fields.${index}.data_type`,
                                                                )}
                                                            />
                                                            <InputField
                                                                type="hidden"
                                                                placeholder={t("field_type")}
                                                                register={register(
                                                                    `content_fields.${index}.field_type`,
                                                                )}
                                                            />
                                                            {renderField(field.value, index)}
                                                        </div>
                                                    </div>
                                                </div>
                                            </>
                                        )
                                    }}
                                    control={control}
                                />

                            </div>
                        )
                    })}

                    <div className="mb-4">
                        <AvoRedButton
                            label="Add"
                            onClick={(e: React.MouseEvent<HTMLButtonElement>) => addFieldButtonOnClick(e, fields.length)}
                            type={ButtonType.button}/>
                    </div>
                    <div className="flex items-center">
                        <button
                            type="submit"
                            className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                        >
                            {t("save")}
                        </button>
                        <Link
                            to={`/admin/content?type=${collectionType}`}
                            className="ml-auto font-medium text-gray-600 hover:text-gray-500"
                        >
                            {t("cancel")}
                        </Link>
                    </div>
                </form>
            </div>
        </div>

    )
})

import {useTranslation} from "react-i18next";
import {ContentSidebar} from "./ContentSidebar";
import {Link, useSearchParams} from "react-router-dom";
import InputField from "../../components/InputField";
import ErrorMessage from "../../components/ErrorMessage";
import React, {useState} from "react";
import {
    ContentFieldDataType,
    AvoRedContentFieldType,
    SaveContentFieldType,
    SaveContentType
} from "../../types/content/ContentType";
import slug from "slug";
import {Controller, useFieldArray, useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import {useContentCreateSchema} from "./schemas/useContentCreateSchema";
import {useStoreContent} from "./hooks/useStoreContent";
import AvoRedButton, {ButtonType} from "../../components/AvoRedButton";
import _ from 'lodash';
import {ContentFieldModal} from "./ContentFieldModal";
import {Cog8ToothIcon, TrashIcon} from "@heroicons/react/24/solid";
import {useGetCollectionByIdentifier} from "./hooks/useGetCollectionByIdentifier";
import {
    CollectionFieldDataType,
    CollectionFieldFieldType,
    SaveCollectionFieldType
} from "../../types/collection/CreatableCollectionType";

export const ContentCreate = (() => {
    const [t] = useTranslation("global")
    const [isContentFieldModalOpen, setIsContentFieldModalOpen] = useState<boolean>(false);
    const [currentIndex, setCurrentIndex] = useState<number>(0);
    const [searchParams] = useSearchParams()
    const collectionType: string = searchParams.get("type") as string
    const {mutate, error} = useStoreContent()


    const api_response_get_collection_by_identifier = useGetCollectionByIdentifier(collectionType)

    const convertToContentModal = (() => {
        var contentModal: SaveContentType = {
            name : '',
            identifier: '',
            content_type: collectionType,
            content_fields: []
        }

        _.get(api_response_get_collection_by_identifier, 'data.data.data.collection_fields', []).forEach((collection_field: SaveCollectionFieldType) => {

            var data_type: ContentFieldDataType
            switch (collection_field.data_type) {
                case CollectionFieldDataType.TEXT:
                    data_type = ContentFieldDataType.TEXT
                    break;
                default:
                    data_type = ContentFieldDataType.TEXT
            }

            var field_type: AvoRedContentFieldType = AvoRedContentFieldType.TEXT;
            switch (collection_field.field_type) {
                case CollectionFieldFieldType.TEXT:
                    field_type = AvoRedContentFieldType.TEXT
                    break;
                default:
                    field_type = AvoRedContentFieldType.TEXT
            }


            let content_field: SaveContentFieldType = {
                name: collection_field.name,
                identifier: collection_field.identifier,
                data_type,
                field_type,
                field_content: {
                    text_value: {
                        text_value: ""
                    }
                }
            }
            contentModal.content_fields.push(content_field)
        })

        return contentModal
    })

    const values = convertToContentModal()

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
        resolver: joiResolver(useContentCreateSchema(), {allowUnknown: true}),
        values
    })

    const { fields, append, remove } = useFieldArray({
            control,
            name: "content_fields", //rename fields
        });

    const deleteContentFieldOnClick = (e: any, index: number) => {
        e.preventDefault();
        remove(index);
        setCurrentIndex(0);
    };

    const onNameChange = (e: React.KeyboardEvent<HTMLInputElement>) => {
        setValue('identifier', slug(e.currentTarget.value || ''))
    }

    const addFieldButtonOnClick = (async (e: React.MouseEvent<HTMLButtonElement>, max_index: number) => {
        e.preventDefault()
        append({
            name: '',
            identifier: '',
            data_type: ContentFieldDataType.TEXT,
            field_type: AvoRedContentFieldType.TEXT,
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
            case AvoRedContentFieldType.TEXT:
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
                        <InputField type="hidden" register={register("content_type")} value={collectionType}  />
                        <InputField
                            label={t("name")}
                            placeholder={t("name")}
                            name="name"
                            register={register("name")}
                            onKeyUp={e => onNameChange(e)}
                            autoFocus={true}
                        />
                        <ErrorMessage
                            frontendErrors={errors}
                            backendErrors={error}
                            identifier="name"
                        />
                    </div>
                    <div className="mb-4">
                        <InputField
                            label={t("identifier")}
                            placeholder={t("identifier")}
                            name="identifier"
                            register={register("identifier")}
                        />
                        <ErrorMessage
                            frontendErrors={errors}
                            backendErrors={error}
                            identifier="identifier"
                        />
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

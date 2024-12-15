import React, { useState } from "react";
import { Link } from "react-router-dom";
import { Cog8ToothIcon, PlusIcon, TrashIcon } from "@heroicons/react/24/solid";
import InputField from "../../components/InputField";
import { useStorePage } from "./hooks/useStorePage";
import { useTranslation } from "react-i18next";
import { Controller, useFieldArray, useForm } from "react-hook-form";
import { joiResolver } from "@hookform/resolvers/joi";
import { usePageCreateSchema } from "./schemas/page.create.schema";
import ErrorMessage from "../../components/ErrorMessage";
import "easymde/dist/easymde.min.css";
import slug from "slug";
import {
    AvoRedPageFieldCheckboxFieldDataOptions,
    AvoRedPageFieldRadioFieldDataOptions, PageFieldContent, PageTextContent,
    SaveFieldType,
    SavePageType,
} from "../../types/page/CreatablePageType";
import {
    AvoRedPageDataType,
    AvoRedPageFieldType, AvoRedPageStatus,
} from "../../types/page/IPageModel";
import _ from "lodash";
import SimpleMdeReact from "react-simplemde-editor";
import { PageFieldModal } from "./PageFieldModal";
import {SingleImageModal} from "./SingleImageModal";
import IAssetModel from "../../types/asset/IAssetModel";
import AvoRedButton, {ButtonType} from "../../components/AvoRedButton";

function PageCreate() {
    const [t] = useTranslation("global");
    const [isOpen, setIsOpen] = useState<boolean>(false);
    const [currentIndex, setCurrentIndex] = useState<number>(0);
    const backend_url = import.meta.env.VITE_AVORED_BACKEND_BASE_URL;
    const [isSingleAssetModalOpen, setisSingleAssetModalOpen] = useState(false);

    const {
        control,
        register,
        handleSubmit,
        formState: { errors },
        setValue,
        getValues,
        trigger,
    } = useForm<SavePageType>({
        resolver: joiResolver(usePageCreateSchema(), { allowUnknown: true }),
    });

    const { fields, append, remove } = useFieldArray({
        control,
        name: "page_fields", //rename fields
    });

    const { mutate, error } = useStorePage();

    const addFieldOnClick = async (
        e: React.MouseEvent<HTMLElement>,
        max_index: number,
    ) => {
        e.preventDefault();
        e.stopPropagation();
        const empty_text_content: PageTextContent =  {
            text_value: ""
        };
        const empty_page_content: PageFieldContent = {
            text_value: empty_text_content
        };

        append({
            name: "",
            identifier: "",
            data_type: AvoRedPageDataType.TEXT,
            field_type: AvoRedPageFieldType.TEXT,
            field_content: empty_page_content,
        });
        await trigger("page_fields");
        setCurrentIndex(max_index);

        // @todo fix this one
        setIsOpen(true);
    };

    const deletePageFieldOnClick = (e: any, index: number) => {
        e.preventDefault();
        remove(index);
        setCurrentIndex(0);
    };

    const submitHandler = async (data: SavePageType) => {
        data.status = AvoRedPageStatus.Draft
        mutate(data);
    };

    const onNameChange = (e: React.KeyboardEvent<HTMLInputElement>) => {
        setValue("identifier", slug(e.currentTarget.value || ""));
    };

    const textEditorOnChange = (value: string, field_index: number) => {
        const text_content: PageTextContent = {
            text_value: value,
        };
        const page_content: PageFieldContent =  {
            text_value: text_content
        }
        setValue(`page_fields.${field_index}.field_content`, page_content);
    };

    const renderField = (field: SaveFieldType, index: number) => {
        switch (field.field_type) {
            case AvoRedPageFieldType.TEXTAREA:
                return (
                    <div className="mb-4">
                        <label className="text-sm text-gray-600">
                            {t!("field_content")}
                        </label>
                        <textarea
                            className="w-full rounded"
                            {...register(`page_fields.${index}.field_content.text_value.text_value`)}
                        ></textarea>
                    </div>
                );
            case AvoRedPageFieldType.TextEditor:
                return (
                    <div className="mb-4">
                        <label className="text-sm text-gray-600">
                            {t!("field_content")}
                        </label>
                        <div className="h-96">
                            <SimpleMdeReact
                                options={{
                                    minHeight: "300px",
                                }}
                                onChange={(contentValue) =>
                                    textEditorOnChange(contentValue, index)
                                }
                            />
                        </div>
                    </div>
                );
            case AvoRedPageFieldType.Radio:
                return (
                    <div className="mb-4">
                        <label className="text-sm text-gray-600">
                            {t!("field_content")}
                        </label>
                        {field.field_data?.radio_field_options?.map(
                            (option: AvoRedPageFieldRadioFieldDataOptions) => {
                                return (
                                    <div key={`avredo-radio-${option.value}`} className="w-full">
                                        <input
                                            id={`avored-radio-${option.value}`}
                                            type="radio"
                                            value={option.value}
                                            {...register(`page_fields.${index}.field_content.text_value.text_value`)}
                                            className="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                                        />
                                        <label
                                            htmlFor={`avored-radio-${option.value}`}
                                            className="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300"
                                        >
                                            {option.label}
                                        </label>
                                    </div>
                                );
                            },
                        )}
                    </div>
                );

                case AvoRedPageFieldType.Checkbox:
                    return (
                        <div className="mb-4">
                            <label className="text-sm text-gray-600">
                                {t!("field_content")}
                            </label>
                            {field.field_data?.checkbox_field_options?.map(
                                (option: AvoRedPageFieldCheckboxFieldDataOptions) => {
                                    return (
                                        <div key={`avredo-checkbox-${option.value}`} className="w-full">
                                            <input
                                                id={`avored-checkbox-${option.value}`}
                                                type="checkbox"
                                                value={option.value}
                                                {...register(`page_fields.${index}.field_content.array_value.array_value`)}
                                                className="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                                            />
                                            <label
                                                htmlFor={`avored-checkbox-${option.value}`}
                                                className="ms-2 text-sm  rounded font-medium text-gray-900 dark:text-gray-300"
                                            >
                                                {option.label}
                                            </label>
                                        </div>
                                    );
                                },
                            )}
                        </div>
                    );
            case AvoRedPageFieldType.SELECT:
                return (
                    <div className="mb-4">
                        <label className="text-sm text-gray-600">
                            {t!("field_content")}
                        </label>

                        <select
                            {...register(`page_fields.${index}.field_content.text_value.text_value`)}
                            className="w-full rounded border-0 ring-1 ring-primary-400 outline-none appearance-none"
                        >
                            {field.field_data?.select_field_options?.map((option) => {
                                return (
                                    <option key={option.value} value={option.value}>
                                        {option.label}
                                    </option>
                                );
                            })}
                        </select>
                    </div>
                );
            case AvoRedPageFieldType.TEXT:
                return (
                    <div className="mb-4">
                        <InputField
                            label={t("field_content")}
                            placeholder={t("field_content")}
                            register={register(`page_fields.${index}.field_content.text_value.text_value`)}
                        />
                    </div>
                );
            case AvoRedPageFieldType.SingleImage:
                return (
                    <div className="mb-4">
                        <label className="text-sm text-gray-600">
                            {t!("field_content")}
                        </label>
                        <div className="mt-2">
                            <div className="flex items-center">
                                <input
                                    type="hidden"
                                    {...register(
                                        `page_fields.${index}.field_content.text_value.text_value`,
                                    )}
                                />
                                <img
                                    src={`${backend_url}${getValues(`page_fields.${index}.field_content.text_value.text_value`)}`}
                                    className="rounded w-48 h-48"
                                    alt={`${getValues(`page_fields.${index}.name`)}`}
                                />
                                <div>
                                    <SingleImageModal
                                        isOpen={isSingleAssetModalOpen}
                                        onCloseModal={closeSingleAssetModal}
                                        selectedAsset={(asset: IAssetModel) => selectedAsset(index ,asset)}
                                    />
                                    <AvoRedButton
                                        onClick={singleImageButtonOnClick}
                                        type={ButtonType.button}
                                        label={t!("select_asset")}
                                        className="ml-3 bg-primary-600 hover:bg-primary-500  focus:ring-primary-500"
                                    ></AvoRedButton>
                                </div>

                            </div>
                        </div>
                    </div>
                );
            default:
                return (
                    <div className="mb-4">
                        <InputField
                            label={t("field_content")}
                            placeholder={t("field_content")}
                            register={register(`page_fields.${index}.field_content.text_value.text_value`)}
                        />
                    </div>
                );
        }
    };

    const selectedAsset = (index: number, selectedAsset: IAssetModel) => {
        setValue(`page_fields.${index}.field_content.text_value.text_value`, selectedAsset.new_path);
        closeSingleAssetModal();
    }
    const singleImageButtonOnClick = (e: React.MouseEvent<HTMLButtonElement>) => {
        e.preventDefault();
        e.stopPropagation();
        openSingleAssetModal()
        console.log("test")
    }
    const openSingleAssetModal = () => {
        setisSingleAssetModalOpen(true);
    };

    const closeSingleAssetModal = () => {
        setisSingleAssetModalOpen(false);
    };
    const saveAsPublishedOnClick = ((e: React.MouseEvent) => {
        e.preventDefault()
        e.stopPropagation()

        const data: SavePageType = {
            name: getValues("name"),
            identifier: getValues("identifier"),
            page_fields: getValues("page_fields"),
            status: AvoRedPageStatus.Published
        };
        mutate(data)
    })
    // template start here
    return (
        <div className="flex-1 bg-white">
            <form onSubmit={handleSubmit(submitHandler)}>
            <div className="px-5 pl-64 ">
                <div className="w-full">
                    <div className="block rounded-lg p-6">
                        <h1 className="text-xl font-semibold mb-4 text-gray-900">
                            {t("page_information")}
                        </h1>
                        <div className="flex w-full">
                            <div className="w-5/6 pr-3">

                                {_.size(fields) > 0 ? (
                                    <PageFieldModal
                                        register={register}
                                        currentIndex={currentIndex}
                                        getValues={getValues}
                                        setValue={setValue}
                                        trigger={trigger}
                                        setIsOpen={setIsOpen}
                                        isOpen={isOpen}
                                    />
                                ) : (
                                    <></>
                                )}

                                <div className="flex w-full">
                                    <div className="w-1/2">
                                        <div className="mb-4">
                                            <InputField
                                                autoFocus={true}
                                                label={t("name")}
                                                placeholder={t("name")}
                                                register={register("name")}
                                                onKeyUp={(e) => onNameChange(e)}
                                            />
                                            <ErrorMessage
                                                frontendErrors={errors}
                                                backendErrors={error}
                                                identifier="name"
                                            />
                                        </div>
                                    </div>
                                    <div className="w-1/2 ml-3">
                                        <div className="mb-4">
                                            <InputField
                                                label={t("identifier")}
                                                placeholder={t("identifier")}
                                                register={register("identifier")}
                                            />
                                            <ErrorMessage
                                                frontendErrors={errors}
                                                backendErrors={error}
                                                identifier="identifier"
                                            />
                                        </div>
                                    </div>
                                </div>
                                {/*}<!-- FIELD CARD -->*/}
                                {fields.map((field, index) => {
                                    return (
                                        <div
                                            key={field.id}
                                            className="hover:ring-1 ring-primary-300 rounded mb-5 flex mt-5 py-3 w-full"
                                        >
                                            <Controller
                                                name={`page_fields.${index}`}
                                                render={({field: page_field}) => {
                                                    return (
                                                        <>
                                                            <div className="flex mt-3 w-full justify-center">
                                                                <div className="flex-1 p-3">
                                                                    <div className="p-3 bg-gray-200 rounded">
                                                                        <div
                                                                            className="flex text-sm w-full border-gray-300 border-b py-2">
                                                                            <div className="flex-1">
                                                                                <span>{page_field.value.name}</span>
                                                                                <span
                                                                                    className="ml-1 text-xs text-gray-500">
                                            ({page_field.value.identifier})
                                          </span>
                                                                            </div>
                                                                            <div className="ml-auto flex items-center">
                                                                                <div>
                                                                                    <button
                                                                                        type="button"
                                                                                        className="outline-none"
                                                                                        onClick={() => setIsOpen(true)}
                                                                                    >
                                                                                        <Cog8ToothIcon className="w-5 h-5"/>
                                                                                    </button>
                                                                                </div>
                                                                                <div
                                                                                    onClick={(e) =>
                                                                                        deletePageFieldOnClick(e, index)
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
                                                                                `page_fields.${index}.data_type`,
                                                                            )}
                                                                        />
                                                                        <InputField
                                                                            type="hidden"
                                                                            placeholder={t("field_type")}
                                                                            register={register(
                                                                                `page_fields.${index}.field_type`,
                                                                            )}
                                                                        />
                                                                        {renderField(page_field.value, index)}
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        </>
                                                    );
                                                }}
                                                control={control}
                                            />
                                        </div>
                                    );
                                })}

                                <div className="mb-4 flex items-center justify-center ring-1 ring-gray-400 rounded p-5">
                                    <button
                                        type="button"
                                        className="flex"
                                        onClick={(e) => addFieldOnClick(e, fields.length)}
                                    >
                                        <PlusIcon className="text-primary-500 h-5 w-5">
                                        </PlusIcon>
                                        <span className="text-sm ml-1 text-primary-500">
                                            {t("add_field")}
                                        </span>
                                    </button>
                                </div>

                            </div>
                            <div className="w-1/6 px-2 ml-auto">
                                <div className="mt-7">
                                    <button type="submit"
                                            className="bg-gray-500 rounded px-4 py-2 text-white">
                                        {t("save_as_draft")}
                                    </button>
                                </div>
                                <div className="mt-5">
                                    <button type="button"
                                        onClick={e => saveAsPublishedOnClick(e)}
                                        className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500">
                                        {t("published")}
                                    </button>
                                </div>
                                <div className="mt-5">
                                    <Link
                                        to={`/admin/page`}
                                        className="mt-5 font-medium text-gray-600 hover:text-gray-500"
                                    >
                                        {t("cancel")}
                                    </Link>
                                </div>
                        </div>
                        </div>
                    </div>
                </div>
            </div>
            </form>
        </div>
    );
}

export default PageCreate;

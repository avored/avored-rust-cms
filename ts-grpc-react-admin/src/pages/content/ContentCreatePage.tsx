import ErrorMessage from "../../components/ErrorMessage";
import {Link, useSearchParams} from "react-router-dom";
import {ContentSidebar} from "./ContentSidebar";
import {useTranslation} from "react-i18next";
import {UseStoreContentHook} from "../../hooks/content/UseStoreContentHook";
import InputField from "../../components/InputField";
import {UseContentCreateSchema} from "../../schemas/content/UseContentCreateSchema";
import {joiResolver} from "@hookform/resolvers/joi";
import {Controller, useFieldArray, useForm} from "react-hook-form";
import {
    ContentFieldDataType,
    ContentFieldFieldType,
    SaveContentFieldType,
    SaveContentType
} from "../../types/content/ContentType";
import slug from "slug";
import {
    StoreContentFieldModel,
    StoreContentRequest,
    ContentFieldFieldContent as GrpcContentFieldFieldContent,
    ContentFieldData as GrpcContentFieldData,
    ContentSelectFieldData as GrpcContentSelectFieldData,
    ContentCheckboxFieldData as GrpcContentCheckboxFieldData,
    ContentRadioFieldData as GrpcContentRadioFieldData,
} from "../../grpc_generated/content_pb";
import {ContentFieldModal} from "./ContentFieldModal";
import {useState} from "react";
import _ from "lodash";
import AvoRedButton, {ButtonType} from "../../components/AvoRedButton";
import {Cog8ToothIcon, TrashIcon} from "@heroicons/react/16/solid";
import {TextareaField} from "../../components/TextareaField";
import SimpleMDE from "react-simplemde-editor";
import {Checkbox, Field, Label, Radio, RadioGroup, Select} from "@headlessui/react";
import clsx from "clsx";
import {ChevronDownIcon, FolderPlusIcon} from "@heroicons/react/24/solid";
import AvoredModal from "../../components/AvoredModal";
import { AssetType } from "../../types/asset/AssetType";
import { AssetPaginateRequest } from "../../grpc_generated/asset_pb";
import { UseAssetTableHook } from "../../hooks/asset/UseAssetTableHook";

export const ContentCreatePage = () => {
    const [t] = useTranslation("global")
    const [searchParams] = useSearchParams()
    const [currentIndex, setCurrentIndex] = useState<number>(0);
    const [isContentFieldModalOpen, setIsContentFieldModalOpen] = useState<boolean>(false);
    const [isSelectAssetModalOpen, setIsSelectAssetModalOpen] = useState<boolean>(false);

    const contentType: string = searchParams.get("type") as string
    const {mutate, error} = UseStoreContentHook()

    const backend_url = process.env.REACT_APP_BACKEND_BASE_URL;

    const assetRequest = new AssetPaginateRequest();
    const asset_api_table_response = UseAssetTableHook(assetRequest);

    const data_list = asset_api_table_response.data?.data?.dataList ?? [];
    const assets = data_list as Array<unknown> as AssetType[];


    const convertToContentModal = (() => {
        var contentModal: SaveContentType = {
            name : '',
            identifier: '',
            content_type: contentType,
            content_fields: []
        }

        return contentModal
    })
    const values = convertToContentModal()

    const {
        register,
        handleSubmit,
        setValue,
        getValues,
        trigger,
        control,
        formState: {errors},
    } = useForm<SaveContentType>({
        resolver: joiResolver(UseContentCreateSchema(), {allowUnknown: true}),
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
            field_type: ContentFieldFieldType.TEXT,
            field_content: {
                text_value: ""
            }
        })
        await trigger("content_fields");
        setCurrentIndex(max_index);
        setIsContentFieldModalOpen(true)
    })

    const getCheckboxCheckedStatus = (field_index: number, option_value: string) => {
        const current_value = getValues(`content_fields.${field_index}.field_content.text_value`) ?? ''

        return current_value === option_value;
    }

    const setCheckboxCheckedStatus = (async (e: any, field_index: number, option_value: string) => {
        setValue(`content_fields.${field_index}.field_content.text_value`, e ? option_value : '')
        await trigger(`content_fields.${field_index}`)
    })

    const getRadioFieldCurrentValue = ((field_index: number) => {
        return getValues(`content_fields.${field_index}.field_content.text_value`) ?? ''
    })

    const setRadioCheckedStatus = (async (option_value: string, field_index: number) => {

        setValue(`content_fields.${field_index}.field_content.text_value`, option_value)
        await trigger(`content_fields.${field_index}`)
    })



    const renderField = (field: SaveContentFieldType, index: number) => {
        switch (field.field_type) {
            case ContentFieldFieldType.TEXT:
                return (
                    <div className="mb-4">
                        <InputField
                            label={t("field_content")}
                            placeholder={t("field_content")}
                            register={register(`content_fields.${index}.field_content.text_value`)}
                        />
                    </div>
                );

            case ContentFieldFieldType.TEXTAREA:
                return (
                    <div className="mb-4">
                        <TextareaField
                            label={t("field_content")}
                            placeholder={t("field_content")}
                            register={register(`content_fields.${index}.field_content.text_value`)}
                        />
                    </div>
                );

            case ContentFieldFieldType.RICH_TEXT_EDITOR:
                return (
                    <div className="mb-4">
                        <SimpleMDE
                            value={getValues(`content_fields.${index}.field_content.text_value`)}
                            onChange={(e) => {setValue(`content_fields.${index}.field_content.text_value`, e)}}
                        />
                    </div>
                );

            case ContentFieldFieldType.NUMBER_TEXT_FIELD:
                return (
                    <div className="mb-4">
                        <InputField
                            type="number"
                            label={t("field_content")}
                            placeholder={t("field_content")}
                            register={register(`content_fields.${index}.field_content.int_value`)}
                        />
                    </div>
                );

            case ContentFieldFieldType.FLOAT_TEXT_FIELD:
                return (
                    <div className="mb-4">
                        <InputField
                            type="number"
                            label={t("field_content")}
                            placeholder={t("field_content")}
                            register={register(`content_fields.${index}.field_content.float_value`)}
                        />
                    </div>
                );
            case ContentFieldFieldType.SELECT:
                return (
                    <div className="mb-4">
                        <div className="w-full">
                            <Field>
                                <label className="text-sm text-gray-600">
                                    <label className="text-sm text-gray-600">
                                        {t!("field_content")}
                                    </label>
                                </label>
                                <div className="relative">
                                    <Select
                                        {...register(
                                            `content_fields.${index}.field_content.text_value`,
                                        )}
                                        className={clsx(
                                            'ring-1 ring-gray-400  px-2 py-2 bg-gray-300  mt-3 block w-full appearance-none rounded-lg border-none bg-gray-400.5 text-sm/6',
                                            'focus:not-data-focus:outline-none data-focus:outline-2 data-focus:-outline-offset-2 data-focus:outline-white/25',
                                            // Make the text of each option black on Windows

                                        )}
                                    >
                                        {field.field_data?.content_select_field_options?.map((option) => {
                                            return (
                                                <option key={option.value} value={option.value}>
                                                    {option.label}
                                                </option>
                                            );
                                        })}
                                    </Select>
                                    <ChevronDownIcon
                                        className="group pointer-events-none absolute mt-2 mr-2 top-1 right-1 inset-3 w-5 h-5 text-gray-400"
                                        aria-hidden="true"
                                    />
                                </div>
                            </Field>

                        </div>
                    </div>
                );

            case ContentFieldFieldType.Checkbox:
                return (
                    <>
                        <div className="mb-4">
                            <div className="w-full">
                                <Field>
                                    <label className="text-sm text-gray-600">
                                        <label className="text-sm text-gray-600">
                                            {t!("field_content")}
                                        </label>
                                    </label>
                                    <div className="relative">

                                        {field.field_data?.content_checkbox_field_data?.map((option, option_index) => {
                                            return (
                                                <Field className="flex items-center gap-2">
                                                    <Checkbox
                                                        checked={getCheckboxCheckedStatus(index , option.value)}
                                                        onChange={e => setCheckboxCheckedStatus(e, index, option.value)}
                                                        value={option.value}
                                                        className="group block size-4 rounded border bg-white data-checked:bg-primary-500"
                                                    >
                                                        <svg className="stroke-white opacity-0 group-data-checked:opacity-100" viewBox="0 0 14 14" fill="none">
                                                            <path d="M3 8L6 11L11 3.5" strokeWidth={2} strokeLinecap="round" strokeLinejoin="round" />
                                                        </svg>
                                                    </Checkbox>
                                                    <Label>{option.label}</Label>
                                                </Field>
                                            );
                                        })}


                                    </div>
                                </Field>

                            </div>
                        </div>
                    </>
                );

            case ContentFieldFieldType.Radio:
                return (
                    <>
                        <div className="mb-4">
                            <div className="w-full">
                                <Field>
                                    <label className="text-sm text-gray-600">
                                        <label className="text-sm text-gray-600">
                                            {t!("field_content")}
                                        </label>
                                    </label>
                                    <div className="relative">

                                        <RadioGroup
                                            value={getRadioFieldCurrentValue(index)}
                                            onChange={value => setRadioCheckedStatus(value, index)}
                                            className="flex items-center gap-2">
                                            {field.field_data?.content_radio_field_data?.map((option, option_index) => {
                                                return (
                                                    <Field key={option.value} className="flex items-center gap-2">
                                                        <Radio
                                                            // onChange={e => setCheckboxCheckedStatus(e, index, option.value)}
                                                            value={option.value}
                                                            className="group block size-4 rounded border bg-white data-checked:bg-primary-500"
                                                        >
                                                            <svg className="stroke-white opacity-0 group-data-checked:opacity-100" viewBox="0 0 14 14" fill="none">
                                                                <path d="M3 8L6 11L11 3.5" strokeWidth={2} strokeLinecap="round" strokeLinejoin="round" />
                                                            </svg>
                                                        </Radio>
                                                        <Label>{option.label}</Label>
                                                    </Field>

                                                );
                                            })}
                                        </RadioGroup>


                                    </div>
                                </Field>

                            </div>
                        </div>
                    </>
                );

            case ContentFieldFieldType.Asset:
                return (
                    <div className="mb-4">
                        <div className="mb-4">
                            <AvoRedButton
                                label="Select Asset"
                                className="bg-primary-700 w-auto"
                                onClick={(e: React.MouseEvent<HTMLButtonElement>) => selectAssetButtonOnClick(e, index)}
                                type={ButtonType.button} />
                        </div>

                        <div>
                            <InputField 
                                type="hidden"
                                register={register(`content_fields.${index}.field_content.text_value`)}
                            />
                        </div>

                    </div>
                );
        }
    }

    const clickOnCogIconButton = ((currentIndex: number) => {
        setCurrentIndex(currentIndex)
        setIsContentFieldModalOpen(true)
    })

    const selectAssetButtonOnClick = ((e: React.MouseEvent<HTMLButtonElement, MouseEvent>, index: number) => {
        e.preventDefault()
        setCurrentIndex(index)
        setIsSelectAssetModalOpen(true)
    })

    const selectedAssetButtonOnClick = ((e: React.MouseEvent<HTMLButtonElement, MouseEvent>, asset: AssetType) => {
        e.preventDefault()
        setValue(`content_fields.${currentIndex}.field_content.text_value`, asset.id)
        closeSelectAssetModal()
    })

    const closeSelectAssetModal = (() => {
        setIsSelectAssetModalOpen(false)
    })

    const submitHandler = (async (data: SaveContentType) => {
        const request  = new StoreContentRequest();
        request.setName(data.name)
        request.setIdentifier(data.identifier)
        request.setContentType(contentType)
        const content_field_data_list: Array<StoreContentFieldModel> = [];

        // console.log(data.content_fields)

        data.content_fields.forEach(content_field => {
            const store_content_field_request = new StoreContentFieldModel();
            const content_field_field_content =  new GrpcContentFieldFieldContent();
            content_field_field_content.setTextValue(content_field.field_content.text_value ?? '')

            store_content_field_request.setName(content_field.name);
            store_content_field_request.setIdentifier(content_field.identifier);
            store_content_field_request.setDataType(content_field.data_type as string);
            store_content_field_request.setFieldType(content_field.field_type as string);
            store_content_field_request.setFieldContent(content_field_field_content)

            switch (content_field.field_type) {
                case ContentFieldFieldType.SELECT:
                    const content_field_options_data = new GrpcContentFieldData();

                    content_field.field_data?.content_select_field_options?.forEach((option, index) => {
                        const grpc_option = new GrpcContentSelectFieldData();
                        grpc_option.setLabel(option.label);
                        grpc_option.setValue(option.value);
                        content_field_options_data.addContentSelectFieldOptions(grpc_option, index);
                    })

                    store_content_field_request.setFieldData(content_field_options_data)
                    break;
                case ContentFieldFieldType.Checkbox:
                    const checkbox_options_data = new GrpcContentFieldData();

                    content_field.field_data?.content_checkbox_field_data?.forEach((option, index) => {
                        const grpc_option = new GrpcContentCheckboxFieldData();
                        grpc_option.setLabel(option.label);
                        grpc_option.setValue(option.value);
                        checkbox_options_data.addContentCheckboxFieldData(grpc_option, index);
                    })

                    store_content_field_request.setFieldData(checkbox_options_data)
                    break;

                case ContentFieldFieldType.Radio:
                    const radio_options_data = new GrpcContentFieldData();

                    content_field.field_data?.content_radio_field_data?.forEach((option, index) => {
                        const grpc_option = new GrpcContentRadioFieldData();
                        grpc_option.setLabel(option.label);
                        grpc_option.setValue(option.value);
                        radio_options_data.addContentRadioFieldData(grpc_option, index);
                    })

                    store_content_field_request.setFieldData(radio_options_data)
                    break;

                default:
                    break;
            }


            content_field_data_list.push(store_content_field_request)
        })

        request.setContentFieldsList(content_field_data_list)

        mutate(request)
    })

    return(
        <>
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
                                collectionType={contentType}
                            />
                        ) : (
                            <></>
                        )}

                        <div className="mb-4">
                            <InputField type="hidden" register={register("content_type")} value={contentType}  />
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
                                                                        <span className="ml-1 text-xs text-gray-500">
                                                                            ({field.value.identifier})
                                                                        </span>
                                                                    </div>
                                                                    <div className="ml-auto flex items-center">
                                                                        <div>
                                                                            <button
                                                                                type="button"
                                                                                className="outline-none"
                                                                                onClick={() => clickOnCogIconButton(index)}
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

                        {/*
                            ASSET MODAL
                        */}
                        <AvoredModal
                            modal_header="please Select asset"
                            modal_body={(
                                <>
                                    <div className="flex">
                                        {assets.map((asset: AssetType) => {
                                            return (
                                                <div>
                                                    <div className="ml-3 justify-center h-40 mb-3">
                                                        {asset.assetType === "FOLDER" ? (
                                                            <>
                                                                <FolderPlusIcon className="h-32 w-32 text-gray-300" />
                                                            </>
                                                        ) : (
                                                            <>
                                                                <img
                                                                    src={`${backend_url}${asset.newPath}`}
                                                                    className="h-40"
                                                                    alt={asset.name}
                                                                />
                                                            </>
                                                        )}
                                                    </div>
                                                    <div>
                                                        <button type="button"
                                                            onClick={e => selectedAssetButtonOnClick(e, asset)}
                                                            className="text-white bg-primary-600 hover:bg-primary-800 focus:ring-4 focus:ring-primary-300 font-medium rounded-lg text-base inline-flex items-center px-3 py-2.5 text-center mr-2">
                                                            Select
                                                        </button>
                                                    </div>
                                                </div>
                                            );
                                        })}
                                    </div>
                                </>
                            )} isOpen={isSelectAssetModalOpen} closeModal={closeSelectAssetModal}
                        ></AvoredModal>

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
                                to={`/admin/content?type=${contentType}`}
                                className="ml-auto font-medium text-gray-600 hover:text-gray-500"
                            >
                                {t("cancel")}
                            </Link>
                        </div>
                    </form>
                </div>
            </div>
        </>
    )
}

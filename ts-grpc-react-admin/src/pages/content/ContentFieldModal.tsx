import {UseFormGetValues, UseFormRegister, UseFormSetValue, UseFormTrigger,} from "react-hook-form";
import {
    ContentCheckboxFieldData,
    ContentFieldData,
    ContentFieldDataType,
    ContentFieldFieldType,
    ContentSelectFieldData,
    SaveContentFieldType,
    SaveContentType,
} from "../../types/content/ContentType";
import AvoredModal from "../../components/AvoredModal";
import AvoRedButton from "../../components/AvoRedButton";
import {useTranslation} from "react-i18next";
import InputField from "../../components/InputField";
import React from "react";
import slug from "slug";
import _ from "lodash";
import {PlusIcon} from "@heroicons/react/24/solid";
import {MinusIcon} from "@heroicons/react/16/solid";

type ContentFieldProps = {
    register: UseFormRegister<SaveContentType>;
    currentIndex: number;
    getValues: UseFormGetValues<SaveContentType>;
    setValue: UseFormSetValue<SaveContentType>;
    trigger: UseFormTrigger<SaveContentType>;
    setIsOpen: React.Dispatch<React.SetStateAction<boolean>>;
    isOpen: boolean;
    collectionType: string;
}

export const ContentFieldModal = ({
                                      register,
                                      currentIndex,
                                      getValues,
                                      setValue,
                                      trigger,
                                      setIsOpen,
                                      isOpen,
                                      collectionType,
                                  }: ContentFieldProps) => {
    const [t] = useTranslation("global");

    const onContentFieldChange = async (
        index: number,
        field_type: ContentFieldFieldType,
        data_type: ContentFieldDataType
    ) => {
        setValue(`content_fields.${index}.field_type`, field_type);
        setValue(`content_fields.${index}.data_type`, data_type);

        switch (field_type) {
            case ContentFieldFieldType.SELECT:
                const empty_option: ContentSelectFieldData = {
                    label: "",
                    value: "",
                };
                const content_field_data: ContentFieldData = {
                    content_select_field_options: [],
                    content_checkbox_field_data: []
                };
                if (typeof content_field_data.content_select_field_options == "undefined") {
                    content_field_data.content_select_field_options = [];
                }
                content_field_data.content_select_field_options.push(empty_option);
                setValue(`content_fields.${index}.field_data`, content_field_data);

                break;

            case ContentFieldFieldType.Checkbox:
                const checkbox_option: ContentCheckboxFieldData = {
                    label: "",
                    value: "",
                };
                const content_field_checkbox_data: ContentFieldData = {
                    content_select_field_options: [],
                    content_checkbox_field_data: []
                };
                if (typeof content_field_checkbox_data.content_checkbox_field_data == "undefined") {
                    content_field_checkbox_data.content_checkbox_field_data = [];
                }
                content_field_checkbox_data.content_checkbox_field_data.push(checkbox_option);
                setValue(`content_fields.${index}.field_data`, content_field_checkbox_data);

                break;
        }

        await trigger(`content_fields.${index}`);
    };

    const contentFieldNameInputChange = ((e: React.KeyboardEvent<HTMLInputElement>, index: number) => {
        e.stopPropagation();

        setValue(`content_fields.${index}.identifier`, slug(e.currentTarget.value));
    })

    const optionLabelOnChange = async (
        e: any,
        field_index: number,
        option_index: number,
    ) => {
        setValue(
            `content_fields.${field_index}.field_data.content_select_field_options.${option_index}.label`,
            e.target.value,
        );
        await trigger("content_fields");
    };

    const optionAddOnClick = async (
        e: React.MouseEvent<HTMLButtonElement>,
        field_index: number,
    ) => {
        e.preventDefault();
        const content_field: SaveContentFieldType = getValues(
            `content_fields.${field_index}`,
        );
        const empty_option: ContentSelectFieldData = {
            label: "",
            value: "",
        };

        content_field.field_data?.content_select_field_options?.push(empty_option);

        await trigger("content_fields");
    };

    const optionRemoveOnClick = async (
        e: React.MouseEvent<HTMLButtonElement>,
        field_index: number,
        option_index: number,
    ) => {
        e.preventDefault();
        const content_field: SaveContentFieldType = getValues(
            `content_fields.${field_index}`,
        );
        content_field.field_data?.content_select_field_options?.splice(option_index, 1);

        await trigger(`content_fields.${field_index}`);
    };

    const optionValueOnChange = async (
        e: any,
        field_index: number,
        option_index: number,
    ) => {
        setValue(
            `content_fields.${field_index}.field_data.content_select_field_options.${option_index}.value`,
            e.target.value,
        );
        await trigger("content_fields");
    };

    const checkboxLabelOnChange = async (
        e: any,
        field_index: number,
        option_index: number,
    ) => {
        setValue(
            `content_fields.${field_index}.field_data.content_checkbox_field_data.${option_index}.label`,
            e.target.value,
        );
        await trigger("content_fields");
    };

    const checkboxAddOnClick = async (
        e: React.MouseEvent<HTMLButtonElement>,
        field_index: number,
    ) => {
        e.preventDefault();
        const content_field: SaveContentFieldType = getValues(
            `content_fields.${field_index}`,
        );
        const empty_option: ContentCheckboxFieldData = {
            label: "",
            value: "",
        };

        content_field.field_data?.content_checkbox_field_data?.push(empty_option);

        await trigger("content_fields");
    };

    const checkboxRemoveOnClick = async (
        e: React.MouseEvent<HTMLButtonElement>,
        field_index: number,
        option_index: number,
    ) => {
        e.preventDefault();
        const content_field: SaveContentFieldType = getValues(
            `content_fields.${field_index}`,
        );
        content_field.field_data?.content_checkbox_field_data?.splice(option_index, 1);

        await trigger(`content_fields.${field_index}`);
    };

    const checkboxValueOnChange = async (
        e: any,
        field_index: number,
        option_index: number,
    ) => {
        setValue(
            `content_fields.${field_index}.field_data.content_checkbox_field_data.${option_index}.value`,
            e.target.value,
        );
        await trigger("content_fields");
    };

    const renderFieldData = ((index: number) => {

        const content_field: SaveContentFieldType = getValues(
            `content_fields.${index}`,
        );

        switch (content_field.field_type) {
            case ContentFieldFieldType.SELECT:
                return (
                    <>
                        {_.get(content_field, 'field_data.content_select_field_options', []).map(
                            (option, option_index) => {
                                return (
                                    <div key={`avored-select-${option_index}`} className="block mt-3 w-full">
                                        <div className="flex w-full items-center">
                                            <div className="w-1/2">
                                                <div className="block">
                                                    <input
                                                        value={option.label}
                                                        onChange={(e) =>
                                                            optionLabelOnChange(
                                                                e,
                                                                index,
                                                                option_index,
                                                            )
                                                        }
                                                        placeholder={t("label")}
                                                        className="appearance-none rounded-md ring-1 ring-gray-400
                                                              relative border-0 block w-full px-3 py-2 placeholder-gray-500 text-gray-900
                                                              active::ring-primary-500
                                                              focus:ring-primary-500 focus:outline-none focus:z-10
                                                              disabled:bg-gray-200 disabled:opacity-70
                                                              sm:text-sm "
                                                    />
                                                </div>
                                            </div>
                                            <div className="w-1/2 ml-3">
                                                <div className="flex items-center w-full">
                                                    <div>
                                                        <input
                                                            value={option.value}
                                                            onChange={(e) =>
                                                                optionValueOnChange(
                                                                    e,
                                                                    index,
                                                                    option_index,
                                                                )
                                                            }
                                                            className="appearance-none rounded-md ring-1 ring-gray-400
                                                                      relative border-0 block w-full px-3 py-2 placeholder-gray-500 text-gray-900
                                                                      active::ring-primary-500
                                                                      focus:ring-primary-500 focus:outline-none focus:z-10
                                                                      disabled:bg-gray-200 disabled:opacity-70
                                                                      sm:text-sm
                                                                      "
                                                            placeholder={t("value")}
                                                        />
                                                    </div>
                                                    <div>
                                                        {_.size(getValues(
                                                            `content_fields.${index}.field_data.content_select_field_options`,
                                                        )) ===
                                                        option_index + 1 ? (
                                                            <>
                                                                <button
                                                                    onClick={(e) =>
                                                                        optionAddOnClick(e, currentIndex)
                                                                    }
                                                                    className="ml-2"
                                                                >
                                                                    <PlusIcon className="w-5 h-5"/>
                                                                </button>
                                                            </>
                                                        ) : (
                                                            <>
                                                                <button
                                                                    onClick={(e) =>
                                                                        optionRemoveOnClick(
                                                                            e,
                                                                            currentIndex,
                                                                            option_index,
                                                                        )
                                                                    }
                                                                    className="ml-2"
                                                                >
                                                                    <MinusIcon className="w-5 h-5"/>
                                                                </button>
                                                            </>
                                                        )}
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                );
                            },
                        )}
                    </>
                );

            case ContentFieldFieldType.Checkbox:
                return (
                    <>
                        {_.get(content_field, 'field_data.content_checkbox_field_data', []).map(
                            (option, option_index) => {
                                return (
                                    <div key={`avored-select-${option_index}`} className="block mt-3 w-full">
                                        <div className="flex w-full items-center">
                                            <div className="w-1/2">
                                                <div className="block">
                                                    <input
                                                        value={option.label}
                                                        onChange={(e) =>
                                                            checkboxLabelOnChange(
                                                                e,
                                                                index,
                                                                option_index,
                                                            )
                                                        }
                                                        placeholder={t("label")}
                                                        className="appearance-none rounded-md ring-1 ring-gray-400
                                                              relative border-0 block w-full px-3 py-2 placeholder-gray-500 text-gray-900
                                                              active::ring-primary-500
                                                              focus:ring-primary-500 focus:outline-none focus:z-10
                                                              disabled:bg-gray-200 disabled:opacity-70
                                                              sm:text-sm "
                                                    />
                                                </div>
                                            </div>
                                            <div className="w-1/2 ml-3">
                                                <div className="flex items-center w-full">
                                                    <div>
                                                        <input
                                                            value={option.value}
                                                            onChange={(e) =>
                                                                checkboxValueOnChange(
                                                                    e,
                                                                    index,
                                                                    option_index,
                                                                )
                                                            }
                                                            className="appearance-none rounded-md ring-1 ring-gray-400
                                                                      relative border-0 block w-full px-3 py-2 placeholder-gray-500 text-gray-900
                                                                      active::ring-primary-500
                                                                      focus:ring-primary-500 focus:outline-none focus:z-10
                                                                      disabled:bg-gray-200 disabled:opacity-70
                                                                      sm:text-sm
                                                                      "
                                                            placeholder={t("value")}
                                                        />
                                                    </div>
                                                    <div>
                                                        {_.size(getValues(
                                                            `content_fields.${index}.field_data.content_checkbox_field_data`,
                                                        )) ===
                                                        option_index + 1 ? (
                                                            <>
                                                                <button
                                                                    onClick={(e) =>
                                                                        checkboxAddOnClick(e, currentIndex)
                                                                    }
                                                                    className="ml-2"
                                                                >
                                                                    <PlusIcon className="w-5 h-5"/>
                                                                </button>
                                                            </>
                                                        ) : (
                                                            <>
                                                                <button
                                                                    onClick={(e) =>
                                                                        checkboxRemoveOnClick(
                                                                            e,
                                                                            currentIndex,
                                                                            option_index,
                                                                        )
                                                                    }
                                                                    className="ml-2"
                                                                >
                                                                    <MinusIcon className="w-5 h-5"/>
                                                                </button>
                                                            </>
                                                        )}
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                );
                            },
                        )}
                    </>
                );

            default:
                return <></>;
        }
    })

    return (
        <AvoredModal
            closeModal={() => setIsOpen(false)}
            modal_body={
                <div className="block">
                    <div className="flex w-full">
                        <div className="flex-1 pr-3">
                            <div className="mb-3">
                                <InputField
                                    placeholder={t("name")}
                                    label={t("name")}
                                    onKeyUp={(e: React.KeyboardEvent<HTMLInputElement>) => contentFieldNameInputChange(e, currentIndex)}
                                    register={register(`content_fields.${currentIndex}.name`)}
                                />
                            </div>
                            <div className="mb-3">
                                <InputField
                                    placeholder={t("identifier")}
                                    label={t("identifier")}
                                    register={register(
                                        `content_fields.${currentIndex}.identifier`
                                    )}
                                />
                            </div>

                            <div className="w-full">{renderFieldData(currentIndex)}</div>

                            {/*<div className="w-full">{renderFieldData(currentIndex)}</div> */}
                        </div>

                        <div className="ml-auto">
                            <div className="w-64 border-l p-3 mr-auto">
                                <div
                                    onClick={() =>
                                        onContentFieldChange(
                                            currentIndex,
                                            ContentFieldFieldType.TEXT,
                                            ContentFieldDataType.TEXT
                                        )
                                    }
                                    className={`${getValues(`content_fields.${currentIndex}.field_type`) === ContentFieldFieldType.TEXT ? "bg-primary-200" : "bg-gray-300"} 
                        ring-1 ring-gray-300 hover:cursor-pointer hover:ring-primary-300 p-3 rounded`}
                                >
                                    {t("text_field")}
                                </div>
                                <div
                                    onClick={() =>
                                        onContentFieldChange(
                                            currentIndex,
                                            ContentFieldFieldType.TEXTAREA,
                                            ContentFieldDataType.TEXT
                                        )
                                    }
                                    className={`${getValues(`content_fields.${currentIndex}.field_type`) === ContentFieldFieldType.TEXTAREA ? "bg-primary-200" : "bg-gray-300"} 
                        ring-1 ring-gray-300 mt-3 hover:cursor-pointer hover:ring-primary-300 p-3 rounded`}
                                >
                                    {t("textarea")}
                                </div>

                                <div
                                    onClick={() =>
                                        onContentFieldChange(
                                            currentIndex,
                                            ContentFieldFieldType.RICH_TEXT_EDITOR,
                                            ContentFieldDataType.TEXT
                                        )
                                    }
                                    className={`${getValues(`content_fields.${currentIndex}.field_type`) === ContentFieldFieldType.RICH_TEXT_EDITOR ? "bg-primary-200" : "bg-gray-300"} 
                        ring-1 ring-gray-300 mt-3 hover:cursor-pointer hover:ring-primary-300 p-3 rounded`}
                                >
                                    {t("rich_text_editor")}
                                </div>

                                <div
                                    onClick={() =>
                                        onContentFieldChange(
                                            currentIndex,
                                            ContentFieldFieldType.NUMBER_TEXT_FIELD,
                                            ContentFieldDataType.INT
                                        )
                                    }
                                    className={`${getValues(`content_fields.${currentIndex}.field_type`) === ContentFieldFieldType.NUMBER_TEXT_FIELD ? "bg-primary-200" : "bg-gray-300"} 
                        ring-1 ring-gray-300 mt-3 hover:cursor-pointer hover:ring-primary-300 p-3 rounded`}
                                >
                                    {t("number_text_field")}
                                </div>

                                <div
                                    onClick={() =>
                                        onContentFieldChange(
                                            currentIndex,
                                            ContentFieldFieldType.FLOAT_TEXT_FIELD,
                                            ContentFieldDataType.FLOAT
                                        )
                                    }
                                    className={`${getValues(`content_fields.${currentIndex}.field_type`) === ContentFieldFieldType.FLOAT_TEXT_FIELD ? "bg-primary-200" : "bg-gray-300"} 
                        ring-1 ring-gray-300 mt-3 hover:cursor-pointer hover:ring-primary-300 p-3 rounded`}
                                >
                                    {t("float_text_field")}
                                </div>

                                <div
                                    onClick={() =>
                                        onContentFieldChange(
                                            currentIndex,
                                            ContentFieldFieldType.SELECT,
                                            ContentFieldDataType.TEXT
                                        )
                                    }
                                    className={`${getValues(`content_fields.${currentIndex}.field_type`) === ContentFieldFieldType.SELECT ? "bg-primary-200" : "bg-gray-300"} 
                        ring-1 ring-gray-300 mt-3 hover:cursor-pointer hover:ring-primary-300 p-3 rounded`}
                                >
                                    {t("select_field")}
                                </div>

                                <div
                                    onClick={() =>
                                        onContentFieldChange(
                                            currentIndex,
                                            ContentFieldFieldType.Checkbox,
                                            ContentFieldDataType.TEXT
                                        )
                                    }
                                    className={`${getValues(`content_fields.${currentIndex}.field_type`) === ContentFieldFieldType.Checkbox ? "bg-primary-200" : "bg-gray-300"} 
                        ring-1 ring-gray-300 mt-3 hover:cursor-pointer hover:ring-primary-300 p-3 rounded`}
                                >
                                    {t("checkbox_field")}
                                </div>


                            </div>
                        </div>
                    </div>
                    <hr className="mt-3"/>
                    <div className="mt-3">
                        <div className="flex">
                            <div>
                                <AvoRedButton
                                    onClick={() => setIsOpen(false)}
                                    className="bg-primary-500"
                                    label={t("create_content_field")}
                                />
                            </div>
                            <div className="ml-3">
                                <AvoRedButton
                                    onClick={() => setIsOpen(false)}
                                    label={t("cancel")}
                                />
                            </div>
                        </div>
                    </div>
                </div>
            }
            modal_header={`${t(collectionType)} ${t("content_field")}`}
            isOpen={isOpen}
        ></AvoredModal>
    );
};

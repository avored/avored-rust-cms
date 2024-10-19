import React, { useState } from "react";
import { Link, useParams } from "react-router-dom";
import {
  Cog8ToothIcon,
  PlusIcon,
  TrashIcon,
} from "@heroicons/react/24/solid";
import InputField from "../../components/InputField";
import { useGetPage } from "./hooks/useGetPage";
import { useUpdatePage } from "./hooks/useUpdatePage";
import { useTranslation } from "react-i18next";
import { Controller, useFieldArray, useForm } from "react-hook-form";
import { joiResolver } from "@hookform/resolvers/joi";
import { usePageEditSchema } from "./schemas/page.edit.schema";
import { usePagePutSchema } from "./schemas/page.put.schema";
import { PutPageIdentifierType } from "../../types/page/PutPageIdentifierType";
import { usePutPageIdentifier } from "./hooks/usePutPageIdentifier";
import {
  AvoRedPageDataType,
  AvoRedPageFieldType,
} from "../../types/page/IPageModel";
import _ from "lodash";
import {
  AvoRedPageFieldRadioFieldDataOptions, PageFieldContent, PageTextContent,
  SaveFieldType,
  SavePageType,
} from "../../types/page/CreatablePageType";
import SimpleMdeReact from "react-simplemde-editor";
import { PageFieldModal } from "./PageFieldModal";

function PageEdit() {
  const [isOpen, setIsOpen] = useState<boolean>(false);
  const [currentIndex, setCurrentIndex] = useState<number>(0);

  const params = useParams();
  const [t] = useTranslation("global");
  const [isEditableIdentifier, setIsEditableIdentifier] =
    useState<boolean>(true);

  const { mutate } = useUpdatePage(params.page_id ?? "");
  const { data } = useGetPage(params.page_id ?? "");
  const values = data?.data.page_model;

  const {
    control,
    register,
    handleSubmit,
    formState: { errors },
    getValues,
    setValue,
    trigger,
  } = useForm<SavePageType>({
    resolver: joiResolver(usePageEditSchema(), { allowUnknown: true }),
    values,
  });

  const { register: putPageRegister, getValues: getPageIdentifierValue } =
    useForm<PutPageIdentifierType>({
      resolver: joiResolver(usePagePutSchema(), { allowUnknown: true }),
      values: {
        identifier: data?.data.page_model.identifier,
      },
    });

  const { mutate: putPageIdentifierMutate } = usePutPageIdentifier(
    params.page_id ?? "",
  );

  const editableIdentifierOnClick = () => {
    setIsEditableIdentifier(false);
  };
  const saveIdentifierOnClick = () => {
    putPageIdentifierMutate({
      identifier: getPageIdentifierValue("identifier"),
    });
    setIsEditableIdentifier(true);
  };

  const cancelIdentifierOnClick = () => {
    setIsEditableIdentifier(true);
  };

  const { fields, append, remove } = useFieldArray({
    control,
    name: "page_fields",
  });

  const deletePageFieldOnClick = async (
    e: React.MouseEvent<HTMLDivElement>,
    index: number,
  ) => {
    e.preventDefault();
    remove(index);
    setCurrentIndex(0);
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
  const textEditorGetValue = (field_index: number): string => {
    const page_field_content: PageFieldContent =  getValues(`page_fields.${field_index}.field_content`);

    return (page_field_content.text_value?.text_value) as string;
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
                {...register(
                    `components_content.${pageComponentIndex}.elements.${pageComponentFieldIndex}.element_content`,
                )}
            ></textarea>
                        <label
                            htmlFor={componentField.identifier}
                            className="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300"
                        >
                            {componentField.name}
                        </label>
                    </div>
                );
            case "select":
                return (
                    <div className="p-3">
                        <AvoRedMultiSelectField
                            label="test dropdown"
                            selectedOption={getSelectFieldDataOptionCurrentValue(pageComponentIndex, pageComponentFieldIndex)}
                            options={componentField.element_data ?? []}
                            onChangeSelectedOption={((val: Array<string>) => setSelectedFieldDataOption(val, pageComponentIndex, pageComponentFieldIndex))}
                        ></AvoRedMultiSelectField>
                        <label
                            htmlFor={componentField.identifier}
                            className="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300"
                        >
                            {componentField.name}
                        </label>
                    </div>
                );
            case "text":
            default:
                return (
                    <div>
                        <InputField
                            label={componentField.name}
                            type="text"
                            name={componentField.identifier}
                            register={register(
                                `components_content.${pageComponentIndex}.elements.${pageComponentFieldIndex}.element_content`,
                            )}
                        />
                    </div>
                );
        }
    };

    const renderComponentField = (
        componentField: IEditablePageComponentFieldModel,
        pageComponentIndex: number,
        pageComponentFieldIndex: number,
    ) => {
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
                value={textEditorGetValue(index)}
                onChange={(contentValue) =>
                  textEditorOnChange(contentValue, index)
                }
              />
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

  const addFieldOnClick = async (
    e: React.MouseEvent<HTMLElement>,
    max_index: number,
  ) => {
    e.preventDefault();
    const empty_content: PageTextContent =  {
      text_value: "",
    }
    const empty_page_content: PageFieldContent = {
      text_value: empty_content
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
    setIsOpen(true);
  };

  const submitHandler = async (data: SavePageType) => {
      console.log(data)
    // mutate(data);
  };

  return (
    <div className="flex-1 bg-white">
      <div className="px-5 pl-64 ">
        <div className="w-full">
          <div className="block rounded-lg p-6">
            <h1 className="text-xl font-semibold mb-4 text-gray-900">
              {t("page_information")}
            </h1>

            <form onSubmit={handleSubmit(submitHandler)}>
              {_.size(fields) > 0 ? (
                <>
                  <PageFieldModal
                    register={register}
                    currentIndex={currentIndex}
                    getValues={getValues}
                    setValue={setValue}
                    trigger={trigger}
                    setIsOpen={setIsOpen}
                    isOpen={isOpen} />
                </>
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
                  register={putPageRegister("identifier")}
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
                    className="relative hover:ring-1 ring-primary-300 rounded mb-5 flex mt-5 py-3 w-full"
                  >
                    <Controller
                      name={`page_fields.${index}`}
                      render={({ field: page_field }) => {
                        return (
                          <>
                            <div className="flex mt-3 w-full justify-center">
                              <div className="flex-1 p-3">
                                <div className="p-3 bg-gray-200 rounded">
                                  <div className="flex text-sm w-full border-gray-300 border-b py-2">
                                    <div className="flex-1 overflow-hidden">
                                      <span>{page_field.value.name}</span>
                                      <span className="ml-1 text-xs text-gray-500">
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
                                          <Cog8ToothIcon className="w-5 h-5" />
                                        </button>
                                      </div>
                                      <div
                                        onClick={(e) =>
                                          deletePageFieldOnClick(e, index)
                                        }
                                        className="ml-3"
                                      >
                                        <TrashIcon className="w-4 h-4" />
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
                  <PlusIcon className="text-primary-500 h-6 w-6" />
                  <span className="text-sm ml-1 text-primary-500">
                    {t("add_field")}
                  </span>
                </button>
              </div>

              <div className="flex items-center">
                <button
                  type="submit"
                  className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                >
                  {t("save")}
                </button>
                <Link
                  to={`/admin/page`}
                  className="ml-auto font-medium text-gray-600 hover:text-gray-500"
                >
                  {t("cancel")}
                </Link>
              </div>
            </form>
          </div>
        </div>
      </div>
    </div>
  );
}

export default PageEdit;

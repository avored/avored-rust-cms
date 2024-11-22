import React, { useState } from "react";
import { Link, useParams } from "react-router-dom";
import { Cog8ToothIcon, PlusIcon, TrashIcon } from "@heroicons/react/24/solid";
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
  AvoRedPageStatus,
} from "../../types/page/IPageModel";
import _ from "lodash";
import {
  AvoRedPageFieldCheckboxFieldDataOptions,
  AvoRedPageFieldRadioFieldDataOptions,
  PageFieldContent,
  PageTextContent,
  SaveFieldType,
  SavePageType,
} from "../../types/page/CreatablePageType";
import SimpleMdeReact from "react-simplemde-editor";
import { PageFieldModal } from "./PageFieldModal";
import AvoRedButton, { ButtonType } from "../../components/AvoRedButton";
import {SingleImageModal} from "./SingleImageModal";
import IAssetModel from "../../types/asset/IAssetModel";

function PageEdit() {
  const [isOpen, setIsOpen] = useState<boolean>(false);
  const [isSingleAssetModalOpen, setisSingleAssetModalOpen] = useState(false);
  const [currentIndex, setCurrentIndex] = useState<number>(0);
  const backend_url = import.meta.env.VITE_AVORED_BACKEND_BASE_URL;
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
    const page_content: PageFieldContent = {
      text_value: text_content,
    };
    setValue(`page_fields.${field_index}.field_content`, page_content);
  };
  const textEditorGetValue = (field_index: number): string => {
    const page_field_content: PageFieldContent = getValues(
      `page_fields.${field_index}.field_content`,
    );

    return page_field_content.text_value?.text_value as string;
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
              {...register(
                `page_fields.${index}.field_content.text_value.text_value`,
              )}
            ></textarea>
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
                  <div key={`avored-radio-${option.value}`} className="w-full">
                    <input
                      id={`avored-radio-${option.value}`}
                      type="radio"
                      value={option.value}
                      {...register(
                        `page_fields.${index}.field_content.text_value.text_value`,
                      )}
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
      case AvoRedPageFieldType.SELECT:
        return (
          <div className="mb-4">
            <label className="text-sm text-gray-600">
              {t!("field_content")}
            </label>

            <select
              {...register(
                `page_fields.${index}.field_content.text_value.text_value`,
              )}
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
              register={register(
                `page_fields.${index}.field_content.text_value.text_value`,
              )}
            />
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
                value={textEditorGetValue(index)}
                onChange={(contentValue) =>
                  textEditorOnChange(contentValue, index)
                }
              />
            </div>
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
                  <div
                    key={`avredo-checkbox-${option.value}`}
                    className="w-full"
                  >
                    <input
                      id={`avored-checkbox-${option.value}`}
                      type="checkbox"
                      value={option.value}
                      {...register(
                        `page_fields.${index}.field_content.array_value.array_value`,
                      )}
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
              register={register(
                `page_fields.${index}.field_content.text_value.text_value`,
              )}
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
    const empty_content: PageTextContent = {
      text_value: "",
    };
    const empty_page_content: PageFieldContent = {
      text_value: empty_content,
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

  const selectedAsset = (index: number, selectedAsset: IAssetModel) => {
      setValue(`page_fields.${index}.field_content.text_value.text_value`, selectedAsset.path);
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

  const submitHandler = async (data: SavePageType) => {
    data.status = AvoRedPageStatus.Draft;
    mutate(data);
  };

  const saveAsPublishedOnClick = (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();

    const data: SavePageType = {
      name: getValues("name"),
      identifier: getValues("identifier"),
      page_fields: getValues("page_fields"),
      status: AvoRedPageStatus.Published,
    };
    mutate(data);
  };

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
                    <>
                      <PageFieldModal
                        register={register}
                        currentIndex={currentIndex}
                        getValues={getValues}
                        setValue={setValue}
                        trigger={trigger}
                        setIsOpen={setIsOpen}
                        isOpen={isOpen}
                      />
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
                </div>
                <div className="w-1/6 pr-3">
                  <div className="mt-7">
                    <button
                      type="submit"
                      className="bg-gray-500 rounded px-4 py-2 text-white"
                    >
                      {t("save_as_draft")}
                    </button>
                  </div>
                  <div className="mt-5">
                    <button
                      type="button"
                      onClick={(e) => saveAsPublishedOnClick(e)}
                      className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                    >
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

export default PageEdit;

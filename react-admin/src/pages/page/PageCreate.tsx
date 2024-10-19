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
  AvoRedPageFieldRadioFieldDataOptions, PageFieldContent, PageTextContent,
  SaveFieldType,
  SavePageType,
} from "../../types/page/CreatablePageType";
import {
  AvoRedPageDataType,
  AvoRedPageFieldType,
} from "../../types/page/IPageModel";
import _ from "lodash";
import SimpleMdeReact from "react-simplemde-editor";
import { PageFieldModal } from "./PageFieldModal";

function PageCreate() {
  const [t] = useTranslation("global");
  const [isOpen, setIsOpen] = useState<boolean>(false);
  const [currentIndex, setCurrentIndex] = useState<number>(0);

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

    const renderComponentElementType = (
        componentElement: ICreatablePageComponentElementModel,
        pageComponentIndex: number,
        componentElementIndex: number,
    ) => {
        switch (componentElement.element_type) {
            case "textarea":
                return (
                    <div className="p-3">
            <textarea
                {...register(
                    `components_content.${pageComponentIndex}.elements.${componentElementIndex}.element_content`,
                )}
            ></textarea>
                        <label
                            htmlFor={componentElement.identifier}
                            className="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300"
                        >
                            {componentElement.name}
                        </label>
                    </div>
                );
            case "select":
                return (
                    <div className="p-3">
                        <AvoRedMultiSelectField
                            label="test dropdown"
                            selectedOption={getSelectElementDataOptionCurrentValue(pageComponentIndex, componentElementIndex)}
                            options={componentElement.element_data ?? []}
                            onChangeSelectedOption={((val: Array<string>) => setSelectedElementDataOption(val, pageComponentIndex, componentElementIndex))}
                        ></AvoRedMultiSelectField>
                        <label
                            htmlFor={componentElement.identifier}
                            className="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300"
                        >
                            {componentElement.name}
                        </label>
                    </div>
                );
            case "radio":
                return (
                    <div className="p-3">
                        <input
                            id={componentElement.identifier}
                            type="radio"
                            value={componentElement.identifier}
                            className="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-primary-500 dark:focus:ring-primary-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                        />
                        <label
                            htmlFor={componentElement.identifier}
                            className="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300"
                        >
                            {componentElement.name}
                        </label>
                    </div>
                );
            case "text":
            default:
                return (
                    <div>
                        <InputField
                            name={`components_content.${pageComponentIndex}.elements.${componentElementIndex}.element_content`}
                            label={componentElement.name}
                            register={register(
                                `components_content.${pageComponentIndex}.elements.${componentElementIndex}.element_content`,
                            )}
                        />
                        <InputField
                            type="hidden"
                            value="TEXT"
                            register={register(
                                `components_content.${pageComponentIndex}.elements.${componentElementIndex}.element_data_type`,
                            )}
                        />
                    </div>
                );
        }
    };

    const renderComponentElement = (
        componentElement: ICreatablePageComponentElementModel,
        pageComponentIndex: number,
        componentElementIndex: number,
    ) => {
        return (
            <div className="ring-1 my-2 ring-gray-200" key={componentElement.identifier}>
                {JSON.stringify(componentElement)}
                {renderComponentElementType(
                    componentElement,
                    pageComponentIndex,
                    componentElementIndex,
                )}
            </div>
        );
      case AvoRedPageFieldType.TEXT:
        return (
            <div
                key={pageComponent.id}
                className="my-5 ring-1 ring-gray-200 rounded p-3"
            >
                <div>component name: {pageComponent.name}</div>
                <div>component identifier: {pageComponent.identifier}</div>
                <div>
                    Component Fields
                    {pageComponent.elements.map((componentElement, componentElementIndex) => {
                        return renderComponentElement(
                            componentElement,
                            pageComponentIndex,
                            componentElementIndex,
                        );
                    })}
                </div>
            </div>
        );
    }
  };

  // template start here
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
                      render={({ field: page_field }) => {
                        return (
                          <>
                            <div className="flex mt-3 w-full justify-center">
                              <div className="flex-1 p-3">
                                <div className="p-3 bg-gray-200 rounded">
                                  <div className="flex text-sm w-full border-gray-300 border-b py-2">
                                    <div className="flex-1">
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

              <hr />
              <div className="mt-5  flex items-center">
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

export default PageCreate;

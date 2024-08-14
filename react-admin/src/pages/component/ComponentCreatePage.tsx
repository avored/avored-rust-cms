import React from "react";
import { Link } from "react-router-dom";
import { PlusIcon, TrashIcon } from "@heroicons/react/24/solid";
import InputField from "../../components/InputField";
import { useStoreComponent } from "./hooks/useStoreComponent";
import { useTranslation } from "react-i18next";
import { AvoRedFieldTypesEnum } from "../../types/field/AvoRedFieldTypesEnum";
import { Controller, useFieldArray, useForm } from "react-hook-form";
import { joiResolver } from "@hookform/resolvers/joi";
import { useComponentCreateSchema } from "./schemas/component.create.schema";
import ICreatableComponent, {CreatableElementDataType} from "../../types/component/ICreatableComponent";
import ErrorMessage from "../../components/ErrorMessage";

export const ComponentCreatePage = (() => {
  const { mutate, error } = useStoreComponent();
  const {
    control,
    register,
    handleSubmit,
    formState: { errors },
    setValue,
    trigger,
  } = useForm<ICreatableComponent>({
    resolver: joiResolver(useComponentCreateSchema()),
  });
  const { fields: elements, append, remove } = useFieldArray({
    control,
    name: "elements",
  });

  const [t] = useTranslation("global");

  const addElementOnClick = () => {
    append({ name: "", identifier: "", element_type: AvoRedFieldTypesEnum.TEXT, element_data_type: 'TEXT' });
  };

  const deleteElementOnClick = (elementIndex: number) => {
    remove(elementIndex);
  };

  const elementTypeButtonOnClick = (
      fieldIndex: number,
      fieldTypeValue: string,
  ) => {
    setValue(`elements.${fieldIndex}.element_type`, fieldTypeValue);
    setValue(`elements.${fieldIndex}.element_data`, [{ label: "", value: "" }]);
    // Ideally value of this data type can be based on element
    // e.g: Number Input field will have INT(It should match rust backend type) data type
    setValue(`elements.${fieldIndex}.element_data_type`, 'TEXT');
    trigger(`elements.${fieldIndex}`);
  };

  const optionDeleteActionOnClick = (
      e: React.MouseEvent,
      elementIndex: number,
      element_data: Array<CreatableElementDataType> | undefined,
      option_index: number,
  ) => {
    element_data?.splice(option_index, 1);
    setValue(`elements.${elementIndex}.element_data`, element_data);
    trigger(`elements.${elementIndex}`);
  };

  const optionAddActionOnClick = (
      e: React.MouseEvent,
      elementIndex: number,
      element_data: Array<CreatableElementDataType> | undefined,
  ) => {
    element_data?.push({ label: "", value: "" });
    setValue(`elements.${elementIndex}.element_data`, element_data);
    trigger(`elements.${elementIndex}`);
  };

  const submitHandler = (data: any) => {
    mutate(data);
  };

  return (
      <div className="flex-1 bg-white">
        <div className="px-5 pl-64 ">
          <div className="w-full">
            <div className="block rounded-lg p-6">
              <h1 className="text-xl font-semibold mb-4 text-gray-900 dark:text-gray-100">
                {t("component_information")}
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
                  <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="name" />
                </div>
                <div className="mb-4">
                  <InputField
                      label={t("identifier")}
                      placeholder={t("identifier")}
                      name="identifier"
                      register={register("identifier")}
                  />
                  <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="identifier" />
                </div>

                {elements.map((element, index) => {
                  return (
                      <div
                          key={`field-${index}-key`}
                          className="block ring-1 ring-gray-300 mb-4 rounded p-5"
                      >
                        <div className="flex w-full">
                          <button
                              type="button"
                              onClick={() => {
                                deleteElementOnClick(index);
                              }}
                              className="ml-auto"
                          >
                            <TrashIcon className="w-4 h-4" />
                          </button>
                        </div>

                        <div className="flex w-full">
                          <div className="border-r p-5 w-1/3">
                            <div
                                onClick={(e) =>
                                    elementTypeButtonOnClick(
                                        index,
                                        AvoRedFieldTypesEnum.TEXT,
                                    )
                                }
                                className={`${
                                    element.element_type === AvoRedFieldTypesEnum.TEXT
                                        ? "bg-primary-300"
                                        : "bg-gray-300"
                                } ring-1 p-3 mt-3 rounded`}
                            >
                              {t("text")}
                            </div>
                            <div
                                onClick={(e) =>
                                    elementTypeButtonOnClick(
                                        index,
                                        AvoRedFieldTypesEnum.TEXTAREA,
                                    )
                                }
                                className={`${
                                    element.element_type === AvoRedFieldTypesEnum.TEXTAREA
                                        ? "bg-primary-300"
                                        : "bg-gray-300"
                                } ring-1 p-3 mt-3 rounded`}
                            >
                              {t("textarea")}
                            </div>
                            <div
                                onClick={(e) =>
                                    elementTypeButtonOnClick(
                                        index,
                                        AvoRedFieldTypesEnum.SELECT,
                                    )
                                }
                                className={`${
                                    element.element_type === AvoRedFieldTypesEnum.SELECT
                                        ? "bg-primary-300"
                                        : "bg-gray-300"
                                } ring-1 p-3 mt-3 rounded`}
                            >
                              {t("select")}
                            </div>
                          </div>

                          <div className="p-3 w-2/3">
                            <div className="mt-3">
                              <Controller
                                  name={`elements.${index}.element_type`}
                                  render={({ field: element }) => {
                                    return <>{t!('element_type')}: {element.value}</>;
                                  }}
                                  control={control}
                              />
                              <InputField
                                  name={`elements.${index}.element_type`}
                                  type="hidden"
                                  register={register(`elements.${index}.element_type`)}
                              />
                            </div>
                            <div className="mt-3">
                              <InputField
                                  name={`elements.${index}.name`}
                                  register={register(`elements.${index}.name`)}
                                  label={t("element_name")}
                                  placeholder={t("element_name")}
                              />
                            </div>
                            <div className="mt-3">
                              <InputField
                                  name={`elements.${index}.identifier`}
                                  register={register(`elements.${index}.identifier`)}
                                  label={t("element_identifier")}
                                  placeholder={t("element_identifier")}
                              />
                            </div>
                            <Controller
                                name={`elements.${index}`}
                                control={control}
                                render={({ field: element }) => {
                                  return element.value.element_type ===
                                  AvoRedFieldTypesEnum.SELECT ? (
                                      <div className="mt-3">
                                        <div className="w-full">
                                          <h6 className="font-semibold">
                                            {t("element_options")}
                                          </h6>
                                        </div>

                                        {element.value.element_data?.map(
                                            (element_data, element_data_index) => {
                                              return (
                                                  <div className="flex">
                                                    <div className="w-1/2">
                                                      <InputField
                                                          label={t("element_option_label")}
                                                          placeholder={t(
                                                              "element_option_label",
                                                          )}
                                                          register={register(
                                                              `elements.${index}.element_data.${element_data_index}.label`,
                                                          )}
                                                      />
                                                    </div>

                                                    <div className="w-1/2 ml-3">
                                                      <label
                                                          htmlFor="hs-inline-leading-pricing-select-label"
                                                          className="text-sm text-gray-600"
                                                      >
                                                        {t("element_option_value")}
                                                      </label>
                                                      <div className="relative">
                                                        <InputField
                                                            placeholder={t(
                                                                "element_option_value",
                                                            )}
                                                            register={register(
                                                                `elements.${index}.element_data.${element_data_index}.value`,
                                                            )}
                                                        />
                                                        <div
                                                            onClick={(e: React.MouseEvent) =>
                                                                optionDeleteActionOnClick(
                                                                    e,
                                                                    index,
                                                                    element.value.element_data,
                                                                    element_data_index,
                                                                )
                                                            }
                                                            className="absolute inset-y-0 end-0 z-40 flex items-center text-gray-500"
                                                        >
                                                          <TrashIcon className="text-primary-500 w-4 h-4 mr-2" />
                                                        </div>
                                                      </div>
                                                    </div>
                                                  </div>
                                              );
                                            },
                                        )}
                                        <div
                                            className="mt-4 flex justify-center ring-1 ring-gray-300 rounded p-1"
                                            onClick={(e: React.MouseEvent) =>
                                                optionAddActionOnClick(
                                                    e,
                                                    index,
                                                    element.value.element_data,
                                                )
                                            }
                                        >
                                          <button
                                              className="flex items-center"
                                              type="button"
                                          >
                                            <PlusIcon className="text-primary-500 h-6 w-6" />
                                            <span className="text-sm ml-1 text-primary-500">
                                      {t("add_option")}
                                    </span>
                                          </button>
                                        </div>
                                      </div>
                                  ) : (
                                      <></>
                                  );
                                }}
                            />
                          </div>
                        </div>
                      </div>
                  );
                })}

                <div className="mb-4 flex items-center justify-center ring-1 ring-gray-300 rounded p-3">
                  <button
                      type="button"
                      className="flex"
                      onClick={addElementOnClick}
                  >
                    <PlusIcon className="text-primary-500 h-6 w-6" />
                    <span className="text-sm ml-1 text-primary-500">
                    {t("add_element")}
                  </span>
                  </button>
                </div>

                <div className="mt-5 flex items-center">
                  <button
                      type="submit"
                      className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                  >
                    {t("save")}
                  </button>
                  <Link
                      to={`/admin/component`}
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
});

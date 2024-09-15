import React from "react";
import { Link } from "react-router-dom";
import { PlusIcon, TrashIcon } from "@heroicons/react/24/solid";
import InputField from "../../components/InputField";
import { useComponentAll } from "./hooks/useComponentAll";
import { useStorePage } from "./hooks/useStorePage";
import { useTranslation } from "react-i18next";
import { Controller, useFieldArray, useForm } from "react-hook-form";
import { joiResolver } from "@hookform/resolvers/joi";
import { usePageCreateSchema } from "./schemas/page.create.schema";
import ErrorMessage from "../../components/ErrorMessage";
import slug from "slug";
import {
  AvoRedDataType,
  AvoRedFieldType,
  CreatableFieldType,
  CreatablePageType,
} from "../../types/page/CreatablePageType";

function PageCreate() {
  // const [isComponentTableModalOpen, setIsComponentTableModalOpen] =
  //   useState(false);
  const [t] = useTranslation("global");

  const {
    control,
    register,
    handleSubmit,
    formState: { errors },
    setValue,
    trigger,
  } = useForm<CreatablePageType>({
    resolver: joiResolver(usePageCreateSchema(), { allowUnknown: true }),
  });

  const { fields, append, remove } = useFieldArray({
    control,
    name: "page_fields", //rename fields
  });

  // const setSelectedElementDataOption = (
  //   selected: Array<string>,
  //   pageComponentIndex: number,
  //   componentElementIndex: number,
  // ) => {
  //   let val = selected.pop() ?? "";
  //
  //   // @todo fix this onese
  //   // Currently all data types is text but will be depend on element_type???
  //   setValue(
  //     `components_content.${pageComponentIndex}.elements.${componentElementIndex}.element_data_type`,
  //     "TEXT",
  //   );
  //   setValue(
  //     `components_content.${pageComponentIndex}.elements.${componentElementIndex}.element_content`,
  //     val,
  //   );
  // };

  // const getSelectElementDataOptionCurrentValue = (
  //   pageComponentIndex: number,
  //   componentElementIndex: number,
  // ) => {
  //   let val = getValues(
  //     `components_content.${pageComponentIndex}.elements.${componentElementIndex}.element_content`,
  //   );
  //   if (val) {
  //     return [val];
  //   }
  //   return [];
  // };

  const component_all_api_response = useComponentAll();
  // const components = _.get(component_all_api_response, "data.data", []);
  const { mutate, error } = useStorePage();

  // const renderComponentElementType = (
  //   componentElement: ICreatablePageComponentElementModel,
  //   pageComponentIndex: number,
  //   componentElementIndex: number,
  // ) => {
  //   switch (componentElement.element_type) {
  //     case "textarea":
  //       return (
  //         <div className="p-3">
  //           <textarea
  //             {...register(
  //               `components_content.${pageComponentIndex}.elements.${componentElementIndex}.element_content`,
  //             )}
  //           ></textarea>
  //           <label
  //             htmlFor={componentElement.identifier}
  //             className="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300"
  //           >
  //             {componentElement.name}
  //           </label>
  //         </div>
  //       );
  //     case "select":
  //       return (
  //         <div className="p-3">
  //           <AvoRedMultiSelectField
  //             label="test dropdown"
  //             selectedOption={getSelectElementDataOptionCurrentValue(
  //               pageComponentIndex,
  //               componentElementIndex,
  //             )}
  //             options={componentElement.element_data ?? []}
  //             onChangeSelectedOption={(val: Array<string>) =>
  //               setSelectedElementDataOption(
  //                 val,
  //                 pageComponentIndex,
  //                 componentElementIndex,
  //               )
  //             }
  //           ></AvoRedMultiSelectField>
  //           <label
  //             htmlFor={componentElement.identifier}
  //             className="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300"
  //           >
  //             {componentElement.name}
  //           </label>
  //         </div>
  //       );
  //     case "radio":
  //       return (
  //         <div className="p-3">
  //           <input
  //             id={componentElement.identifier}
  //             type="radio"
  //             value={componentElement.identifier}
  //             className="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 focus:ring-primary-500 dark:focus:ring-primary-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
  //           />
  //           <label
  //             htmlFor={componentElement.identifier}
  //             className="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300"
  //           >
  //             {componentElement.name}
  //           </label>
  //         </div>
  //       );
  //     case "text":
  //     default:
  //       return (
  //         <div>
  //           <InputField
  //             name={`components_content.${pageComponentIndex}.elements.${componentElementIndex}.element_content`}
  //             label={componentElement.name}
  //             register={register(
  //               `components_content.${pageComponentIndex}.elements.${componentElementIndex}.element_content`,
  //             )}
  //           />
  //           <InputField
  //             type="hidden"
  //             value="TEXT"
  //             register={register(
  //               `components_content.${pageComponentIndex}.elements.${componentElementIndex}.element_data_type`,
  //             )}
  //           />
  //         </div>
  //       );
  //   }
  // };

  // const renderComponentElement = (
  //   componentElement: ICreatablePageComponentElementModel,
  //   pageComponentIndex: number,
  //   componentElementIndex: number,
  // ) => {
  //   return (
  //     <div
  //       className="ring-1 my-2 ring-gray-200"
  //       key={componentElement.identifier}
  //     >
  //       {JSON.stringify(componentElement)}
  //       {renderComponentElementType(
  //         componentElement,
  //         pageComponentIndex,
  //         componentElementIndex,
  //       )}
  //     </div>
  //   );
  // };

  // const componentSelected = (e: React.MouseEvent, componentId: string) => {
  //   e.preventDefault();
  //   const selectedComponent = components.find(
  //     (component: IComponentModel) => component.id === componentId,
  //   );
  //   setIsComponentTableModalOpen(false);
  //
  //   append(selectedComponent);
  // };

  const addPageFieldOnClick = (e: any) => {
    e.preventDefault();
    append({
      name: "",
      identifier: "",
      data_type: AvoRedDataType.TEXT,
      field_type: AvoRedFieldType.TEXT,
      field_content: ""
    });
  };

  const deletePageFieldOnClick = (e: any, index: number) => {
    remove(index);
  };

  // const pageModelOnClose = () => {
  //   setIsComponentTableModalOpen(false);
  // };

  // const renderComponent = (
  //   pageComponent: ICreatablePageComponentModel,
  //   pageComponentIndex: number,
  // ) => {
  //   return (
  //     <div
  //       key={pageComponent.id}
  //       className="my-5 ring-1 ring-gray-200 rounded p-3"
  //     >
  //       <div>component name: {pageComponent.name}</div>
  //       <div>component identifier: {pageComponent.identifier}</div>
  //       <div>
  //         Component Fields
  //         {pageComponent.elements.map(
  //           (componentElement, componentElementIndex) => {
  //             return renderComponentElement(
  //               componentElement,
  //               pageComponentIndex,
  //               componentElementIndex,
  //             );
  //           },
  //         )}
  //       </div>
  //     </div>
  //   );
  // };

  const submitHandler = async (data: CreatablePageType) => {
    // console.log(data)
    mutate(data);
  };

  const onNameChange = (e: React.KeyboardEvent<HTMLInputElement>) => {
    setValue("identifier", slug(e.currentTarget.value || ""));
  };

  const fieldTypeOnClick = async (
    e: any,
    index: number,
    field_type: AvoRedFieldType,
    data_type: AvoRedDataType,
  ) => {
    e.preventDefault();
    setValue(`page_fields.${index}.field_type`, field_type);
    setValue(`page_fields.${index}.data_type`, data_type);
    await trigger(`page_fields.${index}`);
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
                    className="relative hover:ring-1 ring-primary-300 rounded mb-5 flex mt-5 py-3 w-full"
                  >
                    <Controller
                      name={`page_fields.${index}`}
                      render={({field: page_field}) => {
                        return (
                          <>
                            <div
                              onClick={(e) => deletePageFieldOnClick(e, index)}
                              className="absolute right-0 top-0 mt-3 mr-5"
                            >
                              <TrashIcon className="w-4 h-4" />
                            </div>
                            <div className="flex mt-3 w-full justify-center">
                              <div className="flex-1 p-3">
                                <div className="p-3 bg-gray-200 rounded">
                                  <div className="mb-4 mt-3 w-full flex justify-center">
                                    <div className="w-1/2">
                                      <InputField
                                        label={t("field_name")}
                                        placeholder={t("field_name")}
                                        register={register(
                                          `page_fields.${index}.name`,
                                        )}
                                      />
                                    </div>
                                    <div className="w-1/2 ml-3">
                                      <InputField
                                        label={t("field_identifier")}
                                        placeholder={t("field_identifier")}
                                        register={register(
                                          `page_fields.${index}.identifier`,
                                        )}
                                      />
                                    </div>
                                  </div>
                                  <InputField
                                    label={t("hidden")}
                                    placeholder={t("data_type")}
                                    register={register(
                                      `page_fields.${index}.data_type`,
                                    )}
                                  />
                                  <InputField
                                    label={t("hidden")}
                                    placeholder={t("field_type")}
                                    register={register(
                                      `page_fields.${index}.field_type`,
                                    )}
                                  />
                                  <div className="mb-4">
                                    <InputField
                                      label={t("field_content")}
                                      placeholder={t("field_content")}
                                      register={register(`page_fields.${index}.field_content`)}
                                    />
                                  </div>
                                </div>
                              </div>
                              <div className="w-96 border-l p-3 mr-auto">
                                <div
                                  onClick={(e) =>
                                    fieldTypeOnClick(
                                      e,
                                      index,
                                      AvoRedFieldType.TEXT,
                                      AvoRedDataType.TEXT,
                                    )
                                  }
                                  className={`${page_field.value.field_type === AvoRedFieldType.TEXT ? "bg-primary-300" : ""} 
                              ring-1 ring-gray-300 hover:cursor-pointer hover:ring-primary-300 p-3 rounded`}
                                >
                                  TEXT FIELD
                                </div>
                                <div
                                  onClick={(e) =>
                                    fieldTypeOnClick(
                                      e,
                                      index,
                                      AvoRedFieldType.TEXTAREA,
                                      AvoRedDataType.TEXT,
                                    )
                                  }
                                  className={`${page_field.value.field_type === AvoRedFieldType.TEXTAREA ? "bg-primary-300" : ""} 
                              ring-1 mt-2 ring-gray-300 hover:cursor-pointer hover:ring-primary-300 p-3 rounded`}
                                >
                                  TEXTAREA FIELD
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

              {/*<div>*/}
              {/*  {components_content.map(*/}
              {/*    (pageComponent, pageComponentIndex) => {*/}
              {/*      return renderComponent(pageComponent, pageComponentIndex);*/}
              {/*    },*/}
              {/*  )}*/}
              {/*</div>*/}

              <div className="mb-4 flex items-center justify-center ring-1 ring-gray-400 rounded p-5">
                <button
                  type="button"
                  className="flex"
                  onClick={(e) => addPageFieldOnClick(e)}
                >
                  <PlusIcon className="text-primary-500 h-6 w-6" />
                  <span className="text-sm ml-1 text-primary-500">
                    {t("add_page_field")}
                  </span>
                </button>
              </div>

              {/*<AvoredModal*/}
              {/*    closeModal={pageModelOnClose}*/}
              {/*    modal_header={t("select_element")}*/}
              {/*    modal_body={*/}
              {/*        <div className="text-primary-500">*/}
              {/*            Test*/}
              {/*        </div>*/}
              {/*    }*/}
              {/*    isOpen={isComponentTableModalOpen}*/}
              {/*></AvoredModal>*/}
              <hr />
              <div className="mt-5  flex items-center">
                <button
                  type="submit"
                  className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                >
                  {t("save")}
                </button>
                <Link
                  to="/admin/page"
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

import React, { useState } from "react";
import { Link, useParams } from "react-router-dom";
import {PlusIcon, TrashIcon} from "@heroicons/react/24/solid";
import AvoredModal from "../../components/AvoredModal";
import InputField from "../../components/InputField";
import _ from "lodash";
import { useComponentAll } from "./hooks/useComponentAll";
import { useGetPage } from "./hooks/useGetPage";
import { useUpdatePage } from "./hooks/useUpdatePage";
import { useTranslation } from "react-i18next";
import {Controller, useFieldArray, useForm} from "react-hook-form";
import { joiResolver } from "@hookform/resolvers/joi";
import PageComponentTable from "./PageComponentTable";
import IEditablePage, {
    IEditablePageComponentFieldModel,
    IEditablePageComponentModel,
} from "../../types/page/IEditablePage";
import { usePageEditSchema } from "./schemas/page.edit.schema";
import AvoRedMultiSelectField from "../../components/AvoRedMultiSelectField";
import {usePagePutSchema} from "./schemas/page.put.schema";
import {PutPageIdentifierType} from "../../types/page/PutPageIdentifierType";
import {usePutPageIdentifier} from "./hooks/usePutPageIdentifier";
import {EditablePageType} from "../../types/page/EditablePageType";
import {AvoRedDataType, AvoRedFieldType} from "../../types/page/CreatablePageType";

function PageEdit() {
    const [isComponentTableModalOpen, setIsComponentTableModalOpen] =
        useState(false);
    const params = useParams();
    const [t] = useTranslation("global")
    const [isEditableIdentifier, setIsEditableIdentifier] = useState<boolean>(true)

    const component_all_api_response = useComponentAll();
    const components = _.get(component_all_api_response, "data.data", []);

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
        trigger
    } = useForm<EditablePageType>({
        resolver: joiResolver(usePageEditSchema(), { allowUnknown: true }),
        values,
    });


    const deletePageFieldOnClick = (e: any, index: number) => {
        remove(index);
    };

    const {
        register: putPageRegister,
        getValues: getPageIdentifierValue
    } = useForm<PutPageIdentifierType>({
        resolver: joiResolver(usePagePutSchema(), {allowUnknown: true}),
        values: {
            identifier: data?.data.page_model.identifier
        }
    });

    const {mutate: putPageIdentifierMutate} = usePutPageIdentifier(params.page_id ?? "")

    const editableIdentifierOnClick = (() => {
        setIsEditableIdentifier(false)
    })
    const saveIdentifierOnClick = (() => {
        putPageIdentifierMutate({identifier: getPageIdentifierValue('identifier')})
        setIsEditableIdentifier(true)
    })

    const cancelIdentifierOnClick = (() => {
        setIsEditableIdentifier(true)
    })

    const { fields, append, remove } = useFieldArray({
        control,
        name: "page_fields",
    });

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


    const addComponentOnClick = (e: React.MouseEvent<HTMLElement>) => {
        e.preventDefault()
        console.log("test")
        setIsComponentTableModalOpen(true);
    };

    const pageModelOnClose = () => {
        setIsComponentTableModalOpen(false);
    };

    const submitHandler = async (data: EditablePageType) => {
        mutate(data);
    };

    return (
      <div className="flex-1 bg-white">
        <div className="px-5 pl-64 ">
          <div className="w-full">
            <div className="block rounded-lg p-6">
              <h1 className="text-xl font-semibold mb-4 text-gray-900">
                {t("page_information")}
              </h1>
              {/*<p className="text-gray-600 dark:text-gray-300 mb-6">Use a permanent address where you can*/}
              {/*    receive mail.</p>*/}
              <form onSubmit={handleSubmit(submitHandler)}>
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

                <div className="mb-4 flex items-center justify-center ring-1 ring-gray-400 rounded p-5">
                  <button
                    type="button"
                    className="flex"
                    onClick={e => addComponentOnClick(e)}
                  >
                    <PlusIcon className="text-primary-500 h-6 w-6" />
                    <span className="text-sm ml-1 text-primary-500">
                      {t("add_component")}
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

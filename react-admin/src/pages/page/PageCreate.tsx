import React, {useState} from "react";
import {Link} from "react-router-dom";
import {Cog8ToothIcon, MinusIcon, PlusIcon, TrashIcon} from "@heroicons/react/24/solid";
import InputField from "../../components/InputField";
import {useStorePage} from "./hooks/useStorePage";
import {useTranslation} from "react-i18next";
import {Controller, useFieldArray, useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import {usePageCreateSchema} from "./schemas/page.create.schema";
import ErrorMessage from "../../components/ErrorMessage";
import "easymde/dist/easymde.min.css";
import slug from "slug";
import {
  AvoRedPageFieldData,
  AvoRedPageFieldSelectFieldDataOptions,
  CreatableFieldType,
  CreatablePageType,
} from "../../types/page/CreatablePageType";
import {AvoRedPageDataType, AvoRedPageFieldType} from "../../types/page/IPageModel";
import AvoredModal from "../../components/AvoredModal";
import _ from "lodash";
import AvoRedButton from "../../components/AvoRedButton";
import SimpleMdeReact from "react-simplemde-editor";

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
  } = useForm<CreatablePageType>({
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

    append({
      name: "",
      identifier: "",
      data_type: AvoRedPageDataType.TEXT,
      field_type: AvoRedPageFieldType.TEXT,
      field_content: ""
    });
    await trigger("page_fields");

    setCurrentIndex(max_index);

    setIsOpen(true);
  };

  const deletePageFieldOnClick = (e: any, index: number) => {
    e.preventDefault();
    remove(index);
    setCurrentIndex(0);
  };

  const submitHandler = async (data: CreatablePageType) => {
    // console.log(data)
    mutate(data);
  };

  const onNameChange = (e: React.KeyboardEvent<HTMLInputElement>) => {
    setValue("identifier", slug(e.currentTarget.value || ""));
  };

  const onPageFieldChange = async (
      index: number,
      field_type: AvoRedPageFieldType,
      data_type: AvoRedPageDataType,
  ) => {
    setValue(`page_fields.${index}.field_type`, field_type);
    setValue(`page_fields.${index}.data_type`, data_type);

    if (field_type === AvoRedPageFieldType.SELECT) {
      setValue(`page_fields.${index}.field_data.select_field_options`, []);
    }

    switch (field_type) {
      case AvoRedPageFieldType.SELECT:
          const empty_option: AvoRedPageFieldSelectFieldDataOptions  = {
            label: '',
            value: ''
          };

          const options: AvoRedPageFieldData = {
              select_field_options: []
          };
          if (typeof options.select_field_options == 'undefined') {
            options.select_field_options = []
          }

          options.select_field_options.push(empty_option)

        setValue(`page_fields.${index}.field_data`, options);

        break;
      default:
        break;
    }


    await trigger(`page_fields.${index}`);
  };

  const optionAddOnClick = (async (e: React.MouseEvent<HTMLButtonElement>, field_index: number) => {
    e.preventDefault()
    const page_field: CreatableFieldType = getValues(`page_fields.${field_index}`);
    const empty_option: AvoRedPageFieldSelectFieldDataOptions  = {
      label: '',
      value: ''
    };

    page_field.field_data?.select_field_options.push(empty_option)

    await trigger("page_fields")
  })

  const optionRemoveOnClick = (async (e: React.MouseEvent<HTMLButtonElement>, field_index: number, option_index: number) => {
    e.preventDefault()
    const page_field: CreatableFieldType = getValues(`page_fields.${field_index}`);
    page_field.field_data?.select_field_options.splice(option_index, 1);

    await trigger(`page_fields.${field_index}`)
  })

  const optionLabelOnChange = (async (e: any, field_index: number, option_index: number) => {
    setValue(`page_fields.${field_index}.field_data.select_field_options.${option_index}.label`, e.target.value)
    await trigger("page_fields")
  })

  const optionValueOnChange = (async (e: any, field_index: number, option_index: number) => {
    setValue(`page_fields.${field_index}.field_data.select_field_options.${option_index}.value`, e.target.value)
    await trigger("page_fields")
  })

  const textEditorOnChange = ((value: string, field_index: number) => {
    setValue(`page_fields.${field_index}.field_content`, value)
  })
  const renderFieldData = (current_index: number) => {
    const page_field: CreatableFieldType = getValues(
      `page_fields.${current_index}`,
    );

    switch (page_field.field_type) {
      case AvoRedPageFieldType.SELECT:
          return (
              <>
                {page_field.field_data?.select_field_options.map((option, option_index) => {
                  return (
                    <div key={option_index} className="block mt-3 w-full">
                      <div className="flex w-full items-center">
                        <div className="w-1/2">
                          <div className="block">
                            <input
                              value={option.label}
                              onChange={(e) =>
                                optionLabelOnChange(
                                  e,
                                  current_index,
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
                                    current_index,
                                    option_index,
                                  )
                                }
                                placeholder={t("value")}
                                className="appearance-none rounded-md ring-1 ring-gray-400
                                    relative border-0 block w-full px-3 py-2 placeholder-gray-500 text-gray-900
                                    active::ring-primary-500
                                    focus:ring-primary-500 focus:outline-none focus:z-10
                                    disabled:bg-gray-200 disabled:opacity-70
                                    sm:text-sm "
                              />
                            </div>
                            <div>
                              {getValues(
                                `page_fields.${current_index}.field_data.select_field_options`,
                              ).length ===
                              option_index + 1 ? (
                                <>
                                  <button
                                    onClick={(e) =>
                                      optionAddOnClick(e, currentIndex)
                                    }
                                    className="ml-2"
                                  >
                                    <PlusIcon className="w-5 h-5" />
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
                                    <MinusIcon className="w-5 h-5" />
                                  </button>
                                </>
                              )}
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  );
                })}
              </>
          )
      default:
        return (
            <></>
        )
    }

  };

  const renderField = (field: CreatableFieldType, index: number) => {
    switch (field.field_type) {
      case AvoRedPageFieldType.TEXTAREA:
        return (
            <div className="mb-4">
              <label className="text-sm text-gray-600">
                {t!("field_content")}
            </label>
            <textarea
              className="w-full rounded"
              {...register(`page_fields.${index}.field_content`)}
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
                      minHeight: '300px'
                    }}
                    value={`# test`}
                    onChange={contentValue => textEditorOnChange(contentValue, index)}
                />
              </div>
            </div>
        );
      case AvoRedPageFieldType.SELECT:
        return (
          <div className="mb-4">
            <label className="text-sm text-gray-600">
              {t!("field_content")}
            </label>
            
            <select {...register(`page_fields.${index}.field_content`)}
                    className="w-full rounded border-0 ring-1 ring-primary-400 outline-none appearance-none">
              {field.field_data?.select_field_options.map((option) => {
                return <option key={option.value} value={option.value}>{option.label}</option>;
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
              register={register(`page_fields.${index}.field_content`)}
            />
          </div>
        );
      default:
        return (
          <div className="mb-4">
            <InputField
              label={t("field_content")}
              placeholder={t("field_content")}
              register={register(`page_fields.${index}.field_content`)}
            />
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
                <AvoredModal
                  closeModal={() => setIsOpen(false)}
                  modal_header={`Page Field`}
                  modal_body={
                    <div className="block">
                      <div className="flex w-full">
                        <div className="flex-1 pr-3">
                          <div className="mb-3">
                            <InputField
                              placeholder={t("page_field_name")}
                              label={t("page_field_name")}
                              register={register(
                                `page_fields.${currentIndex}.name`,
                              )}
                            />
                          </div>
                          <div className="mb-3">
                            <InputField
                              placeholder={t("page_field_identifier")}
                              label={t("page_field_identifier")}
                              register={register(
                                `page_fields.${currentIndex}.identifier`,
                              )}
                            />
                          </div>

                          <div className="w-full">
                            {renderFieldData(currentIndex)}
                          </div>
                        </div>
                        <div className="ml-auto">
                          <div className="w-64 border-l p-3 mr-auto">
                            <div
                              onClick={() =>
                                onPageFieldChange(
                                  currentIndex,
                                  AvoRedPageFieldType.TEXT,
                                  AvoRedPageDataType.TEXT,
                                )
                              }
                              className={`${getValues(`page_fields.${currentIndex}.field_type`) === AvoRedPageFieldType.TEXT ? "bg-primary-200" : "bg-gray-300"} 
                    ring-1 ring-gray-300 hover:cursor-pointer hover:ring-primary-300 p-3 rounded`}
                            >
                              {t("text_field")}
                            </div>
                            <div
                              onClick={() =>
                                onPageFieldChange(
                                  currentIndex,
                                  AvoRedPageFieldType.TEXTAREA,
                                  AvoRedPageDataType.TEXT,
                                )
                              }
                              className={`${getValues(`page_fields.${currentIndex}.field_type`) === AvoRedPageFieldType.TEXTAREA ? "bg-primary-200" : "bg-gray-300"}  
                  ring-1 mt-2 ring-gray-300 hover:cursor-pointer hover:ring-primary-300 p-3 rounded`}
                            >
                              {t("textarea_field")}
                            </div>
                            <div
                              onClick={() =>
                                onPageFieldChange(
                                  currentIndex,
                                  AvoRedPageFieldType.SELECT,
                                  AvoRedPageDataType.TEXT,
                                )
                              }
                              className={`${getValues(`page_fields.${currentIndex}.field_type`) === AvoRedPageFieldType.SELECT ? "bg-primary-200" : "bg-gray-300"}  
                  ring-1 mt-2 ring-gray-300 hover:cursor-pointer hover:ring-primary-300 p-3 rounded`}
                            >
                              {t("select_field")}
                            </div>

                            <div
                              onClick={() =>
                                onPageFieldChange(
                                  currentIndex,
                                  AvoRedPageFieldType.TextEditor,
                                  AvoRedPageDataType.TEXT,
                                )
                              }
                              className={`${getValues(`page_fields.${currentIndex}.field_type`) === AvoRedPageFieldType.TextEditor ? "bg-primary-200" : "bg-gray-300"}  
                  ring-1 mt-2 ring-gray-300 hover:cursor-pointer hover:ring-primary-300 p-3 rounded`}
                            >
                              {t("text_editor_field")}
                            </div>
                          </div>
                        </div>
                      </div>
                      <hr className="mt-3" />
                      <div className="mt-3">
                        <div className="flex">
                          <div>
                            <AvoRedButton
                              onClick={() => setIsOpen(false)}
                              className="bg-primary-500"
                              label={t("create_page_field")}
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
                  isOpen={isOpen}
                ></AvoredModal>
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

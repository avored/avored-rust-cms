import AvoredModal from "../../components/AvoredModal";
import InputField from "../../components/InputField";
import { useTranslation } from "react-i18next";
import { AvoRedPageDataType, AvoRedPageFieldType } from "../../types/page/IPageModel";
import { AvoRedPageFieldData, AvoRedPageFieldSelectFieldDataOptions,  SaveFieldType,  SavePageType } from "../../types/page/CreatablePageType";
import { MinusIcon, PlusIcon } from "@heroicons/react/24/solid";
import AvoRedButton from "../../components/AvoRedButton";
import { UseFormGetValues, UseFormRegister, UseFormSetValue, UseFormTrigger } from "react-hook-form";

type PageFieldProps = {
  register: UseFormRegister<SavePageType>
  currentIndex: number
  getValues: UseFormGetValues<SavePageType>
  setValue: UseFormSetValue<SavePageType>
  trigger: UseFormTrigger<SavePageType>
  setIsOpen: React.Dispatch<React.SetStateAction<boolean>>
  isOpen: boolean
}

export const PageFieldModal = (({
  register,
  currentIndex,
  getValues,
  setValue,
  trigger,
  setIsOpen,
  isOpen
}: PageFieldProps) => {
    
    const [t] = useTranslation("global");

    
  const optionLabelOnChange = async (
    e: any,
    field_index: number,
    option_index: number,
  ) => {
    setValue(
      `page_fields.${field_index}.field_data.select_field_options.${option_index}.label`,
      e.target.value,
    );
    await trigger("page_fields");
  };

  const optionValueOnChange = async (
    e: any,
    field_index: number,
    option_index: number,
  ) => {
    setValue(
      `page_fields.${field_index}.field_data.select_field_options.${option_index}.value`,
      e.target.value,
    );
    await trigger("page_fields");
  };

  
  const optionAddOnClick = async (
    e: React.MouseEvent<HTMLButtonElement>,
    field_index: number,
  ) => {
    e.preventDefault();
    const page_field: SaveFieldType = getValues(
      `page_fields.${field_index}`,
    );
    const empty_option: AvoRedPageFieldSelectFieldDataOptions = {
      label: "",
      value: "",
    };

    page_field.field_data?.select_field_options.push(empty_option);

    await trigger("page_fields");
  };

  const optionRemoveOnClick = async (
    e: React.MouseEvent<HTMLButtonElement>,
    field_index: number,
    option_index: number,
  ) => {
    e.preventDefault();
    const page_field: SaveFieldType = getValues(
      `page_fields.${field_index}`,
    );
    page_field.field_data?.select_field_options.splice(option_index, 1);

    await trigger(`page_fields.${field_index}`);
  };

  const renderFieldData = (current_index: number) => {
    const page_field: SaveFieldType = getValues(
      `page_fields.${current_index}`,
    );
      
  
      switch (page_field.field_type) {
        case AvoRedPageFieldType.SELECT:
          return (
            <>
              {page_field.field_data?.select_field_options.map(
                (option, option_index) => {
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
                },
              )}
            </>
          );
        default:
          return <></>;
      }
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
          const empty_option: AvoRedPageFieldSelectFieldDataOptions = {
            label: "",
            value: "",
          };
  
          const options: AvoRedPageFieldData = {
            select_field_options: [],
          };
          if (typeof options.select_field_options == "undefined") {
            options.select_field_options = [];
          }
          options.select_field_options.push(empty_option);
          setValue(`page_fields.${index}.field_data`, options);
  
          break;
        default:
          break;
      }
  
      await trigger(`page_fields.${index}`);
    };
  
    return (
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
    );
})

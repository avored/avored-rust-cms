import {
  UseFormGetValues,
  UseFormRegister,
  UseFormSetValue,
  UseFormTrigger,
} from "react-hook-form";
import AvoredModal from "../../components/AvoredModal";
import AvoRedButton from "../../components/AvoRedButton";
import { useTranslation } from "react-i18next";
import InputField from "../../components/InputField";
import React from "react";
import slug from "slug";
import {
  CollectionFieldDataType,
  SavableCollectionType, CollectionFieldFieldType
} from "../../types/collection/CreatableCollectionType";

type CollectionFieldProps = {
  register: UseFormRegister<SavableCollectionType>;
  currentIndex: number;
  getValues: UseFormGetValues<SavableCollectionType>;
  setValue: UseFormSetValue<SavableCollectionType>;
  trigger: UseFormTrigger<SavableCollectionType>;
  setIsOpen: React.Dispatch<React.SetStateAction<boolean>>;
  isOpen: boolean;
};

export const CollectionFieldModal = ({
  register,
  currentIndex,
  getValues,
  setValue,
  trigger,
  setIsOpen,
  isOpen,
}: CollectionFieldProps) => {
  const [t] = useTranslation("global");

  const onContentFieldChange = async (
    index: number,
    field_type: CollectionFieldFieldType,
    data_type: CollectionFieldDataType
  ) => {
    setValue(`collection_fields.${index}.field_type`, field_type);
    setValue(`collection_fields.${index}.data_type`, data_type);
    await trigger(`collection_fields.${index}`);
  };

  const contentFieldNameInputChange = ((e: React.KeyboardEvent<HTMLInputElement>, index: number) => {
    e.stopPropagation();

    setValue(`collection_fields.${index}.identifier`, slug(e.currentTarget.value));
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
                  register={register(`collection_fields.${currentIndex}.name`)}
                />
              </div>
              <div className="mb-3">
                <InputField
                  placeholder={t("identifier")}
                  label={t("identifier")}
                  register={register(
                    `collection_fields.${currentIndex}.identifier`
                  )}
                />
              </div>

              {/* <div className="w-full">{renderFieldData(currentIndex)}</div> */}
            </div>

            <div className="ml-auto">
              <div className="w-64 border-l p-3 mr-auto">
                <div
                  onClick={() =>
                    onContentFieldChange(
                      currentIndex,
                      CollectionFieldFieldType.TEXT,
                      CollectionFieldDataType.TEXT
                    )
                  }
                  className={`${getValues(`collection_fields.${currentIndex}.field_type`) === CollectionFieldFieldType.TEXT ? "bg-primary-200" : "bg-gray-300"} 
                        ring-1 ring-gray-300 hover:cursor-pointer hover:ring-primary-300 p-3 rounded`}
                >
                  {t("text_field")}
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
      modal_header={t("collection_field")}
      isOpen={isOpen}
    ></AvoredModal>
  );
};

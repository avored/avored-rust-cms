import InputField from "../../components/InputField";
import {useTranslation} from "react-i18next";
import {UseFormRegister} from "react-hook-form";
import {useState} from "react";

type RoleFieldsParams = {
    register: UseFormRegister<any>
}

export const RoleFields = (({register}: RoleFieldsParams) => {
    const [t] = useTranslation("global")
    const [isEditableIdentifier, setIsEditableIdentifier] = useState<boolean>(true)
    const editableIdentifierOnClick = (() => {
        setIsEditableIdentifier(false)
    })
    const saveIdentifierOnClick = (() => {
        console.log("todo call an api here")
        setIsEditableIdentifier(true)
    })
    const cancelIdentifierOnClick = (() => {
        setIsEditableIdentifier(true)
    })
    return (
      <>
        <div className="mb-4">
          <InputField
            label={t("name")}
            placeholder={t("name")}
            name="name"
            register={register("name")}
            autoFocus={true}
          />
        </div>
        <div className="mb-4">
          <InputField
            label={t("identifier")}
            placeholder={t("identifier")}
            name="identifier"
            register={register("identifier")}
            disabled={isEditableIdentifier}
          />
          <div
            className="mt-2"
          >
            {isEditableIdentifier ? (
              <>
                <span onClick={editableIdentifierOnClick}
                      className="text-xs text-blue-600 cursor-pointer">
                    {t("edit_identifier")}
                </span>
              </>
            ) : (
                <>
                    <button type="button" onClick={saveIdentifierOnClick}
                            className="text-xs text-blue-600 cursor-pointer">
                        {t('save')}
                    </button>
                    <button type="button" onClick={cancelIdentifierOnClick}
                            className="ml-3 text-xs text-blue-600 cursor-pointer">
                        {t('cancel')}
                    </button>
                </>
            )}
          </div>
        </div>
      </>
    );
})
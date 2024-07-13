import InputField from "../../components/InputField";
import {useTranslation} from "react-i18next";
import {useForm, UseFormRegister} from "react-hook-form";
import {useState} from "react";
import {usePutRoleIdentifier} from "./hooks/usePutRoleIdentifier";
import IEditableRole from "../../types/role/IEditableRole";
import {joiResolver} from "@hookform/resolvers/joi";
import {useRoleEditSchema} from "./schemas/role.edit.schema";
import {useRolePutSchema} from "./schemas/role.put.schema";
import {PutRoleIdentifierType} from "../../types/role/PutRoleIdentifierType";

type RoleFieldsParams = {
    register: UseFormRegister<any>;
    role_id: string;
}

export const RoleFields = (({register, role_id}: RoleFieldsParams) => {
    const [t] = useTranslation("global")
    const [isEditableIdentifier, setIsEditableIdentifier] = useState<boolean>(true)
    const {
        register: putRoleRegister,
        formState: {errors}
    } = useForm<PutRoleIdentifierType>({
        resolver: joiResolver(useRolePutSchema(), {allowUnknown: true})
    });
    const {mutate} = usePutRoleIdentifier(role_id)
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
            register={putRoleRegister("identifier")}
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
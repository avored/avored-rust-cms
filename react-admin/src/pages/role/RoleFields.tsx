import InputField from "../../components/InputField";
import {useTranslation} from "react-i18next";
import {UseFormRegister} from "react-hook-form";

type RoleFieldsParams = {
    register: UseFormRegister<any>
}

export const RoleFields = (({register}: RoleFieldsParams) => {
    const [t] = useTranslation("global")
    return (
        <>
            <div className="mb-4">
                <InputField
                    label={t('name')}
                    placeholder={t('name')}
                    name="name"
                    register={register("name")}
                    autoFocus={true}
                />
            </div>
            <div className="mb-4">
                <InputField
                    label={t('identifier')}
                    placeholder={t('identifier')}
                    name="identifier"
                    register={register("identifier")}
                />
            </div>
        </>
    )
})
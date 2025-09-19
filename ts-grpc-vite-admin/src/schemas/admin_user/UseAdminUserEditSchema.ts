import Joi from 'joi'
import {useTranslation} from "react-i18next"

export const UseAdminUserEditSchema = (() => {

    const [t] = useTranslation("global")
    return Joi.object({
        fullName : Joi.string().required().messages({
            'string.empty': t("empty_message", {attribute: t("full_name")}),
        })
    });
})

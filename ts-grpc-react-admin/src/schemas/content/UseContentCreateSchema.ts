import Joi from 'joi'
import {useTranslation} from "react-i18next"

export const UseContentCreateSchema = (() => {

    const [t] = useTranslation("global")
    return Joi.object({
        name : Joi.string().required().messages({
            'string.empty': t("empty_message", {attribute: t("name")}),
        })
    });
})

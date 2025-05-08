import Joi from 'joi'
import {useTranslation} from "react-i18next"

export const UseRoleCreateSchema = (() => {

    const [t] = useTranslation("global")
    return Joi.object({
        identifier : Joi.string().required().messages({
            'string.empty': t("empty_message", {attribute: t("identifier")}),
        }),
        name : Joi.string().required().messages({
            'string.empty': t("empty_message", {attribute: t("name")}),
        })
    });
})

import Joi from 'joi'
import {useTranslation} from "react-i18next"

export const useLoginSchema = (() => {

    const [t] = useTranslation("global")
    return Joi.object({
        email : Joi.string().email({ tlds: { allow: false } }).required().messages({
            'string.empty': t("empty_message", {attribute: t("email")}),
            'string.email': t("invalid_email"),
        }),
        password : Joi.string().required().messages({
            'string.empty': t("empty_message", {attribute: t("password")}),
        })
    });
})
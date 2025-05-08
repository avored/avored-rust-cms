import Joi from 'joi'
import {useTranslation} from "react-i18next"

export const UseAdminUserCreateSchema = (() => {

    const [t] = useTranslation("global")
    return Joi.object({
        full_name : Joi.string().required().messages({
            'string.empty': t("empty_message", {attribute: t("full_name")})
        }),
        email : Joi.string().email({ tlds: { allow: false } }).required().messages({
            'string.empty': t("empty_message", {attribute: t("email")}),
            'string.email': t("invalid_email"),
        }),
        password : Joi.string().required().messages({
            'string.empty': t("empty_message", {attribute: t("password")}),
        }),
        confirmation_password: Joi.any().valid(Joi.ref('password')).required().messages({
            'string.empty': t("empty_message", {attribute: t("confirmation_password")}),
            'any.only': t("confirm_password_does_not_match_with_current_password"),
        })

    });
})

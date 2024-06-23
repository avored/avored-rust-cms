import Joi from 'joi';
import {useTranslation} from "react-i18next";

export const useResetPasswordSchema = (() => {
    const [t] = useTranslation("global")
    return Joi.object({
        email: Joi.string().required().messages({
            'string.empty': t("empty_message", {attribute: t("email")})
        }),
        password: Joi.string().required().messages({
            'string.empty': t("empty_message", {attribute: t("password")})
        }),
        confirm_password: Joi.string().required().messages({
            'string.empty': t("empty_message", {attribute: t("confirmation_password")})
        }),
        token: Joi.string().required().messages({
            'string.empty': t("empty_message", {attribute: t("token")})
        }),
    })
})
// export const resetPasswordSchema = ;

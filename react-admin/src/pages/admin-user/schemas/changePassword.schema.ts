import Joi from 'joi';
import {useTranslation} from "react-i18next";

export const useChangePasswordSchema = (() => {
    const [t] = useTranslation("global")
    return Joi.object({
        current_password: Joi.string().required().messages({
            'string.empty': t("empty_message", {attribute: t("current_password")}),
        }),
        password: Joi.string().messages({
            'string.empty': t("empty_message", {attribute: t("password")}),
        }),
        confirm_password: Joi.string().equal(Joi.ref('password')).messages({
            'any.only': t("confirm_password_does_not_match_with_current_password"),
            'string.empty': t("empty_message", {attribute: t("confirm_password")}),
        })

    })
})
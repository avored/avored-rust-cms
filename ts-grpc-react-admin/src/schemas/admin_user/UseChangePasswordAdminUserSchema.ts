import Joi from 'joi';
import {useTranslation} from "react-i18next";

export const UseChangePasswordAdminUserSchema = (() => {

    const [t] = useTranslation("global")
    return Joi.object({
        password : Joi.string().required().messages({
            'string.empty': t("empty_message", {attribute: t("password")}),
        }),
        new_password: Joi.string().required().messages({
            'string.empty': t("empty_message", {attribute: t("new_password")}),
        }),
        confirm_password: Joi.any().valid(Joi.ref('new_password')).required().messages({
            'string.empty': t("empty_message", {attribute: t("current_password")}),
            'any.only': t("confirm_password_does_not_match_with_current_password"),
        }),
    });
})

import Joi from 'joi'
import {useTranslation} from "react-i18next"

export const UseRoleCreateSchema = (() => {

    const [t] = useTranslation("global")
    return Joi.object({

    });
})

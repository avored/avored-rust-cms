import Joi from 'joi';


export const useContactUsFormSchema = (() => {

    return Joi.object({
        first_name : Joi.string().required().messages({
            'string.empty': "First name is required",
        }),
        last_name : Joi.string().required().messages({
            'string.empty': "Last name is required",
        }),
        phone : Joi.string().required().messages({
            'string.empty': "Phone is required",
        }),
        email : Joi.string().email({tlds: { allow: false }}).messages({
            'string.empty': "Email is required",
            'string.email': 'Email is not in valid format.'
        }),
        message : Joi.string().required().messages({
            'string.empty': "Message is required",
        })
    });
})
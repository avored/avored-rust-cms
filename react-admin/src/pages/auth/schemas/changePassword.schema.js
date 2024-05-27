import Joi from 'joi';

export const changePasswordSchema = Joi.object({
    password: Joi.string().required().messages({
        'string.empty': 'Password is required field'
    }),
    confirm_password: Joi.string().required().messages({
        'string.empty': 'Confirm Password is required field'
    }),
    token: Joi.string().required().messages({
        'string.empty': 'Token is required field'
    }),
});

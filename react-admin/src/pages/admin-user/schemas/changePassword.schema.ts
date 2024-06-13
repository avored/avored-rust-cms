import Joi from 'joi';

export const changePasswordSchema = Joi.object({
    current_password: Joi.string().required().messages({
        'string.empty': 'Password is required field'
    }),
    password: Joi.string().messages({
        'string.empty': 'Password is required field'
    }),
    confirm_password: Joi.string().equal(Joi.ref('password')).messages({
        'any.only': "Confirm password does not match with new password",
        'string.empty': 'Conform Password is required field',
    })

});

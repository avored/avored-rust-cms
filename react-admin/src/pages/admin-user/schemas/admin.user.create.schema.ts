import Joi from 'joi';

export const AdminUserCreateSchema = Joi.object({
    email : Joi.string().email({ tlds: { allow: false } }).required().messages({
        'string.empty': 'Email is not allowed to be empty',
        'string.email': 'Invalid Email Address',
    })
});

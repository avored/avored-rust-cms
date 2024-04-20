import Joi from 'joi';

export const loginSchema = Joi.object({
  email : Joi.string().required().messages({
    'string.empty': 'Email address is required field'
  }),
  password : Joi.string().required().messages({
    'string.empty': 'Password is required field'
  }),
});

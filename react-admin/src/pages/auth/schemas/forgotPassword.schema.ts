import Joi from 'joi';

export const forgotPasswordSchema = Joi.object({
  email : Joi.string().required().messages({
    'string.empty': 'Email address is required field'
  }),
});

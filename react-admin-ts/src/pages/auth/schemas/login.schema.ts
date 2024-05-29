import Joi, { ObjectSchema } from "joi";

interface LoginSchema {
  email: string;
  password: string;
}

export const loginSchema: ObjectSchema<LoginSchema> = Joi.object({
  email: Joi.string().required().messages({
    "string.empty": "Email address is required field",
  }),
  password: Joi.string().required().messages({
    "string.empty": "Password is required field",
  }),
});

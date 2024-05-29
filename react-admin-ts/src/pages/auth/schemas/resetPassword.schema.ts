import Joi, { ObjectSchema } from "joi";

interface ResetPasswordSchema {
  email: string;
  password: string;
  confirm_password: string;
  token: string;
}

export const resetPasswordSchema: ObjectSchema<ResetPasswordSchema> =
  Joi.object({
    email: Joi.string().required().messages({
      "string.empty": "Email address is required field",
    }),
    password: Joi.string().required().messages({
      "string.empty": "Password is required field",
    }),
    confirm_password: Joi.string().required().messages({
      "string.empty": "Confirm Password is required field",
    }),
    token: Joi.string().required().messages({
      "string.empty": "Token is required field",
    }),
  });

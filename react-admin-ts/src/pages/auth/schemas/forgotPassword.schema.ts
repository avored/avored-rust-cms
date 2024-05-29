import Joi, { ObjectSchema } from "joi";

interface ForgotPasswordSchema {
  email: string;
}

export const forgotPasswordSchema: ObjectSchema<ForgotPasswordSchema> =
  Joi.object({
    email: Joi.string().required().messages({
      "string.empty": "Email address is required field",
    }),
  });

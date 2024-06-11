import Joi from 'joi';

export const PageEditSchema = Joi.object({
    name : Joi.string().required().messages({
        'string.empty': 'Name is required.',
    }),
    identifier : Joi.string().required().messages({
        'string.empty': 'Identifier is required.',
    })
});

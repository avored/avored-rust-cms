import Joi from 'joi'

export const ComponentEditSchema = Joi.object({
    name : Joi.string().required().messages({
        'string.empty': 'Name is required field',
    })
})

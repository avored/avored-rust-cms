import _ from "lodash";
import {PageFieldType} from "../types/CmsPageType";

export const GetElementValue = ((fields: Array<PageFieldType>, element_identifier: string) => {
    console.log(fields, "fields");
    const field = fields.find((ele: any) => {
        return ele.identifier === element_identifier
    })
    return _.get(field, 'field_content.text_value.text_value', '')
})
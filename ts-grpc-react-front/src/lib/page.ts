import _ from "lodash";
import {ContentFieldType} from "../types/CmsPageType";

export const GetElementValue = ((fields: Array<ContentFieldType>, element_identifier: string) => {
    // console.log(fields, "fields");
    const field = fields.find((ele: any) => {
        return ele.identifier === element_identifier
    })
    // console.log(field, "field")
    return _.get(field, 'fieldContent.textValue', '')
})

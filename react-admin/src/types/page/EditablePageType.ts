import {AvoRedPageFieldData} from "./CreatablePageType";
import {AvoRedPageDataType, AvoRedPageFieldType} from "./IPageModel";


export type EditablePageType = {
    id: string;
    name: string;
    identifier: string;
    page_fields?: Array<EditableFieldType>
}

export type EditableFieldType = {
    name: string;
    identifier: string;
    data_type: AvoRedPageDataType;
    field_type: AvoRedPageFieldType;
    field_content: string | number;
    field_data?: AvoRedPageFieldData
}


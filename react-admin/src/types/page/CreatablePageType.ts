import {AvoRedPageDataType, AvoRedPageFieldType} from "./IPageModel";

export type CreatablePageType = {
    name: string;
    identifier: string;
    page_fields?: Array<CreatableFieldType>
}

export type CreatableFieldType = {
    name: string;
    identifier: string;
    data_type: AvoRedPageDataType;
    field_type: AvoRedPageFieldType;
    field_content: string | number;
    field_data?: AvoRedPageFieldData,
}

export type AvoRedPageFieldData = {
    select_field_options: Array<AvoRedPageFieldSelectFieldDataOptions>
}
export type EmptyPageFieldData = {}

export type AvoRedPageFieldSelectFieldDataOptions = {
    label: string;
    value: string;
}
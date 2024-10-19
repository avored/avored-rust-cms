import {AvoRedPageDataType, AvoRedPageFieldType} from "./IPageModel";

export type SavePageType = {
    name: string;
    identifier: string;
    page_fields?: Array<SaveFieldType>
}

export type SaveFieldType = {
    name: string;
    identifier: string;
    data_type: AvoRedPageDataType;
    field_type: AvoRedPageFieldType;
    field_data?: AvoRedPageFieldData,
}

export type AvoRedPageFieldData = {
    select_field_options?: Array<AvoRedPageFieldSelectFieldDataOptions>,
    radio_field_options?: Array<AvoRedPageFieldRadioFieldDataOptions>
}
export type EmptyPageFieldData = {}

export type AvoRedPageFieldSelectFieldDataOptions = {
    label: string;
    value: string;
}


export type AvoRedPageFieldRadioFieldDataOptions = {
    label: string;
    value: string;
}

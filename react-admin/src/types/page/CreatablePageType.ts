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
    field_content: PageFieldContent;
    field_data?: AvoRedPageFieldData,
}

export type PageFieldContent = {
    text_value?: PageTextContent,
    integer_value?: PageIntegerContent
}

export type PageTextContent = {
    text_value: string;
}
export type PageIntegerContent = {
    integer_value: number;
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

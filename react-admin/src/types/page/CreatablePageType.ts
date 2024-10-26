import {AvoRedPageDataType, AvoRedPageFieldType, AvoRedPageStatus} from "./IPageModel";

export type SavePageType = {
    name: string;
    identifier: string;
    status: AvoRedPageStatus;
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
    integer_value?: PageIntegerContent,
    array_value?: PageArrayContent
}

export type PageTextContent = {
    text_value: string;
}
export type PageIntegerContent = {
    integer_value: number;
}

export type PageArrayContent = {
    array_value: Array<string>;
}


export type AvoRedPageFieldData = {
    select_field_options?: Array<AvoRedPageFieldSelectFieldDataOptions>,
    radio_field_options?: Array<AvoRedPageFieldRadioFieldDataOptions>
    checkbox_field_options?: Array<AvoRedPageFieldCheckboxFieldDataOptions>
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


export type AvoRedPageFieldCheckboxFieldDataOptions = {
    label: string;
    value: string;
}

import {GrpcTimeStamp} from "../common/common";

export type CollectionType = {
    id: string;
    name: string;
    identifier: string;
    createdAt: string;
    updatedAt: string;
    createdBy: string;
    updatedBy: string;
    action?: string;
}
export enum ContentFieldDataType {
    Bool = "Bool",
    TEXT = "TEXT",
    INT = "INT",
    FLOAT = "FLOAT",
    Array = "Array"
}

export enum ContentFieldFieldType {
    TEXT = "TEXT",
    TEXTAREA = "TEXTAREA",
    RICH_TEXT_EDITOR = "RICH_TEXT_EDITOR",
    NUMBER_TEXT_FIELD = "NUMBER_TEXT_FIELD",
    FLOAT_TEXT_FIELD = "FLOAT_TEXT_FIELD",
    SELECT = "Select",
    Checkbox = "Checkbox",
    Radio = "Radio",
    Switch = "Switch"
}

export type ContentSelectFieldData = {
    label: string;
    value: string;
}

export type ContentCheckboxFieldData = {
    label: string;
    value: string;
}

export type ContentRadioFieldData = {
    label: string;
    value: string;
}

export type ContentFieldData = {
    content_select_field_options: Array<ContentSelectFieldData>;
    content_checkbox_field_data: Array<ContentCheckboxFieldData>;
    content_radio_field_data: Array<ContentRadioFieldData>;
}

export type ContentFieldType = {
    name: string;
    identifier: string;
    data_type: ContentFieldDataType;
    field_type: ContentFieldFieldType;
    field_content: ContentFieldFieldContent;
    field_data?: ContentFieldData
}

export type ContentFieldFieldContent = {
    text_value?: string,
    int_value?: number,
    float_value?: number,
    array_value?: Array<string>,
    bool_value?: boolean,
}

export type ContentType = {
    id: string;
    name: string;
    identifier: string;
    createdAt: GrpcTimeStamp;
    createdBy: string;
    updatedAt: GrpcTimeStamp;
    updatedBy: string;
    content_fields: Array<ContentFieldType>;
    action: string;
}

export type SaveContentType = {
    id?: String;
    name: string;
    content_type: string;
    identifier: string;
    content_fields: Array<SaveContentFieldType>;
}

export type SaveContentFieldType = {
    name: string;
    identifier: string;
    data_type: ContentFieldDataType;
    field_type: ContentFieldFieldType;
    field_content: ContentFieldFieldContent;
    field_data?: ContentFieldData;
}

export type StoreCollectionType = {
    name: string;
    identifier: string;
}
export type UpdateCollectionType = {
    id: string;
    name: string;
    identifier: string;
}

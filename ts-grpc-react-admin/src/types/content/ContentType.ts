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
    TEXT = "TEXT",
    // INT = "INT",
    // Array_Text = "Array_Text"
}

export enum ContentFieldFieldType {
    TEXT = "TEXT",
    TEXTAREA = "TEXTAREA",
    RICH_TEXT_EDITOR = "RICH_TEXT_EDITOR",
    // INT = "INT",
    // Array_Text = "Array_Text"
}

export type ContentFieldType = {
    name: string;
    identifier: string;
    data_type: ContentFieldDataType;
    field_type: ContentFieldFieldType;
    field_content: ContentFieldFieldContent;
}

export type ContentFieldFieldContent = {
    text_value?: string,
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
}

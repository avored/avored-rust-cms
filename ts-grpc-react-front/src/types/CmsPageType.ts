export type GrpcTimeStamp = {
    nanos: number;
    seconds: number;
}

export type CmsContentType = {
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

export enum ContentFieldDataType {
    TEXT = "TEXT",
    // INT = "INT",
    // Array_Text = "Array_Text"
}

export enum ContentFieldFieldType {
    TEXT = "TEXT",
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


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
    contentFields: Array<ContentFieldType>;
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
    dataType: ContentFieldDataType;
    fieldType: ContentFieldFieldType;
    fieldContent: ContentFieldFieldContent;
}

export type ContentFieldFieldContent = {
    textValue?: string,
}


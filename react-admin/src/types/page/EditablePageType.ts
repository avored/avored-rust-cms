export type EditablePageType = {
    id: string;
    name: string;
    identifier: string;
    page_fields?: Array<CreatableFieldType>
}

export type CreatableFieldType = {
    name: string;
    identifier: string;
    data_type: AvoRedDataType;
    field_type: AvoRedFieldType;
    field_content: string | number;
}

export enum AvoRedFieldType {
    TEXT = "TEXT",
    TEXTAREA = "TEXTAREA"
}


export enum AvoRedDataType {
    TEXT = "TEXT",
    INT = "INT"
}
export type ContentModel = {
    id: string;
    name: string;
    identifier: string;
    created_at: string;
    created_by: string;
    updated_at: string;
    updated_by: string;
    action: string;
}


export type SaveContentType = {
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
    field_content: AvoRedContentFieldContent;
    // field_data?: AvoRedContentFieldData,
}


export enum ContentFieldDataType {
    TEXT = "Text",
    // INT = "INT",
    // Array_Text = "Array_Text"
}

export type AvoRedContentFieldContent = {
    text_value?: ContentTextContent,
}

export type ContentTextContent = {
    text_value: string;
}

export enum ContentFieldFieldType {
    TEXT = "Text",
    // TEXTAREA = "Textarea",
    // SELECT = "Select",
    // TextEditor = "TextEditor",
    // Radio = "Radio",
    // Checkbox = "Checkbox",
    // SingleImage = "SingleImage"
 }

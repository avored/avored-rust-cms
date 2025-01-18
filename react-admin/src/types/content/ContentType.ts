export type SaveContentType = {
    name: string;
    type: string;
    identifier: string;
    content_fields: Array<SaveContentFieldType>;
}

export type SaveContentFieldType = {
    name: string;
    identifier: string;
    data_type: AvoRedContentDataType;
    field_type: AvoRedContentFieldType;
    field_content: AvoRedContentFieldContent;
    // field_data?: AvoRedContentFieldData,
}


export enum AvoRedContentDataType {
    TEXT = "TEXT",
    // INT = "INT",
    // Array_Text = "Array_Text"
}

export type AvoRedContentFieldContent = {
    text_value?: ContentTextContent,
}

export type ContentTextContent = {
    text_value: string;
}

export enum AvoRedContentFieldType {
    TEXT = "Text",
    // TEXTAREA = "Textarea",
    // SELECT = "Select",
    // TextEditor = "TextEditor",
    // Radio = "Radio",
    // Checkbox = "Checkbox",
    // SingleImage = "SingleImage"
 }

import {PageDataType, PageFieldType} from "./IPageModel";


export type EditablePageType = {
    id: string;
    name: string;
    identifier: string;
    page_fields?: Array<EditableFieldType>
}

export type EditableFieldType = {
    name: string;
    identifier: string;
    data_type: PageDataType;
    field_type: PageFieldType;
    field_content: string | number;
}


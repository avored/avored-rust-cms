import {PageDataType, PageFieldType} from "./IPageModel";

export type CreatablePageType = {
    name: string;
    identifier: string;
    page_fields?: Array<CreatableFieldType>
}

export type CreatableFieldType = {
    name: string;
    identifier: string;
    data_type: PageDataType;
    field_type: PageFieldType;
    field_content: string | number;
}


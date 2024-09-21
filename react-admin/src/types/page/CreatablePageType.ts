import {AvoRedPageDataTYpe, AvoRedPageFieldType} from "./IPageModel";

export type CreatablePageType = {
    name: string;
    identifier: string;
    page_fields?: Array<CreatableFieldType>
}

export type CreatableFieldType = {
    name: string;
    identifier: string;
    data_type: AvoRedPageDataTYpe;
    field_type: AvoRedPageFieldType;
    field_content: string | number;
}


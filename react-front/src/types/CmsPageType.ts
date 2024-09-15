export type CmsPageType = {
    status: boolean;
    data: {
        page_model: CmsPageModelType
    }
}

export type CmsPageModelType = {
    id: string,
    name: string,
    identifier: string,
    page_fields: Array<PageFieldType>,
    created_by: string,
    updated_by: string,
    created_at: Date,
    updated_at: Date
}

export enum AvoRedElementType {
    TEXT = "text"
}
export type PageFieldType = {
    field_content: string;
    data_type: String;
    field_type: AvoRedElementType;
    identifier: string;
    name: string;
}

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
    components_content: ComponentContentType[],
    created_by: string,
    updated_by: string,
    created_at: Date,
    updated_at: Date
}

export enum AvoRedElementType {
    TEXT = "text"
}
export type ComponentContentType = {
    element_content: string;
    element_data: ComponentContentDataType[];
    element_type: AvoRedElementType;
    identifier: string;
    name: string;
}

export type ComponentContentDataType = {
    label: string,
    value: string
}
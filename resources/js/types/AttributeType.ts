export interface AttributeInterface {
    id: string;
    entity_id: string;
    name: string;
    identifier: string;
    field_type: string;
    data_type: string;
    created_at: string;
    created_by: string;
    updated_at: string;
    updated_by: string;
    deleted_at?: string | null;
    deleted_by?: string | null;
}



export interface AttributePaginationResponse {
    data: AttributeInterface[];
    total: number;
}

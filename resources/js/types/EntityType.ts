export interface AttributeInterface {
    id: string;
    name: string;
    identifier: string;
    data_type: string;
    field_type: string;
    created_at: Date;
    created_by: string;
    updated_at: Date;
    updated_by: string;
    deleted_at?: Date;
    deleted_by?: string;
}

export interface EntityInterface {
    id: string;
    name: string;
    identifier: string;
    created_at: Date;
    created_by: string;
    updated_at: Date;
    updated_by: string;
    deleted_at?: Date;
    deleted_by?: string;
    attributes: Array<AttributeInterface>
}

export interface CreateEntityPayload {
    name: string;
    identifier: string;
}

export interface UpdateEntityPayload {
    name: string;
    identifier: string;
}

export interface EntityPaginationResponse {
    data: EntityInterface[];
    total: number;
}

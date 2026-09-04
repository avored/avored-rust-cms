export interface EntityInterface {
    id: string;
    name: string;
    identifier: string;
    data_type: string;
    created_at: string;
    updated_at: string;
    deleted_at?: string | null;
}

export interface CreateEntityPayload {
    name: string;
    identifier: string;
    data_type: string;
}

export interface UpdateEntityPayload {
    name: string;
    identifier: string;
    data_type: string;
}

export interface EntityPaginationResponse {
    data: EntityInterface[];
    total: number;
}

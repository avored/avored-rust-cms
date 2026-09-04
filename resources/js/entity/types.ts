export interface EntityInterface {
    id: string;
    name: string;
    identifier: string;
    created_at: string;
    updated_at: string;
    deleted_at?: string | null;
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

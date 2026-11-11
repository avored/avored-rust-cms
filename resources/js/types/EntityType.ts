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

export interface EntityOptionInterface {
    id: string;
    name: string;
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

export interface PaginationInterface {
    total: number;
    per_page: number;
    current_page: number;
    last_page: number;
}

export interface PaginatedResponse<T> {
    data: T[];
    pagination: PaginationInterface;
}

export type EntityPaginationResponse = PaginatedResponse<EntityInterface>;

export interface CollectionPaginationResponse {
    entity: EntityInterface;
    data: Array<Record<string, any>>;
    pagination: PaginationInterface;
}

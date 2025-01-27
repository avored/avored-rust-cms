
export type SavableCollectionType = {
    name: string;
    identifier: string;
    collection_fields: Array<SaveCollectionFieldType>
}

export type SaveCollectionFieldType = {
    name: string;
    identifier: string;
    data_type: CollectionFieldDataType;
    field_type: CollectionFieldFieldType;
}

export enum CollectionFieldDataType {
    TEXT = "Text",
}

export enum CollectionFieldFieldType {
    TEXT = "Text",
}
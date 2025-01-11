export type CreatableModelType = {
    name: string;
    identifier: string;
    model_fields: Array<SaveModelFieldType>
}

export type SaveModelFieldType = {
    name: string;
}
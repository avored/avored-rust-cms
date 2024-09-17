export default interface IPageModel {
    id: string;
    name: string;
    identifier: string;
    created_at: string;
    created_by: string;
    updated_at: string;
    updated_by: string;
    action: string;
}


export enum PageDataType {
    TEXT = "TEXT",
    INT = "INT"
}


export enum PageFieldType {
    TEXT = "TEXT",
    TEXTAREA = "TEXTAREA"
}

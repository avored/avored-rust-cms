export default interface IEditableField {
    id: string;
    name: string;
    identifier: string;
    field_type: string;
    field_data?: Array<IOptionField>
}

export interface IOptionField {
    label: string;
    value: string
}

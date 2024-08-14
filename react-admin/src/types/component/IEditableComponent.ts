export default  interface IEditableComponent {
    id: string;
    name: string;
    elements: Array<EditableComponentElementType>;
    identifier: string;
}

export type EditableComponentElementType = {
    name: string;
    identifier: string;
    element_type: string;
    element_data_type: string;
    element_data?: Array<EditableComponentElementDataType>
}

export type EditableComponentElementDataType = {
    label: string;
    value: string;
}
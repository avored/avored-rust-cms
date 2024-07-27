export default  interface ICreatableComponent {
    name: string;
    identifier: string;
    elements: Array<CreatableElementType>;
}

export type CreatableElementType = {
    name: string;
    identifier: string;
    element_type: string;
    element_data?: Array<CreatableElementDataType>
}

export type CreatableElementDataType = {
    label: string;
    value: string;
}
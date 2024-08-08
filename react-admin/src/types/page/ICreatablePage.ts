export default interface ICreatablePage {
    name: string;
    identifier: string;
    components_content: Array<ICreatablePageComponentModel>
}

export interface ICreatablePageComponentModel {
    id: string;
    name: string;
    identifier: string;
    created_at: string;
    updated_at: string;
    created_by: string;
    updated_by: string;
    elements: Array<ICreatablePageComponentElementModel>
}

export interface ICreatablePageComponentElementModel {
    id: string;
    name: string;
    identifier: string;
    element_type: string;
    element_data_type: string;
    element_content: string;
    element_data?: Array<ICreatablePageComponentElementDataModel>;
    created_at: string;
    updated_at: string;
    created_by: string;
    updated_by: string;
}

export interface ICreatablePageComponentElementDataModel {
    label: string;
    value: string;
    element_data_content: string;
}
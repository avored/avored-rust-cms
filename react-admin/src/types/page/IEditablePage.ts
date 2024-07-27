import IComponentModel from "../component/IComponentModel";
import IFieldModel from "../component/IFieldModel";

export default interface IEditablePage {
    id: string;
    name: string;
    components_content: Array<IEditablePageComponentModel>
}

export interface IEditablePageComponentModel {
    id: string;
    name: string;
    identifier: string;
    created_at: string;
    updated_at: string;
    created_by: string;
    updated_by: string;
    elements: Array<IEditablePageComponentFieldModel>
}

export interface IEditablePageComponentFieldModel {
    name: string;
    identifier: string;
    element_type: string;
    element_content: string;
    element_data?: Array<IEditablePageComponentFieldDataModel>;
    created_at: string;
    updated_at: string;
    created_by: string;
    updated_by: string;
}


export interface IEditablePageComponentFieldDataModel {
    label: string;
    value: string;
    element_data_content: string;
}
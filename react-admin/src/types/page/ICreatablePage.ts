import IComponentModel from "../component/IComponentModel";
import IFieldModel from "../component/IFieldModel";

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
    fields: Array<ICreatablePageComponentFieldModel>
}

export interface ICreatablePageComponentFieldModel {
    id: string;
    name: string;
    identifier: string;
    field_type: string;
    field_content: string;
    field_data?: Array<any>;
    created_at: string;
    updated_at: string;
    created_by: string;
    updated_by: string;
}
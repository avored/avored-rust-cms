import CreatableFieldDataType from "./CreatableFieldDataType";


export default interface ICreatableField {
    name: string;
    identifier: string;
    field_type: string;
    field_data: Array<CreatableFieldDataType>
}


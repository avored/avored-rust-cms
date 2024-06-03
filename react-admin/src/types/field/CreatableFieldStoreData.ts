import CreatableFieldType from "./CreatableFieldType";

export default  interface CreatableFieldStoreData {
    name: string,
    fields: Array<CreatableFieldType>,
    identifier: string
}
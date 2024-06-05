import IEditableField from "./IEditableField";

export default  interface IEditableComponent {
    id: string;
    name: string;
    fields: Array<IEditableField>;
    identifier: string;
}
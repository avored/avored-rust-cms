import ICreatableField from "./ICreatableField";

export default  interface ICreatableComponent {
    name: string;
    fields: Array<ICreatableField>;
    identifier: string;
}
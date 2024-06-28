import {useTranslation} from "react-i18next";
import {getFormattedDate} from "../../lib/common";
import IComponentModel from "../../types/component/IComponentModel";

interface ISelectedComponentFunction {
    (e: React.MouseEvent, id: string): void
}

interface IPageComponentProps {
    components: Array<IComponentModel>;
    componentSelected: ISelectedComponentFunction
}

function PageComponentTable(props: IPageComponentProps) {
    const [t] = useTranslation("global");
    return (
        <table className="min-w-full bg-white shadow-md rounded">
            <thead>
            <tr className="bg-gray-700 text-white">
                <th className="py-3 px-4 rounded-l font-semibold text-left">
                    {t("id")}
                </th>
                <th className="py-3 px-4 font-semibol text-left">
                    {t("name")}
                </th>
                <th className="py-3 px-4 font-semibol text-left">
                    {t("identifier")}
                </th>
                <th className="py-3 px-4 font-semibol text-left">
                    {t("created_at")}
                </th>
                <th className="py-3 px-4 font-semibol text-left">
                    {t("updated_at")}
                </th>
                <th className="py-3 px-4 font-semibol text-left">
                    {t("created_by")}
                </th>
                <th className="py-3 px-4 font-semibol text-left">
                    {t("updated_by")}
                </th>
                <th className="py-3 px-4 rounded-r font-semibol text-left">
                    {t("action")}
                </th>
            </tr>
            </thead>
            <tbody className="">
            {props.components.map((component) => {
                return (
                    <tr key={component.id} className="border-b">
                        <td className="py-3 px-4">{component.id}</td>
                        <td className="py-3 px-4">{component.name}</td>
                        <td className="py-3 px-4">
                            {component.identifier}
                        </td>
                        <td className="py-3 px-4">
                            {getFormattedDate(component.created_at)}
                        </td>
                        <td className="py-3 px-4">
                            {getFormattedDate(component.updated_at)}
                        </td>
                        <td className="py-3 px-4">
                            {component.created_by}
                        </td>
                        <td className="py-3 px-4">
                            {component.updated_by}
                        </td>
                        <td className="py-3 px-4">
                            <button
                                type="button"
                                className="font-medium text-primary-600 hover:text-primary-800"
                                onClick={(e) =>
                                    props.componentSelected(e, component.id)
                                }
                            >
                                {t("select")}
                            </button>
                        </td>
                    </tr>
                );
            })}
            </tbody>
        </table>
    )
}

export default PageComponentTable
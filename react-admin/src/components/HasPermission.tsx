import { ReactNode } from "react";
import { hasPermission } from "../lib/common";

type HasPermissionProps = {
    identifier: string;
    children: ReactNode;
    displayDenied?: boolean
}
const HasPermission = (props: HasPermissionProps) => {
    return (
        <>
            {hasPermission(props.identifier) ?  props.children : (props.displayDenied) ? <div>permission denied</div> : ""}
        </>
    )
}

export default HasPermission

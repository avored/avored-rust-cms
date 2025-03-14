import {ReactNode} from "react";


type HasPermissionProps = {
    identifier: string;
    children: ReactNode;
    displayDenied?: boolean
}
const HasPermission = (props: HasPermissionProps) => {
    // @todo fix it
    const hasPermission = (test: string) => {
        return true
    }

    return (
        <>
            {hasPermission(props.identifier) ? props.children : (props.displayDenied) ?
                <div>permission denied</div> : ""}
        </>
    )
}

export default HasPermission

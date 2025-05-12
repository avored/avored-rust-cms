import {ReactNode} from "react";
import {UseLoggedInUserHook} from "../hooks/general/UseLoggedInUserHook";
import {AdminUserType, RoleType} from "../types/admin_user/AdminUserType";
import {LoggedInUserRequest} from "../grpc_generated/general_pb";


type HasPermissionProps = {
    identifier: string;
    children: ReactNode;
    displayDenied?: boolean
}
const HasPermission = (props: HasPermissionProps) => {
    // @todo fix it
    const hasPermission = (identifier: string) => {
        const request = new LoggedInUserRequest();
        const auth_user_model = UseLoggedInUserHook(request);

        const logged_in_user: AdminUserType = auth_user_model?.data?.data as unknown as AdminUserType;

        // console.log(logged_in_user)

        if (logged_in_user?.isSuperAdmin) {
            return true
        }
        let has_permission: boolean = false
        logged_in_user?.rolesList.forEach((role: RoleType) => {
            return role.permissionsList.forEach((permission: string) => {
                if(permission === identifier) {
                    has_permission = true
                }
            })
        })
        // console.log(has_permission, identifier)
        return has_permission
    }

    return (
        <>
            {hasPermission(props.identifier) ? props.children : (props.displayDenied) ?
                <div>permission denied</div> : ""}
        </>
    )
}

export default HasPermission

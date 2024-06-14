import {DateTime} from "luxon";
import { useLoggedInUser } from "../hooks/useLoggedInUser";
import IRoleModel from "../types/admin-user/IRoleModel";

const randomString = ((length: number = 16) => {
    let result = '';
    const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    const charactersLength = characters.length;
    let counter = 0;
    while (counter < length) {
        result += characters.charAt(Math.floor(Math.random() * charactersLength));
        counter += 1;
    }
    return result;
})

const getFormattedDate = ((date: string) => {
    let dateObject  = DateTime.fromJSDate(new Date(date))

    return dateObject.toLocaleString(DateTime.DATE_MED)
})

const hasPermission = ((identifier: string ): boolean => {
    const logged_in_user = useLoggedInUser()
    if (logged_in_user.is_super_admin) {
        return true
    }
    let has_permission: boolean = false
    logged_in_user.roles.map((role: IRoleModel) => {
        role.permissions.map((permission: string) => {
            if(permission === identifier) {
                has_permission = true
            }
        })
    })
    return has_permission
})

export {
    randomString,
    getFormattedDate,
    hasPermission
}

import IRoleModel from "./IRoleModel";

export default interface IAdminUserModel {
    id: string;
    full_name: string;
    email: string;
    password: string;
    is_super_admin: boolean;
    profile_image: string;
    roles: Array<IRoleModel>;
    created_at: string;
    created_by: string;
    updated_at: string;
    updated_by: string;
    action: string;
}
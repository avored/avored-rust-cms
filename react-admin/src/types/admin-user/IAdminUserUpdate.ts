import IRoleModel from "./IRoleModel";

export default interface IAdminUserUpdate {
    id: string;
    full_name: string;
    email: string;
    password: string;
    image?: File;
    is_super_admin: boolean;
    profile_image: string;
    roles: Array<IRoleModel>;
    created_at: string;
    created_by: string;
    updated_at: string;
    updated_by: string;
    role_ids: Array<string>;
    action: string;
}
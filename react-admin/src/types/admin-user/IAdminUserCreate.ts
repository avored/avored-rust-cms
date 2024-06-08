import IRoleModel from "./IRoleModel";

export default interface IAdminUserCreate {
    full_name: string;
    email: string;
    image?: File;
    password: string;
    confirmation_password: string;
    profile_image?: Array<File>;
    is_super_admin: boolean;
    role_ids: Array<string>;
}
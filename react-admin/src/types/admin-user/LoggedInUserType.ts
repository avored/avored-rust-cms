import IRoleModel from "./IRoleModel";

type LoggedInUserType = {
    id: string;
    full_name: string;
    email: string;
    is_super_admin: boolean;
    created_at: string;
    updated_at: string;
    created_by: string;
    updated_by: string;
    password: string;
    profile_image: string;
    roles: Array<IRoleModel>
}

export default LoggedInUserType;

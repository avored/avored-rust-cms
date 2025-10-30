import type { GrpcTimeStamp } from "../common/common";

export type AdminUserType = {
    id: string;
    email: string;
    fullName: string;
    isSuperAdmin: boolean;
    profileImage: string;
    rolesList: Array<RoleType>;
    createdAt: GrpcTimeStamp;
    updatedAt: GrpcTimeStamp;
    createdBy: string;
    updatedBy: string;
    action: string;
}

export type CreateAdminUserType = {
    full_name: string;
    email: string;
    password: string;
    confirmation_password: string;
    profile_image: FileList;
    locale: string;
}

export type EditAdminUserType = {
    id: string;
    fullName: string;
    profileImage: string;
    email: string;
    profile_image: FileList;
    roles: Array<RoleType>;
    isSuperAdmin: boolean;
    locale: string;
}

export type ChangePasswordType = {
    password: string;
    new_password: string;
    confirm_password: string;
}

export type RoleOptionType = {
    label: string;
    value: string;
}


export type RoleType = {
    id: string;
    name: string;
    identifier: string;
    createdAt: GrpcTimeStamp;
    updatedAt: GrpcTimeStamp;
    permissions: Array<string>;
    permissionsList: Array<string>;
    createdBy: string;
    updatedBy: string;
    action: string;
}

export type CreatableRoleType = {
    name: string;
    identifier: string;
    permissions: Array<string>;
}


export type EditRoleType = {
    id: string;
    name: string;
    identifier: string;
    permissions: Array<string>;
}

export type PutRoleIdentifierType = {
    identifier: String;
}

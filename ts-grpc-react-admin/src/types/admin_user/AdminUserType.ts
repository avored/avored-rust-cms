import {GrpcTimeStamp} from "../common/common";

export type AdminUserType = {
    id: string;
    email: string;
    fullName: string;
    isSuperAdmin: boolean;
    profileImage: string;
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
    profile_image: string;
}

export type EditAdminUserType = {
    full_name: string;
    profile_image: string;
}

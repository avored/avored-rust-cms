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
    profile_image: FileList;
}

export type EditAdminUserType = {
    id: string;
    fullName: string;
    profileImage: string;
    email: string;
    profile_image: FileList;
}

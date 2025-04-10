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



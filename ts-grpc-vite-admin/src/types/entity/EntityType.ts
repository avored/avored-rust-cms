import { GrpcTimeStamp } from "../common/common";

export type EntityType = {
    id: string;
    name: string;
    identifier: string;
    createdAt: GrpcTimeStamp;
    updatedAt: GrpcTimeStamp;
    createdBy: string;
    updatedBy: string;
    action: string;
}

export type CreateEntityType = {
    name: string;
    identifier: string;
}
import {GrpcTimeStamp} from "../common/common";

export type CollectionType = {
    id: string;
    name: string;
    identifier: string;
    createdAt: string;
    updatedAt: string;
    createdBy: string;
    updatedBy: string;
    action?: string;
}
export type ContentType = {
    id: string;
    name: string;
    identifier: string;
    createdAt: GrpcTimeStamp;
    createdBy: string;
    updatedAt: GrpcTimeStamp;
    updatedBy: string;
    action: string;
}

export type SaveContentType = {
    id: String;
    name: string;
    content_type: string;
    identifier: string;
    // content_fields: Array<SaveContentFieldType>;
}

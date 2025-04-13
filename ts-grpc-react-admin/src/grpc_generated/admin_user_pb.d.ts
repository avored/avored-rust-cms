import * as jspb from 'google-protobuf'

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb'; // proto import: "google/protobuf/timestamp.proto"


export class AdminUserModel extends jspb.Message {
  getId(): string;
  setId(value: string): AdminUserModel;

  getFullName(): string;
  setFullName(value: string): AdminUserModel;

  getEmail(): string;
  setEmail(value: string): AdminUserModel;

  getProfileImage(): string;
  setProfileImage(value: string): AdminUserModel;

  getIsSuperAdmin(): boolean;
  setIsSuperAdmin(value: boolean): AdminUserModel;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): AdminUserModel;
  hasCreatedAt(): boolean;
  clearCreatedAt(): AdminUserModel;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): AdminUserModel;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): AdminUserModel;

  getCreatedBy(): string;
  setCreatedBy(value: string): AdminUserModel;

  getUpdatedBy(): string;
  setUpdatedBy(value: string): AdminUserModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AdminUserModel.AsObject;
  static toObject(includeInstance: boolean, msg: AdminUserModel): AdminUserModel.AsObject;
  static serializeBinaryToWriter(message: AdminUserModel, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AdminUserModel;
  static deserializeBinaryFromReader(message: AdminUserModel, reader: jspb.BinaryReader): AdminUserModel;
}

export namespace AdminUserModel {
  export type AsObject = {
    id: string,
    fullName: string,
    email: string,
    profileImage: string,
    isSuperAdmin: boolean,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    createdBy: string,
    updatedBy: string,
  }
}

export class AdminUserPaginateRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AdminUserPaginateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: AdminUserPaginateRequest): AdminUserPaginateRequest.AsObject;
  static serializeBinaryToWriter(message: AdminUserPaginateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AdminUserPaginateRequest;
  static deserializeBinaryFromReader(message: AdminUserPaginateRequest, reader: jspb.BinaryReader): AdminUserPaginateRequest;
}

export namespace AdminUserPaginateRequest {
  export type AsObject = {
  }
}

export class AdminUserPaginateResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): AdminUserPaginateResponse;

  getData(): AdminUserPaginateResponse.AdminUserPaginateData | undefined;
  setData(value?: AdminUserPaginateResponse.AdminUserPaginateData): AdminUserPaginateResponse;
  hasData(): boolean;
  clearData(): AdminUserPaginateResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AdminUserPaginateResponse.AsObject;
  static toObject(includeInstance: boolean, msg: AdminUserPaginateResponse): AdminUserPaginateResponse.AsObject;
  static serializeBinaryToWriter(message: AdminUserPaginateResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AdminUserPaginateResponse;
  static deserializeBinaryFromReader(message: AdminUserPaginateResponse, reader: jspb.BinaryReader): AdminUserPaginateResponse;
}

export namespace AdminUserPaginateResponse {
  export type AsObject = {
    status: boolean,
    data?: AdminUserPaginateResponse.AdminUserPaginateData.AsObject,
  }

  export class AdminUserPagination extends jspb.Message {
    getTotal(): number;
    setTotal(value: number): AdminUserPagination;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): AdminUserPagination.AsObject;
    static toObject(includeInstance: boolean, msg: AdminUserPagination): AdminUserPagination.AsObject;
    static serializeBinaryToWriter(message: AdminUserPagination, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): AdminUserPagination;
    static deserializeBinaryFromReader(message: AdminUserPagination, reader: jspb.BinaryReader): AdminUserPagination;
  }

  export namespace AdminUserPagination {
    export type AsObject = {
      total: number,
    }
  }


  export class AdminUserPaginateData extends jspb.Message {
    getPagination(): AdminUserPaginateResponse.AdminUserPagination | undefined;
    setPagination(value?: AdminUserPaginateResponse.AdminUserPagination): AdminUserPaginateData;
    hasPagination(): boolean;
    clearPagination(): AdminUserPaginateData;

    getDataList(): Array<AdminUserModel>;
    setDataList(value: Array<AdminUserModel>): AdminUserPaginateData;
    clearDataList(): AdminUserPaginateData;
    addData(value?: AdminUserModel, index?: number): AdminUserModel;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): AdminUserPaginateData.AsObject;
    static toObject(includeInstance: boolean, msg: AdminUserPaginateData): AdminUserPaginateData.AsObject;
    static serializeBinaryToWriter(message: AdminUserPaginateData, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): AdminUserPaginateData;
    static deserializeBinaryFromReader(message: AdminUserPaginateData, reader: jspb.BinaryReader): AdminUserPaginateData;
  }

  export namespace AdminUserPaginateData {
    export type AsObject = {
      pagination?: AdminUserPaginateResponse.AdminUserPagination.AsObject,
      dataList: Array<AdminUserModel.AsObject>,
    }
  }

}

export class StoreAdminUserRequest extends jspb.Message {
  getFullName(): string;
  setFullName(value: string): StoreAdminUserRequest;

  getEmail(): string;
  setEmail(value: string): StoreAdminUserRequest;

  getPassword(): string;
  setPassword(value: string): StoreAdminUserRequest;

  getConfirmPassword(): string;
  setConfirmPassword(value: string): StoreAdminUserRequest;

  getIsSuperAdmin(): boolean;
  setIsSuperAdmin(value: boolean): StoreAdminUserRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): StoreAdminUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: StoreAdminUserRequest): StoreAdminUserRequest.AsObject;
  static serializeBinaryToWriter(message: StoreAdminUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): StoreAdminUserRequest;
  static deserializeBinaryFromReader(message: StoreAdminUserRequest, reader: jspb.BinaryReader): StoreAdminUserRequest;
}

export namespace StoreAdminUserRequest {
  export type AsObject = {
    fullName: string,
    email: string,
    password: string,
    confirmPassword: string,
    isSuperAdmin: boolean,
  }
}

export class StoreAdminUserResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): StoreAdminUserResponse;

  getData(): AdminUserModel | undefined;
  setData(value?: AdminUserModel): StoreAdminUserResponse;
  hasData(): boolean;
  clearData(): StoreAdminUserResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): StoreAdminUserResponse.AsObject;
  static toObject(includeInstance: boolean, msg: StoreAdminUserResponse): StoreAdminUserResponse.AsObject;
  static serializeBinaryToWriter(message: StoreAdminUserResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): StoreAdminUserResponse;
  static deserializeBinaryFromReader(message: StoreAdminUserResponse, reader: jspb.BinaryReader): StoreAdminUserResponse;
}

export namespace StoreAdminUserResponse {
  export type AsObject = {
    status: boolean,
    data?: AdminUserModel.AsObject,
  }
}

export class GetAdminUserRequest extends jspb.Message {
  getAdminUserId(): string;
  setAdminUserId(value: string): GetAdminUserRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetAdminUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetAdminUserRequest): GetAdminUserRequest.AsObject;
  static serializeBinaryToWriter(message: GetAdminUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetAdminUserRequest;
  static deserializeBinaryFromReader(message: GetAdminUserRequest, reader: jspb.BinaryReader): GetAdminUserRequest;
}

export namespace GetAdminUserRequest {
  export type AsObject = {
    adminUserId: string,
  }
}

export class GetAdminUserResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): GetAdminUserResponse;

  getData(): AdminUserModel | undefined;
  setData(value?: AdminUserModel): GetAdminUserResponse;
  hasData(): boolean;
  clearData(): GetAdminUserResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetAdminUserResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetAdminUserResponse): GetAdminUserResponse.AsObject;
  static serializeBinaryToWriter(message: GetAdminUserResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetAdminUserResponse;
  static deserializeBinaryFromReader(message: GetAdminUserResponse, reader: jspb.BinaryReader): GetAdminUserResponse;
}

export namespace GetAdminUserResponse {
  export type AsObject = {
    status: boolean,
    data?: AdminUserModel.AsObject,
  }
}

export class UpdateAdminUserRequest extends jspb.Message {
  getAdminUserId(): string;
  setAdminUserId(value: string): UpdateAdminUserRequest;

  getFullName(): string;
  setFullName(value: string): UpdateAdminUserRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateAdminUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateAdminUserRequest): UpdateAdminUserRequest.AsObject;
  static serializeBinaryToWriter(message: UpdateAdminUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateAdminUserRequest;
  static deserializeBinaryFromReader(message: UpdateAdminUserRequest, reader: jspb.BinaryReader): UpdateAdminUserRequest;
}

export namespace UpdateAdminUserRequest {
  export type AsObject = {
    adminUserId: string,
    fullName: string,
  }
}

export class UpdateAdminUserResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): UpdateAdminUserResponse;

  getData(): AdminUserModel | undefined;
  setData(value?: AdminUserModel): UpdateAdminUserResponse;
  hasData(): boolean;
  clearData(): UpdateAdminUserResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateAdminUserResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateAdminUserResponse): UpdateAdminUserResponse.AsObject;
  static serializeBinaryToWriter(message: UpdateAdminUserResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateAdminUserResponse;
  static deserializeBinaryFromReader(message: UpdateAdminUserResponse, reader: jspb.BinaryReader): UpdateAdminUserResponse;
}

export namespace UpdateAdminUserResponse {
  export type AsObject = {
    status: boolean,
    data?: AdminUserModel.AsObject,
  }
}


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

export class RoleModel extends jspb.Message {
  getId(): string;
  setId(value: string): RoleModel;

  getName(): string;
  setName(value: string): RoleModel;

  getIdentifier(): string;
  setIdentifier(value: string): RoleModel;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): RoleModel;
  hasCreatedAt(): boolean;
  clearCreatedAt(): RoleModel;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): RoleModel;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): RoleModel;

  getCreatedBy(): string;
  setCreatedBy(value: string): RoleModel;

  getUpdatedBy(): string;
  setUpdatedBy(value: string): RoleModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RoleModel.AsObject;
  static toObject(includeInstance: boolean, msg: RoleModel): RoleModel.AsObject;
  static serializeBinaryToWriter(message: RoleModel, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RoleModel;
  static deserializeBinaryFromReader(message: RoleModel, reader: jspb.BinaryReader): RoleModel;
}

export namespace RoleModel {
  export type AsObject = {
    id: string,
    name: string,
    identifier: string,
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

  getProfileImageContent(): Uint8Array | string;
  getProfileImageContent_asU8(): Uint8Array;
  getProfileImageContent_asB64(): string;
  setProfileImageContent(value: Uint8Array | string): StoreAdminUserRequest;

  getProfileImageFileName(): string;
  setProfileImageFileName(value: string): StoreAdminUserRequest;

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
    profileImageContent: Uint8Array | string,
    profileImageFileName: string,
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

  getProfileImageContent(): Uint8Array | string;
  getProfileImageContent_asU8(): Uint8Array;
  getProfileImageContent_asB64(): string;
  setProfileImageContent(value: Uint8Array | string): UpdateAdminUserRequest;

  getProfileImageFileName(): string;
  setProfileImageFileName(value: string): UpdateAdminUserRequest;

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
    profileImageContent: Uint8Array | string,
    profileImageFileName: string,
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

export class RolePaginateRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RolePaginateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: RolePaginateRequest): RolePaginateRequest.AsObject;
  static serializeBinaryToWriter(message: RolePaginateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RolePaginateRequest;
  static deserializeBinaryFromReader(message: RolePaginateRequest, reader: jspb.BinaryReader): RolePaginateRequest;
}

export namespace RolePaginateRequest {
  export type AsObject = {
  }
}

export class RolePaginateResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): RolePaginateResponse;

  getData(): RolePaginateResponse.RolePaginateData | undefined;
  setData(value?: RolePaginateResponse.RolePaginateData): RolePaginateResponse;
  hasData(): boolean;
  clearData(): RolePaginateResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RolePaginateResponse.AsObject;
  static toObject(includeInstance: boolean, msg: RolePaginateResponse): RolePaginateResponse.AsObject;
  static serializeBinaryToWriter(message: RolePaginateResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RolePaginateResponse;
  static deserializeBinaryFromReader(message: RolePaginateResponse, reader: jspb.BinaryReader): RolePaginateResponse;
}

export namespace RolePaginateResponse {
  export type AsObject = {
    status: boolean,
    data?: RolePaginateResponse.RolePaginateData.AsObject,
  }

  export class RolePagination extends jspb.Message {
    getTotal(): number;
    setTotal(value: number): RolePagination;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): RolePagination.AsObject;
    static toObject(includeInstance: boolean, msg: RolePagination): RolePagination.AsObject;
    static serializeBinaryToWriter(message: RolePagination, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): RolePagination;
    static deserializeBinaryFromReader(message: RolePagination, reader: jspb.BinaryReader): RolePagination;
  }

  export namespace RolePagination {
    export type AsObject = {
      total: number,
    }
  }


  export class RolePaginateData extends jspb.Message {
    getPagination(): RolePaginateResponse.RolePagination | undefined;
    setPagination(value?: RolePaginateResponse.RolePagination): RolePaginateData;
    hasPagination(): boolean;
    clearPagination(): RolePaginateData;

    getDataList(): Array<RoleModel>;
    setDataList(value: Array<RoleModel>): RolePaginateData;
    clearDataList(): RolePaginateData;
    addData(value?: RoleModel, index?: number): RoleModel;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): RolePaginateData.AsObject;
    static toObject(includeInstance: boolean, msg: RolePaginateData): RolePaginateData.AsObject;
    static serializeBinaryToWriter(message: RolePaginateData, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): RolePaginateData;
    static deserializeBinaryFromReader(message: RolePaginateData, reader: jspb.BinaryReader): RolePaginateData;
  }

  export namespace RolePaginateData {
    export type AsObject = {
      pagination?: RolePaginateResponse.RolePagination.AsObject,
      dataList: Array<RoleModel.AsObject>,
    }
  }

}


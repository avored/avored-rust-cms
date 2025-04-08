import * as jspb from 'google-protobuf'



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
    }
  }


  export class AdminUserPaginateData extends jspb.Message {
    getPagination(): AdminUserPaginateResponse.AdminUserPagination | undefined;
    setPagination(value?: AdminUserPaginateResponse.AdminUserPagination): AdminUserPaginateData;
    hasPagination(): boolean;
    clearPagination(): AdminUserPaginateData;

    getDataList(): Array<AdminUserPaginateResponse.AdminUserModel>;
    setDataList(value: Array<AdminUserPaginateResponse.AdminUserModel>): AdminUserPaginateData;
    clearDataList(): AdminUserPaginateData;
    addData(value?: AdminUserPaginateResponse.AdminUserModel, index?: number): AdminUserPaginateResponse.AdminUserModel;

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
      dataList: Array<AdminUserPaginateResponse.AdminUserModel.AsObject>,
    }
  }

}


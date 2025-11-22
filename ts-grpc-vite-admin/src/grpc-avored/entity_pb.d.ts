import * as jspb from 'google-protobuf'

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb'; // proto import: "google/protobuf/timestamp.proto"


export class EntityModel extends jspb.Message {
  getId(): string;
  setId(value: string): EntityModel;

  getName(): string;
  setName(value: string): EntityModel;

  getIdentifier(): string;
  setIdentifier(value: string): EntityModel;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): EntityModel;
  hasCreatedAt(): boolean;
  clearCreatedAt(): EntityModel;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): EntityModel;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): EntityModel;

  getCreatedBy(): string;
  setCreatedBy(value: string): EntityModel;

  getUpdatedBy(): string;
  setUpdatedBy(value: string): EntityModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EntityModel.AsObject;
  static toObject(includeInstance: boolean, msg: EntityModel): EntityModel.AsObject;
  static serializeBinaryToWriter(message: EntityModel, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EntityModel;
  static deserializeBinaryFromReader(message: EntityModel, reader: jspb.BinaryReader): EntityModel;
}

export namespace EntityModel {
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

export class StoreEntityRequest extends jspb.Message {
  getName(): string;
  setName(value: string): StoreEntityRequest;

  getIdentifier(): string;
  setIdentifier(value: string): StoreEntityRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): StoreEntityRequest.AsObject;
  static toObject(includeInstance: boolean, msg: StoreEntityRequest): StoreEntityRequest.AsObject;
  static serializeBinaryToWriter(message: StoreEntityRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): StoreEntityRequest;
  static deserializeBinaryFromReader(message: StoreEntityRequest, reader: jspb.BinaryReader): StoreEntityRequest;
}

export namespace StoreEntityRequest {
  export type AsObject = {
    name: string,
    identifier: string,
  }
}

export class StoreEntityResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): StoreEntityResponse;

  getData(): EntityModel | undefined;
  setData(value?: EntityModel): StoreEntityResponse;
  hasData(): boolean;
  clearData(): StoreEntityResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): StoreEntityResponse.AsObject;
  static toObject(includeInstance: boolean, msg: StoreEntityResponse): StoreEntityResponse.AsObject;
  static serializeBinaryToWriter(message: StoreEntityResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): StoreEntityResponse;
  static deserializeBinaryFromReader(message: StoreEntityResponse, reader: jspb.BinaryReader): StoreEntityResponse;
}

export namespace StoreEntityResponse {
  export type AsObject = {
    status: boolean,
    data?: EntityModel.AsObject,
  }
}

export class EntityPaginateRequest extends jspb.Message {
  getPage(): number;
  setPage(value: number): EntityPaginateRequest;
  hasPage(): boolean;
  clearPage(): EntityPaginateRequest;

  getOrder(): string;
  setOrder(value: string): EntityPaginateRequest;
  hasOrder(): boolean;
  clearOrder(): EntityPaginateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EntityPaginateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: EntityPaginateRequest): EntityPaginateRequest.AsObject;
  static serializeBinaryToWriter(message: EntityPaginateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EntityPaginateRequest;
  static deserializeBinaryFromReader(message: EntityPaginateRequest, reader: jspb.BinaryReader): EntityPaginateRequest;
}

export namespace EntityPaginateRequest {
  export type AsObject = {
    page?: number,
    order?: string,
  }

  export enum PageCase { 
    _PAGE_NOT_SET = 0,
    PAGE = 1,
  }

  export enum OrderCase { 
    _ORDER_NOT_SET = 0,
    ORDER = 2,
  }
}

export class EntityPaginateResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): EntityPaginateResponse;

  getData(): EntityPaginateResponse.EntityPaginateData | undefined;
  setData(value?: EntityPaginateResponse.EntityPaginateData): EntityPaginateResponse;
  hasData(): boolean;
  clearData(): EntityPaginateResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): EntityPaginateResponse.AsObject;
  static toObject(includeInstance: boolean, msg: EntityPaginateResponse): EntityPaginateResponse.AsObject;
  static serializeBinaryToWriter(message: EntityPaginateResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): EntityPaginateResponse;
  static deserializeBinaryFromReader(message: EntityPaginateResponse, reader: jspb.BinaryReader): EntityPaginateResponse;
}

export namespace EntityPaginateResponse {
  export type AsObject = {
    status: boolean,
    data?: EntityPaginateResponse.EntityPaginateData.AsObject,
  }

  export class EntityPagination extends jspb.Message {
    getTotal(): number;
    setTotal(value: number): EntityPagination;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): EntityPagination.AsObject;
    static toObject(includeInstance: boolean, msg: EntityPagination): EntityPagination.AsObject;
    static serializeBinaryToWriter(message: EntityPagination, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): EntityPagination;
    static deserializeBinaryFromReader(message: EntityPagination, reader: jspb.BinaryReader): EntityPagination;
  }

  export namespace EntityPagination {
    export type AsObject = {
      total: number,
    }
  }


  export class EntityPaginateData extends jspb.Message {
    getPagination(): EntityPaginateResponse.EntityPagination | undefined;
    setPagination(value?: EntityPaginateResponse.EntityPagination): EntityPaginateData;
    hasPagination(): boolean;
    clearPagination(): EntityPaginateData;

    getDataList(): Array<EntityModel>;
    setDataList(value: Array<EntityModel>): EntityPaginateData;
    clearDataList(): EntityPaginateData;
    addData(value?: EntityModel, index?: number): EntityModel;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): EntityPaginateData.AsObject;
    static toObject(includeInstance: boolean, msg: EntityPaginateData): EntityPaginateData.AsObject;
    static serializeBinaryToWriter(message: EntityPaginateData, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): EntityPaginateData;
    static deserializeBinaryFromReader(message: EntityPaginateData, reader: jspb.BinaryReader): EntityPaginateData;
  }

  export namespace EntityPaginateData {
    export type AsObject = {
      pagination?: EntityPaginateResponse.EntityPagination.AsObject,
      dataList: Array<EntityModel.AsObject>,
    }
  }

}


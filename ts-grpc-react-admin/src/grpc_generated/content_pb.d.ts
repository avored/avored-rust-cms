import * as jspb from 'google-protobuf'

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb'; // proto import: "google/protobuf/timestamp.proto"


export class CollectionModel extends jspb.Message {
  getId(): string;
  setId(value: string): CollectionModel;

  getName(): string;
  setName(value: string): CollectionModel;

  getIdentifier(): string;
  setIdentifier(value: string): CollectionModel;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): CollectionModel;
  hasCreatedAt(): boolean;
  clearCreatedAt(): CollectionModel;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): CollectionModel;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): CollectionModel;

  getCreatedBy(): string;
  setCreatedBy(value: string): CollectionModel;

  getUpdatedBy(): string;
  setUpdatedBy(value: string): CollectionModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CollectionModel.AsObject;
  static toObject(includeInstance: boolean, msg: CollectionModel): CollectionModel.AsObject;
  static serializeBinaryToWriter(message: CollectionModel, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CollectionModel;
  static deserializeBinaryFromReader(message: CollectionModel, reader: jspb.BinaryReader): CollectionModel;
}

export namespace CollectionModel {
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

export class CollectionAllRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CollectionAllRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CollectionAllRequest): CollectionAllRequest.AsObject;
  static serializeBinaryToWriter(message: CollectionAllRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CollectionAllRequest;
  static deserializeBinaryFromReader(message: CollectionAllRequest, reader: jspb.BinaryReader): CollectionAllRequest;
}

export namespace CollectionAllRequest {
  export type AsObject = {
  }
}

export class CollectionAllResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): CollectionAllResponse;

  getDataList(): Array<CollectionModel>;
  setDataList(value: Array<CollectionModel>): CollectionAllResponse;
  clearDataList(): CollectionAllResponse;
  addData(value?: CollectionModel, index?: number): CollectionModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CollectionAllResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CollectionAllResponse): CollectionAllResponse.AsObject;
  static serializeBinaryToWriter(message: CollectionAllResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CollectionAllResponse;
  static deserializeBinaryFromReader(message: CollectionAllResponse, reader: jspb.BinaryReader): CollectionAllResponse;
}

export namespace CollectionAllResponse {
  export type AsObject = {
    status: boolean,
    dataList: Array<CollectionModel.AsObject>,
  }
}


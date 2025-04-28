import * as jspb from 'google-protobuf'

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb'; // proto import: "google/protobuf/timestamp.proto"


export class SettingModel extends jspb.Message {
  getId(): string;
  setId(value: string): SettingModel;

  getIdentifier(): string;
  setIdentifier(value: string): SettingModel;

  getValue(): string;
  setValue(value: string): SettingModel;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): SettingModel;
  hasCreatedAt(): boolean;
  clearCreatedAt(): SettingModel;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): SettingModel;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): SettingModel;

  getCreatedBy(): string;
  setCreatedBy(value: string): SettingModel;

  getUpdatedBy(): string;
  setUpdatedBy(value: string): SettingModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SettingModel.AsObject;
  static toObject(includeInstance: boolean, msg: SettingModel): SettingModel.AsObject;
  static serializeBinaryToWriter(message: SettingModel, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SettingModel;
  static deserializeBinaryFromReader(message: SettingModel, reader: jspb.BinaryReader): SettingModel;
}

export namespace SettingModel {
  export type AsObject = {
    id: string,
    identifier: string,
    value: string,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    createdBy: string,
    updatedBy: string,
  }
}

export class GetSettingRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetSettingRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetSettingRequest): GetSettingRequest.AsObject;
  static serializeBinaryToWriter(message: GetSettingRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetSettingRequest;
  static deserializeBinaryFromReader(message: GetSettingRequest, reader: jspb.BinaryReader): GetSettingRequest;
}

export namespace GetSettingRequest {
  export type AsObject = {
  }
}

export class GetSettingResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): GetSettingResponse;

  getDataList(): Array<SettingModel>;
  setDataList(value: Array<SettingModel>): GetSettingResponse;
  clearDataList(): GetSettingResponse;
  addData(value?: SettingModel, index?: number): SettingModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetSettingResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetSettingResponse): GetSettingResponse.AsObject;
  static serializeBinaryToWriter(message: GetSettingResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetSettingResponse;
  static deserializeBinaryFromReader(message: GetSettingResponse, reader: jspb.BinaryReader): GetSettingResponse;
}

export namespace GetSettingResponse {
  export type AsObject = {
    status: boolean,
    dataList: Array<SettingModel.AsObject>,
  }
}


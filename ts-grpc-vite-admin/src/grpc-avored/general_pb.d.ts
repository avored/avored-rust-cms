import * as jspb from 'google-protobuf'

import * as admin_user_pb from './admin_user_pb'; // proto import: "admin_user.proto"


export class LoggedInUserRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LoggedInUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: LoggedInUserRequest): LoggedInUserRequest.AsObject;
  static serializeBinaryToWriter(message: LoggedInUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LoggedInUserRequest;
  static deserializeBinaryFromReader(message: LoggedInUserRequest, reader: jspb.BinaryReader): LoggedInUserRequest;
}

export namespace LoggedInUserRequest {
  export type AsObject = {
  }
}

export class LoggedInUserResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): LoggedInUserResponse;

  getData(): admin_user_pb.AdminUserModel | undefined;
  setData(value?: admin_user_pb.AdminUserModel): LoggedInUserResponse;
  hasData(): boolean;
  clearData(): LoggedInUserResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LoggedInUserResponse.AsObject;
  static toObject(includeInstance: boolean, msg: LoggedInUserResponse): LoggedInUserResponse.AsObject;
  static serializeBinaryToWriter(message: LoggedInUserResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LoggedInUserResponse;
  static deserializeBinaryFromReader(message: LoggedInUserResponse, reader: jspb.BinaryReader): LoggedInUserResponse;
}

export namespace LoggedInUserResponse {
  export type AsObject = {
    status: boolean,
    data?: admin_user_pb.AdminUserModel.AsObject,
  }
}


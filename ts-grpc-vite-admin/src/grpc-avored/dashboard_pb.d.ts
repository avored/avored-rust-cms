import * as jspb from 'google-protobuf'



export class DashboardRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DashboardRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DashboardRequest): DashboardRequest.AsObject;
  static serializeBinaryToWriter(message: DashboardRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DashboardRequest;
  static deserializeBinaryFromReader(message: DashboardRequest, reader: jspb.BinaryReader): DashboardRequest;
}

export namespace DashboardRequest {
  export type AsObject = {
  }
}

export class DashboardResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): DashboardResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DashboardResponse.AsObject;
  static toObject(includeInstance: boolean, msg: DashboardResponse): DashboardResponse.AsObject;
  static serializeBinaryToWriter(message: DashboardResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DashboardResponse;
  static deserializeBinaryFromReader(message: DashboardResponse, reader: jspb.BinaryReader): DashboardResponse;
}

export namespace DashboardResponse {
  export type AsObject = {
    status: boolean,
  }
}


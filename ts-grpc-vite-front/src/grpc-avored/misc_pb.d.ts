import * as jspb from 'google-protobuf'



export class SetupRequest extends jspb.Message {
  getEmail(): string;
  setEmail(value: string): SetupRequest;

  getPassword(): string;
  setPassword(value: string): SetupRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SetupRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SetupRequest): SetupRequest.AsObject;
  static serializeBinaryToWriter(message: SetupRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SetupRequest;
  static deserializeBinaryFromReader(message: SetupRequest, reader: jspb.BinaryReader): SetupRequest;
}

export namespace SetupRequest {
  export type AsObject = {
    email: string,
    password: string,
  }
}

export class SetupResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): SetupResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SetupResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SetupResponse): SetupResponse.AsObject;
  static serializeBinaryToWriter(message: SetupResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SetupResponse;
  static deserializeBinaryFromReader(message: SetupResponse, reader: jspb.BinaryReader): SetupResponse;
}

export namespace SetupResponse {
  export type AsObject = {
    status: boolean,
  }
}

export class HealthCheckRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): HealthCheckRequest.AsObject;
  static toObject(includeInstance: boolean, msg: HealthCheckRequest): HealthCheckRequest.AsObject;
  static serializeBinaryToWriter(message: HealthCheckRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): HealthCheckRequest;
  static deserializeBinaryFromReader(message: HealthCheckRequest, reader: jspb.BinaryReader): HealthCheckRequest;
}

export namespace HealthCheckRequest {
  export type AsObject = {
  }
}

export class HealthCheckResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): HealthCheckResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): HealthCheckResponse.AsObject;
  static toObject(includeInstance: boolean, msg: HealthCheckResponse): HealthCheckResponse.AsObject;
  static serializeBinaryToWriter(message: HealthCheckResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): HealthCheckResponse;
  static deserializeBinaryFromReader(message: HealthCheckResponse, reader: jspb.BinaryReader): HealthCheckResponse;
}

export namespace HealthCheckResponse {
  export type AsObject = {
    status: boolean,
  }
}

export class InstallDemoDataRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): InstallDemoDataRequest.AsObject;
  static toObject(includeInstance: boolean, msg: InstallDemoDataRequest): InstallDemoDataRequest.AsObject;
  static serializeBinaryToWriter(message: InstallDemoDataRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): InstallDemoDataRequest;
  static deserializeBinaryFromReader(message: InstallDemoDataRequest, reader: jspb.BinaryReader): InstallDemoDataRequest;
}

export namespace InstallDemoDataRequest {
  export type AsObject = {
  }
}

export class InstallDemoDataResponse extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): InstallDemoDataResponse.AsObject;
  static toObject(includeInstance: boolean, msg: InstallDemoDataResponse): InstallDemoDataResponse.AsObject;
  static serializeBinaryToWriter(message: InstallDemoDataResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): InstallDemoDataResponse;
  static deserializeBinaryFromReader(message: InstallDemoDataResponse, reader: jspb.BinaryReader): InstallDemoDataResponse;
}

export namespace InstallDemoDataResponse {
  export type AsObject = {
  }
}

export class DeleteDemoDataRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteDemoDataRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteDemoDataRequest): DeleteDemoDataRequest.AsObject;
  static serializeBinaryToWriter(message: DeleteDemoDataRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteDemoDataRequest;
  static deserializeBinaryFromReader(message: DeleteDemoDataRequest, reader: jspb.BinaryReader): DeleteDemoDataRequest;
}

export namespace DeleteDemoDataRequest {
  export type AsObject = {
  }
}

export class DeleteDemoDataResponse extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteDemoDataResponse.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteDemoDataResponse): DeleteDemoDataResponse.AsObject;
  static serializeBinaryToWriter(message: DeleteDemoDataResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteDemoDataResponse;
  static deserializeBinaryFromReader(message: DeleteDemoDataResponse, reader: jspb.BinaryReader): DeleteDemoDataResponse;
}

export namespace DeleteDemoDataResponse {
  export type AsObject = {
  }
}


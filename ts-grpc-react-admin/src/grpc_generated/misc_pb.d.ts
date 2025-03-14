import * as jspb from 'google-protobuf'


export class SetupRequest extends jspb.Message {
    static toObject(includeInstance: boolean, msg: SetupRequest): SetupRequest.AsObject;

    static serializeBinaryToWriter(message: SetupRequest, writer: jspb.BinaryWriter): void;

    static deserializeBinary(bytes: Uint8Array): SetupRequest;

    static deserializeBinaryFromReader(message: SetupRequest, reader: jspb.BinaryReader): SetupRequest;

    getEmail(): string;

    setEmail(value: string): SetupRequest;

    getPassword(): string;

    setPassword(value: string): SetupRequest;

    serializeBinary(): Uint8Array;

    toObject(includeInstance?: boolean): SetupRequest.AsObject;
}

export namespace SetupRequest {
    export type AsObject = {
        email: string,
        password: string,
    }
}

export class SetupResponse extends jspb.Message {
    static toObject(includeInstance: boolean, msg: SetupResponse): SetupResponse.AsObject;

    static serializeBinaryToWriter(message: SetupResponse, writer: jspb.BinaryWriter): void;

    static deserializeBinary(bytes: Uint8Array): SetupResponse;

    static deserializeBinaryFromReader(message: SetupResponse, reader: jspb.BinaryReader): SetupResponse;

    getStatus(): boolean;

    setStatus(value: boolean): SetupResponse;

    serializeBinary(): Uint8Array;

    toObject(includeInstance?: boolean): SetupResponse.AsObject;
}

export namespace SetupResponse {
    export type AsObject = {
        status: boolean,
    }
}

export class HealthCheckRequest extends jspb.Message {
    static toObject(includeInstance: boolean, msg: HealthCheckRequest): HealthCheckRequest.AsObject;

    static serializeBinaryToWriter(message: HealthCheckRequest, writer: jspb.BinaryWriter): void;

    static deserializeBinary(bytes: Uint8Array): HealthCheckRequest;

    static deserializeBinaryFromReader(message: HealthCheckRequest, reader: jspb.BinaryReader): HealthCheckRequest;

    serializeBinary(): Uint8Array;

    toObject(includeInstance?: boolean): HealthCheckRequest.AsObject;
}

export namespace HealthCheckRequest {
    export type AsObject = {}
}

export class HealthCheckResponse extends jspb.Message {
    static toObject(includeInstance: boolean, msg: HealthCheckResponse): HealthCheckResponse.AsObject;

    static serializeBinaryToWriter(message: HealthCheckResponse, writer: jspb.BinaryWriter): void;

    static deserializeBinary(bytes: Uint8Array): HealthCheckResponse;

    static deserializeBinaryFromReader(message: HealthCheckResponse, reader: jspb.BinaryReader): HealthCheckResponse;

    getStatus(): boolean;

    setStatus(value: boolean): HealthCheckResponse;

    serializeBinary(): Uint8Array;

    toObject(includeInstance?: boolean): HealthCheckResponse.AsObject;
}

export namespace HealthCheckResponse {
    export type AsObject = {
        status: boolean,
    }
}


import * as jspb from 'google-protobuf'

import * as content_pb from './content_pb'; // proto import: "content.proto"


export class GetCmsContentRequest extends jspb.Message {
  getContentId(): string;
  setContentId(value: string): GetCmsContentRequest;

  getContentType(): string;
  setContentType(value: string): GetCmsContentRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetCmsContentRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetCmsContentRequest): GetCmsContentRequest.AsObject;
  static serializeBinaryToWriter(message: GetCmsContentRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetCmsContentRequest;
  static deserializeBinaryFromReader(message: GetCmsContentRequest, reader: jspb.BinaryReader): GetCmsContentRequest;
}

export namespace GetCmsContentRequest {
  export type AsObject = {
    contentId: string,
    contentType: string,
  }
}

export class GetCmsContentResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): GetCmsContentResponse;

  getData(): content_pb.ContentModel | undefined;
  setData(value?: content_pb.ContentModel): GetCmsContentResponse;
  hasData(): boolean;
  clearData(): GetCmsContentResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetCmsContentResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetCmsContentResponse): GetCmsContentResponse.AsObject;
  static serializeBinaryToWriter(message: GetCmsContentResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetCmsContentResponse;
  static deserializeBinaryFromReader(message: GetCmsContentResponse, reader: jspb.BinaryReader): GetCmsContentResponse;
}

export namespace GetCmsContentResponse {
  export type AsObject = {
    status: boolean,
    data?: content_pb.ContentModel.AsObject,
  }
}

export class SentContactFormRequest extends jspb.Message {
  getFirstName(): string;
  setFirstName(value: string): SentContactFormRequest;

  getLastName(): string;
  setLastName(value: string): SentContactFormRequest;

  getEmail(): string;
  setEmail(value: string): SentContactFormRequest;

  getPhone(): string;
  setPhone(value: string): SentContactFormRequest;

  getMessage(): string;
  setMessage(value: string): SentContactFormRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SentContactFormRequest.AsObject;
  static toObject(includeInstance: boolean, msg: SentContactFormRequest): SentContactFormRequest.AsObject;
  static serializeBinaryToWriter(message: SentContactFormRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SentContactFormRequest;
  static deserializeBinaryFromReader(message: SentContactFormRequest, reader: jspb.BinaryReader): SentContactFormRequest;
}

export namespace SentContactFormRequest {
  export type AsObject = {
    firstName: string,
    lastName: string,
    email: string,
    phone: string,
    message: string,
  }
}

export class SentContactFormResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): SentContactFormResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SentContactFormResponse.AsObject;
  static toObject(includeInstance: boolean, msg: SentContactFormResponse): SentContactFormResponse.AsObject;
  static serializeBinaryToWriter(message: SentContactFormResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SentContactFormResponse;
  static deserializeBinaryFromReader(message: SentContactFormResponse, reader: jspb.BinaryReader): SentContactFormResponse;
}

export namespace SentContactFormResponse {
  export type AsObject = {
    status: boolean,
  }
}


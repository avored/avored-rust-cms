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


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

export class GetCollectionRequest extends jspb.Message {
  getCollectionId(): string;
  setCollectionId(value: string): GetCollectionRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetCollectionRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetCollectionRequest): GetCollectionRequest.AsObject;
  static serializeBinaryToWriter(message: GetCollectionRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetCollectionRequest;
  static deserializeBinaryFromReader(message: GetCollectionRequest, reader: jspb.BinaryReader): GetCollectionRequest;
}

export namespace GetCollectionRequest {
  export type AsObject = {
    collectionId: string,
  }
}

export class GetCollectionResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): GetCollectionResponse;

  getData(): CollectionModel | undefined;
  setData(value?: CollectionModel): GetCollectionResponse;
  hasData(): boolean;
  clearData(): GetCollectionResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetCollectionResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetCollectionResponse): GetCollectionResponse.AsObject;
  static serializeBinaryToWriter(message: GetCollectionResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetCollectionResponse;
  static deserializeBinaryFromReader(message: GetCollectionResponse, reader: jspb.BinaryReader): GetCollectionResponse;
}

export namespace GetCollectionResponse {
  export type AsObject = {
    status: boolean,
    data?: CollectionModel.AsObject,
  }
}

export class StoreCollectionRequest extends jspb.Message {
  getName(): string;
  setName(value: string): StoreCollectionRequest;

  getIdentifier(): string;
  setIdentifier(value: string): StoreCollectionRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): StoreCollectionRequest.AsObject;
  static toObject(includeInstance: boolean, msg: StoreCollectionRequest): StoreCollectionRequest.AsObject;
  static serializeBinaryToWriter(message: StoreCollectionRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): StoreCollectionRequest;
  static deserializeBinaryFromReader(message: StoreCollectionRequest, reader: jspb.BinaryReader): StoreCollectionRequest;
}

export namespace StoreCollectionRequest {
  export type AsObject = {
    name: string,
    identifier: string,
  }
}

export class StoreCollectionResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): StoreCollectionResponse;

  getData(): CollectionModel | undefined;
  setData(value?: CollectionModel): StoreCollectionResponse;
  hasData(): boolean;
  clearData(): StoreCollectionResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): StoreCollectionResponse.AsObject;
  static toObject(includeInstance: boolean, msg: StoreCollectionResponse): StoreCollectionResponse.AsObject;
  static serializeBinaryToWriter(message: StoreCollectionResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): StoreCollectionResponse;
  static deserializeBinaryFromReader(message: StoreCollectionResponse, reader: jspb.BinaryReader): StoreCollectionResponse;
}

export namespace StoreCollectionResponse {
  export type AsObject = {
    status: boolean,
    data?: CollectionModel.AsObject,
  }
}

export class UpdateCollectionRequest extends jspb.Message {
  getId(): string;
  setId(value: string): UpdateCollectionRequest;

  getName(): string;
  setName(value: string): UpdateCollectionRequest;

  getIdentifier(): string;
  setIdentifier(value: string): UpdateCollectionRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateCollectionRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateCollectionRequest): UpdateCollectionRequest.AsObject;
  static serializeBinaryToWriter(message: UpdateCollectionRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateCollectionRequest;
  static deserializeBinaryFromReader(message: UpdateCollectionRequest, reader: jspb.BinaryReader): UpdateCollectionRequest;
}

export namespace UpdateCollectionRequest {
  export type AsObject = {
    id: string,
    name: string,
    identifier: string,
  }
}

export class UpdateCollectionResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): UpdateCollectionResponse;

  getData(): CollectionModel | undefined;
  setData(value?: CollectionModel): UpdateCollectionResponse;
  hasData(): boolean;
  clearData(): UpdateCollectionResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateCollectionResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateCollectionResponse): UpdateCollectionResponse.AsObject;
  static serializeBinaryToWriter(message: UpdateCollectionResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateCollectionResponse;
  static deserializeBinaryFromReader(message: UpdateCollectionResponse, reader: jspb.BinaryReader): UpdateCollectionResponse;
}

export namespace UpdateCollectionResponse {
  export type AsObject = {
    status: boolean,
    data?: CollectionModel.AsObject,
  }
}

export class ContentFieldFieldContent extends jspb.Message {
  getTextValue(): string;
  setTextValue(value: string): ContentFieldFieldContent;
  hasTextValue(): boolean;
  clearTextValue(): ContentFieldFieldContent;

  getIntValue(): number;
  setIntValue(value: number): ContentFieldFieldContent;
  hasIntValue(): boolean;
  clearIntValue(): ContentFieldFieldContent;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ContentFieldFieldContent.AsObject;
  static toObject(includeInstance: boolean, msg: ContentFieldFieldContent): ContentFieldFieldContent.AsObject;
  static serializeBinaryToWriter(message: ContentFieldFieldContent, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ContentFieldFieldContent;
  static deserializeBinaryFromReader(message: ContentFieldFieldContent, reader: jspb.BinaryReader): ContentFieldFieldContent;
}

export namespace ContentFieldFieldContent {
  export type AsObject = {
    textValue?: string,
    intValue?: number,
  }

  export enum TextValueCase { 
    _TEXT_VALUE_NOT_SET = 0,
    TEXT_VALUE = 1,
  }

  export enum IntValueCase { 
    _INT_VALUE_NOT_SET = 0,
    INT_VALUE = 2,
  }
}

export class ContentFieldModel extends jspb.Message {
  getName(): string;
  setName(value: string): ContentFieldModel;

  getIdentifier(): string;
  setIdentifier(value: string): ContentFieldModel;

  getDataType(): string;
  setDataType(value: string): ContentFieldModel;

  getFieldType(): string;
  setFieldType(value: string): ContentFieldModel;

  getFieldContent(): ContentFieldFieldContent | undefined;
  setFieldContent(value?: ContentFieldFieldContent): ContentFieldModel;
  hasFieldContent(): boolean;
  clearFieldContent(): ContentFieldModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ContentFieldModel.AsObject;
  static toObject(includeInstance: boolean, msg: ContentFieldModel): ContentFieldModel.AsObject;
  static serializeBinaryToWriter(message: ContentFieldModel, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ContentFieldModel;
  static deserializeBinaryFromReader(message: ContentFieldModel, reader: jspb.BinaryReader): ContentFieldModel;
}

export namespace ContentFieldModel {
  export type AsObject = {
    name: string,
    identifier: string,
    dataType: string,
    fieldType: string,
    fieldContent?: ContentFieldFieldContent.AsObject,
  }
}

export class ContentModel extends jspb.Message {
  getId(): string;
  setId(value: string): ContentModel;

  getName(): string;
  setName(value: string): ContentModel;

  getIdentifier(): string;
  setIdentifier(value: string): ContentModel;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): ContentModel;
  hasCreatedAt(): boolean;
  clearCreatedAt(): ContentModel;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): ContentModel;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): ContentModel;

  getCreatedBy(): string;
  setCreatedBy(value: string): ContentModel;

  getUpdatedBy(): string;
  setUpdatedBy(value: string): ContentModel;

  getContentFieldsList(): Array<ContentFieldModel>;
  setContentFieldsList(value: Array<ContentFieldModel>): ContentModel;
  clearContentFieldsList(): ContentModel;
  addContentFields(value?: ContentFieldModel, index?: number): ContentFieldModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ContentModel.AsObject;
  static toObject(includeInstance: boolean, msg: ContentModel): ContentModel.AsObject;
  static serializeBinaryToWriter(message: ContentModel, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ContentModel;
  static deserializeBinaryFromReader(message: ContentModel, reader: jspb.BinaryReader): ContentModel;
}

export namespace ContentModel {
  export type AsObject = {
    id: string,
    name: string,
    identifier: string,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    createdBy: string,
    updatedBy: string,
    contentFieldsList: Array<ContentFieldModel.AsObject>,
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

export class ContentPaginateRequest extends jspb.Message {
  getContentType(): string;
  setContentType(value: string): ContentPaginateRequest;

  getPage(): number;
  setPage(value: number): ContentPaginateRequest;
  hasPage(): boolean;
  clearPage(): ContentPaginateRequest;

  getOrder(): string;
  setOrder(value: string): ContentPaginateRequest;
  hasOrder(): boolean;
  clearOrder(): ContentPaginateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ContentPaginateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ContentPaginateRequest): ContentPaginateRequest.AsObject;
  static serializeBinaryToWriter(message: ContentPaginateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ContentPaginateRequest;
  static deserializeBinaryFromReader(message: ContentPaginateRequest, reader: jspb.BinaryReader): ContentPaginateRequest;
}

export namespace ContentPaginateRequest {
  export type AsObject = {
    contentType: string,
    page?: number,
    order?: string,
  }

  export enum PageCase { 
    _PAGE_NOT_SET = 0,
    PAGE = 2,
  }

  export enum OrderCase { 
    _ORDER_NOT_SET = 0,
    ORDER = 3,
  }
}

export class ContentPaginateResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): ContentPaginateResponse;

  getData(): ContentPaginateResponse.ContentPaginateData | undefined;
  setData(value?: ContentPaginateResponse.ContentPaginateData): ContentPaginateResponse;
  hasData(): boolean;
  clearData(): ContentPaginateResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ContentPaginateResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ContentPaginateResponse): ContentPaginateResponse.AsObject;
  static serializeBinaryToWriter(message: ContentPaginateResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ContentPaginateResponse;
  static deserializeBinaryFromReader(message: ContentPaginateResponse, reader: jspb.BinaryReader): ContentPaginateResponse;
}

export namespace ContentPaginateResponse {
  export type AsObject = {
    status: boolean,
    data?: ContentPaginateResponse.ContentPaginateData.AsObject,
  }

  export class ContentPagination extends jspb.Message {
    getTotal(): number;
    setTotal(value: number): ContentPagination;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): ContentPagination.AsObject;
    static toObject(includeInstance: boolean, msg: ContentPagination): ContentPagination.AsObject;
    static serializeBinaryToWriter(message: ContentPagination, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): ContentPagination;
    static deserializeBinaryFromReader(message: ContentPagination, reader: jspb.BinaryReader): ContentPagination;
  }

  export namespace ContentPagination {
    export type AsObject = {
      total: number,
    }
  }


  export class ContentPaginateData extends jspb.Message {
    getPagination(): ContentPaginateResponse.ContentPagination | undefined;
    setPagination(value?: ContentPaginateResponse.ContentPagination): ContentPaginateData;
    hasPagination(): boolean;
    clearPagination(): ContentPaginateData;

    getDataList(): Array<ContentModel>;
    setDataList(value: Array<ContentModel>): ContentPaginateData;
    clearDataList(): ContentPaginateData;
    addData(value?: ContentModel, index?: number): ContentModel;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): ContentPaginateData.AsObject;
    static toObject(includeInstance: boolean, msg: ContentPaginateData): ContentPaginateData.AsObject;
    static serializeBinaryToWriter(message: ContentPaginateData, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): ContentPaginateData;
    static deserializeBinaryFromReader(message: ContentPaginateData, reader: jspb.BinaryReader): ContentPaginateData;
  }

  export namespace ContentPaginateData {
    export type AsObject = {
      pagination?: ContentPaginateResponse.ContentPagination.AsObject,
      dataList: Array<ContentModel.AsObject>,
    }
  }

}

export class StoreContentFieldModel extends jspb.Message {
  getName(): string;
  setName(value: string): StoreContentFieldModel;

  getIdentifier(): string;
  setIdentifier(value: string): StoreContentFieldModel;

  getDataType(): string;
  setDataType(value: string): StoreContentFieldModel;

  getFieldType(): string;
  setFieldType(value: string): StoreContentFieldModel;

  getFieldContent(): ContentFieldFieldContent | undefined;
  setFieldContent(value?: ContentFieldFieldContent): StoreContentFieldModel;
  hasFieldContent(): boolean;
  clearFieldContent(): StoreContentFieldModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): StoreContentFieldModel.AsObject;
  static toObject(includeInstance: boolean, msg: StoreContentFieldModel): StoreContentFieldModel.AsObject;
  static serializeBinaryToWriter(message: StoreContentFieldModel, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): StoreContentFieldModel;
  static deserializeBinaryFromReader(message: StoreContentFieldModel, reader: jspb.BinaryReader): StoreContentFieldModel;
}

export namespace StoreContentFieldModel {
  export type AsObject = {
    name: string,
    identifier: string,
    dataType: string,
    fieldType: string,
    fieldContent?: ContentFieldFieldContent.AsObject,
  }
}

export class StoreContentRequest extends jspb.Message {
  getName(): string;
  setName(value: string): StoreContentRequest;

  getIdentifier(): string;
  setIdentifier(value: string): StoreContentRequest;

  getContentType(): string;
  setContentType(value: string): StoreContentRequest;

  getContentFieldsList(): Array<StoreContentFieldModel>;
  setContentFieldsList(value: Array<StoreContentFieldModel>): StoreContentRequest;
  clearContentFieldsList(): StoreContentRequest;
  addContentFields(value?: StoreContentFieldModel, index?: number): StoreContentFieldModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): StoreContentRequest.AsObject;
  static toObject(includeInstance: boolean, msg: StoreContentRequest): StoreContentRequest.AsObject;
  static serializeBinaryToWriter(message: StoreContentRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): StoreContentRequest;
  static deserializeBinaryFromReader(message: StoreContentRequest, reader: jspb.BinaryReader): StoreContentRequest;
}

export namespace StoreContentRequest {
  export type AsObject = {
    name: string,
    identifier: string,
    contentType: string,
    contentFieldsList: Array<StoreContentFieldModel.AsObject>,
  }
}

export class StoreContentResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): StoreContentResponse;

  getData(): ContentModel | undefined;
  setData(value?: ContentModel): StoreContentResponse;
  hasData(): boolean;
  clearData(): StoreContentResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): StoreContentResponse.AsObject;
  static toObject(includeInstance: boolean, msg: StoreContentResponse): StoreContentResponse.AsObject;
  static serializeBinaryToWriter(message: StoreContentResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): StoreContentResponse;
  static deserializeBinaryFromReader(message: StoreContentResponse, reader: jspb.BinaryReader): StoreContentResponse;
}

export namespace StoreContentResponse {
  export type AsObject = {
    status: boolean,
    data?: ContentModel.AsObject,
  }
}

export class GetContentRequest extends jspb.Message {
  getContentId(): string;
  setContentId(value: string): GetContentRequest;

  getContentType(): string;
  setContentType(value: string): GetContentRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetContentRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetContentRequest): GetContentRequest.AsObject;
  static serializeBinaryToWriter(message: GetContentRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetContentRequest;
  static deserializeBinaryFromReader(message: GetContentRequest, reader: jspb.BinaryReader): GetContentRequest;
}

export namespace GetContentRequest {
  export type AsObject = {
    contentId: string,
    contentType: string,
  }
}

export class GetContentResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): GetContentResponse;

  getData(): ContentModel | undefined;
  setData(value?: ContentModel): GetContentResponse;
  hasData(): boolean;
  clearData(): GetContentResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetContentResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetContentResponse): GetContentResponse.AsObject;
  static serializeBinaryToWriter(message: GetContentResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetContentResponse;
  static deserializeBinaryFromReader(message: GetContentResponse, reader: jspb.BinaryReader): GetContentResponse;
}

export namespace GetContentResponse {
  export type AsObject = {
    status: boolean,
    data?: ContentModel.AsObject,
  }
}

export class UpdateContentFieldModel extends jspb.Message {
  getName(): string;
  setName(value: string): UpdateContentFieldModel;

  getIdentifier(): string;
  setIdentifier(value: string): UpdateContentFieldModel;

  getDataType(): string;
  setDataType(value: string): UpdateContentFieldModel;

  getFieldType(): string;
  setFieldType(value: string): UpdateContentFieldModel;

  getFieldContent(): ContentFieldFieldContent | undefined;
  setFieldContent(value?: ContentFieldFieldContent): UpdateContentFieldModel;
  hasFieldContent(): boolean;
  clearFieldContent(): UpdateContentFieldModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateContentFieldModel.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateContentFieldModel): UpdateContentFieldModel.AsObject;
  static serializeBinaryToWriter(message: UpdateContentFieldModel, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateContentFieldModel;
  static deserializeBinaryFromReader(message: UpdateContentFieldModel, reader: jspb.BinaryReader): UpdateContentFieldModel;
}

export namespace UpdateContentFieldModel {
  export type AsObject = {
    name: string,
    identifier: string,
    dataType: string,
    fieldType: string,
    fieldContent?: ContentFieldFieldContent.AsObject,
  }
}

export class UpdateContentRequest extends jspb.Message {
  getContentId(): string;
  setContentId(value: string): UpdateContentRequest;

  getName(): string;
  setName(value: string): UpdateContentRequest;

  getContentType(): string;
  setContentType(value: string): UpdateContentRequest;

  getContentFieldsList(): Array<UpdateContentFieldModel>;
  setContentFieldsList(value: Array<UpdateContentFieldModel>): UpdateContentRequest;
  clearContentFieldsList(): UpdateContentRequest;
  addContentFields(value?: UpdateContentFieldModel, index?: number): UpdateContentFieldModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateContentRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateContentRequest): UpdateContentRequest.AsObject;
  static serializeBinaryToWriter(message: UpdateContentRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateContentRequest;
  static deserializeBinaryFromReader(message: UpdateContentRequest, reader: jspb.BinaryReader): UpdateContentRequest;
}

export namespace UpdateContentRequest {
  export type AsObject = {
    contentId: string,
    name: string,
    contentType: string,
    contentFieldsList: Array<UpdateContentFieldModel.AsObject>,
  }
}

export class UpdateContentResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): UpdateContentResponse;

  getData(): ContentModel | undefined;
  setData(value?: ContentModel): UpdateContentResponse;
  hasData(): boolean;
  clearData(): UpdateContentResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateContentResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateContentResponse): UpdateContentResponse.AsObject;
  static serializeBinaryToWriter(message: UpdateContentResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateContentResponse;
  static deserializeBinaryFromReader(message: UpdateContentResponse, reader: jspb.BinaryReader): UpdateContentResponse;
}

export namespace UpdateContentResponse {
  export type AsObject = {
    status: boolean,
    data?: ContentModel.AsObject,
  }
}

export class PutContentIdentifierRequest extends jspb.Message {
  getContentId(): string;
  setContentId(value: string): PutContentIdentifierRequest;

  getIdentifier(): string;
  setIdentifier(value: string): PutContentIdentifierRequest;

  getContentType(): string;
  setContentType(value: string): PutContentIdentifierRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PutContentIdentifierRequest.AsObject;
  static toObject(includeInstance: boolean, msg: PutContentIdentifierRequest): PutContentIdentifierRequest.AsObject;
  static serializeBinaryToWriter(message: PutContentIdentifierRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PutContentIdentifierRequest;
  static deserializeBinaryFromReader(message: PutContentIdentifierRequest, reader: jspb.BinaryReader): PutContentIdentifierRequest;
}

export namespace PutContentIdentifierRequest {
  export type AsObject = {
    contentId: string,
    identifier: string,
    contentType: string,
  }
}

export class PutContentIdentifierResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): PutContentIdentifierResponse;

  getData(): ContentModel | undefined;
  setData(value?: ContentModel): PutContentIdentifierResponse;
  hasData(): boolean;
  clearData(): PutContentIdentifierResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PutContentIdentifierResponse.AsObject;
  static toObject(includeInstance: boolean, msg: PutContentIdentifierResponse): PutContentIdentifierResponse.AsObject;
  static serializeBinaryToWriter(message: PutContentIdentifierResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PutContentIdentifierResponse;
  static deserializeBinaryFromReader(message: PutContentIdentifierResponse, reader: jspb.BinaryReader): PutContentIdentifierResponse;
}

export namespace PutContentIdentifierResponse {
  export type AsObject = {
    status: boolean,
    data?: ContentModel.AsObject,
  }
}


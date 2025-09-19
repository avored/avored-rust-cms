import * as jspb from 'google-protobuf'

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb'; // proto import: "google/protobuf/timestamp.proto"


export class FolderTypeMetaData extends jspb.Message {
  getColor(): string;
  setColor(value: string): FolderTypeMetaData;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): FolderTypeMetaData.AsObject;
  static toObject(includeInstance: boolean, msg: FolderTypeMetaData): FolderTypeMetaData.AsObject;
  static serializeBinaryToWriter(message: FolderTypeMetaData, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): FolderTypeMetaData;
  static deserializeBinaryFromReader(message: FolderTypeMetaData, reader: jspb.BinaryReader): FolderTypeMetaData;
}

export namespace FolderTypeMetaData {
  export type AsObject = {
    color: string,
  }
}

export class FileTypeMetaData extends jspb.Message {
  getFileType(): string;
  setFileType(value: string): FileTypeMetaData;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): FileTypeMetaData.AsObject;
  static toObject(includeInstance: boolean, msg: FileTypeMetaData): FileTypeMetaData.AsObject;
  static serializeBinaryToWriter(message: FileTypeMetaData, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): FileTypeMetaData;
  static deserializeBinaryFromReader(message: FileTypeMetaData, reader: jspb.BinaryReader): FileTypeMetaData;
}

export namespace FileTypeMetaData {
  export type AsObject = {
    fileType: string,
  }
}

export class MetaDataType extends jspb.Message {
  getFileMetaData(): FileTypeMetaData | undefined;
  setFileMetaData(value?: FileTypeMetaData): MetaDataType;
  hasFileMetaData(): boolean;
  clearFileMetaData(): MetaDataType;

  getFolderMetaData(): FolderTypeMetaData | undefined;
  setFolderMetaData(value?: FolderTypeMetaData): MetaDataType;
  hasFolderMetaData(): boolean;
  clearFolderMetaData(): MetaDataType;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MetaDataType.AsObject;
  static toObject(includeInstance: boolean, msg: MetaDataType): MetaDataType.AsObject;
  static serializeBinaryToWriter(message: MetaDataType, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MetaDataType;
  static deserializeBinaryFromReader(message: MetaDataType, reader: jspb.BinaryReader): MetaDataType;
}

export namespace MetaDataType {
  export type AsObject = {
    fileMetaData?: FileTypeMetaData.AsObject,
    folderMetaData?: FolderTypeMetaData.AsObject,
  }

  export enum FileMetaDataCase { 
    _FILE_META_DATA_NOT_SET = 0,
    FILE_META_DATA = 1,
  }

  export enum FolderMetaDataCase { 
    _FOLDER_META_DATA_NOT_SET = 0,
    FOLDER_META_DATA = 2,
  }
}

export class AssetModel extends jspb.Message {
  getId(): string;
  setId(value: string): AssetModel;

  getParentId(): string;
  setParentId(value: string): AssetModel;
  hasParentId(): boolean;
  clearParentId(): AssetModel;

  getName(): string;
  setName(value: string): AssetModel;

  getNewPath(): string;
  setNewPath(value: string): AssetModel;

  getAssetType(): string;
  setAssetType(value: string): AssetModel;

  getMetadata(): MetaDataType | undefined;
  setMetadata(value?: MetaDataType): AssetModel;
  hasMetadata(): boolean;
  clearMetadata(): AssetModel;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): AssetModel;
  hasCreatedAt(): boolean;
  clearCreatedAt(): AssetModel;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): AssetModel;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): AssetModel;

  getCreatedBy(): string;
  setCreatedBy(value: string): AssetModel;

  getUpdatedBy(): string;
  setUpdatedBy(value: string): AssetModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AssetModel.AsObject;
  static toObject(includeInstance: boolean, msg: AssetModel): AssetModel.AsObject;
  static serializeBinaryToWriter(message: AssetModel, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AssetModel;
  static deserializeBinaryFromReader(message: AssetModel, reader: jspb.BinaryReader): AssetModel;
}

export namespace AssetModel {
  export type AsObject = {
    id: string,
    parentId?: string,
    name: string,
    newPath: string,
    assetType: string,
    metadata?: MetaDataType.AsObject,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    createdBy: string,
    updatedBy: string,
  }

  export enum ParentIdCase { 
    _PARENT_ID_NOT_SET = 0,
    PARENT_ID = 2,
  }
}

export class AssetPaginateRequest extends jspb.Message {
  getPage(): number;
  setPage(value: number): AssetPaginateRequest;
  hasPage(): boolean;
  clearPage(): AssetPaginateRequest;

  getOrder(): string;
  setOrder(value: string): AssetPaginateRequest;
  hasOrder(): boolean;
  clearOrder(): AssetPaginateRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AssetPaginateRequest.AsObject;
  static toObject(includeInstance: boolean, msg: AssetPaginateRequest): AssetPaginateRequest.AsObject;
  static serializeBinaryToWriter(message: AssetPaginateRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AssetPaginateRequest;
  static deserializeBinaryFromReader(message: AssetPaginateRequest, reader: jspb.BinaryReader): AssetPaginateRequest;
}

export namespace AssetPaginateRequest {
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

export class AssetPaginateResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): AssetPaginateResponse;

  getData(): AssetPaginateResponse.AssetPaginateData | undefined;
  setData(value?: AssetPaginateResponse.AssetPaginateData): AssetPaginateResponse;
  hasData(): boolean;
  clearData(): AssetPaginateResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AssetPaginateResponse.AsObject;
  static toObject(includeInstance: boolean, msg: AssetPaginateResponse): AssetPaginateResponse.AsObject;
  static serializeBinaryToWriter(message: AssetPaginateResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AssetPaginateResponse;
  static deserializeBinaryFromReader(message: AssetPaginateResponse, reader: jspb.BinaryReader): AssetPaginateResponse;
}

export namespace AssetPaginateResponse {
  export type AsObject = {
    status: boolean,
    data?: AssetPaginateResponse.AssetPaginateData.AsObject,
  }

  export class AssetPagination extends jspb.Message {
    getTotal(): number;
    setTotal(value: number): AssetPagination;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): AssetPagination.AsObject;
    static toObject(includeInstance: boolean, msg: AssetPagination): AssetPagination.AsObject;
    static serializeBinaryToWriter(message: AssetPagination, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): AssetPagination;
    static deserializeBinaryFromReader(message: AssetPagination, reader: jspb.BinaryReader): AssetPagination;
  }

  export namespace AssetPagination {
    export type AsObject = {
      total: number,
    }
  }


  export class AssetPaginateData extends jspb.Message {
    getPagination(): AssetPaginateResponse.AssetPagination | undefined;
    setPagination(value?: AssetPaginateResponse.AssetPagination): AssetPaginateData;
    hasPagination(): boolean;
    clearPagination(): AssetPaginateData;

    getDataList(): Array<AssetModel>;
    setDataList(value: Array<AssetModel>): AssetPaginateData;
    clearDataList(): AssetPaginateData;
    addData(value?: AssetModel, index?: number): AssetModel;

    serializeBinary(): Uint8Array;
    toObject(includeInstance?: boolean): AssetPaginateData.AsObject;
    static toObject(includeInstance: boolean, msg: AssetPaginateData): AssetPaginateData.AsObject;
    static serializeBinaryToWriter(message: AssetPaginateData, writer: jspb.BinaryWriter): void;
    static deserializeBinary(bytes: Uint8Array): AssetPaginateData;
    static deserializeBinaryFromReader(message: AssetPaginateData, reader: jspb.BinaryReader): AssetPaginateData;
  }

  export namespace AssetPaginateData {
    export type AsObject = {
      pagination?: AssetPaginateResponse.AssetPagination.AsObject,
      dataList: Array<AssetModel.AsObject>,
    }
  }

}

export class CreateFolderRequest extends jspb.Message {
  getName(): string;
  setName(value: string): CreateFolderRequest;

  getParentId(): string;
  setParentId(value: string): CreateFolderRequest;
  hasParentId(): boolean;
  clearParentId(): CreateFolderRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateFolderRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateFolderRequest): CreateFolderRequest.AsObject;
  static serializeBinaryToWriter(message: CreateFolderRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateFolderRequest;
  static deserializeBinaryFromReader(message: CreateFolderRequest, reader: jspb.BinaryReader): CreateFolderRequest;
}

export namespace CreateFolderRequest {
  export type AsObject = {
    name: string,
    parentId?: string,
  }

  export enum ParentIdCase { 
    _PARENT_ID_NOT_SET = 0,
    PARENT_ID = 2,
  }
}

export class CreateFolderResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): CreateFolderResponse;

  getData(): AssetModel | undefined;
  setData(value?: AssetModel): CreateFolderResponse;
  hasData(): boolean;
  clearData(): CreateFolderResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateFolderResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CreateFolderResponse): CreateFolderResponse.AsObject;
  static serializeBinaryToWriter(message: CreateFolderResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateFolderResponse;
  static deserializeBinaryFromReader(message: CreateFolderResponse, reader: jspb.BinaryReader): CreateFolderResponse;
}

export namespace CreateFolderResponse {
  export type AsObject = {
    status: boolean,
    data?: AssetModel.AsObject,
  }
}

export class DeleteAssetRequest extends jspb.Message {
  getAssetId(): string;
  setAssetId(value: string): DeleteAssetRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteAssetRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteAssetRequest): DeleteAssetRequest.AsObject;
  static serializeBinaryToWriter(message: DeleteAssetRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteAssetRequest;
  static deserializeBinaryFromReader(message: DeleteAssetRequest, reader: jspb.BinaryReader): DeleteAssetRequest;
}

export namespace DeleteAssetRequest {
  export type AsObject = {
    assetId: string,
  }
}

export class DeleteAssetResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): DeleteAssetResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteAssetResponse.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteAssetResponse): DeleteAssetResponse.AsObject;
  static serializeBinaryToWriter(message: DeleteAssetResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteAssetResponse;
  static deserializeBinaryFromReader(message: DeleteAssetResponse, reader: jspb.BinaryReader): DeleteAssetResponse;
}

export namespace DeleteAssetResponse {
  export type AsObject = {
    status: boolean,
  }
}

export class DeleteFolderRequest extends jspb.Message {
  getFolderId(): string;
  setFolderId(value: string): DeleteFolderRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteFolderRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteFolderRequest): DeleteFolderRequest.AsObject;
  static serializeBinaryToWriter(message: DeleteFolderRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteFolderRequest;
  static deserializeBinaryFromReader(message: DeleteFolderRequest, reader: jspb.BinaryReader): DeleteFolderRequest;
}

export namespace DeleteFolderRequest {
  export type AsObject = {
    folderId: string,
  }
}

export class DeleteFolderResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): DeleteFolderResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteFolderResponse.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteFolderResponse): DeleteFolderResponse.AsObject;
  static serializeBinaryToWriter(message: DeleteFolderResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteFolderResponse;
  static deserializeBinaryFromReader(message: DeleteFolderResponse, reader: jspb.BinaryReader): DeleteFolderResponse;
}

export namespace DeleteFolderResponse {
  export type AsObject = {
    status: boolean,
  }
}

export class RenameAssetRequest extends jspb.Message {
  getAssetId(): string;
  setAssetId(value: string): RenameAssetRequest;

  getName(): string;
  setName(value: string): RenameAssetRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RenameAssetRequest.AsObject;
  static toObject(includeInstance: boolean, msg: RenameAssetRequest): RenameAssetRequest.AsObject;
  static serializeBinaryToWriter(message: RenameAssetRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RenameAssetRequest;
  static deserializeBinaryFromReader(message: RenameAssetRequest, reader: jspb.BinaryReader): RenameAssetRequest;
}

export namespace RenameAssetRequest {
  export type AsObject = {
    assetId: string,
    name: string,
  }
}

export class RenameAssetResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): RenameAssetResponse;

  getData(): AssetModel | undefined;
  setData(value?: AssetModel): RenameAssetResponse;
  hasData(): boolean;
  clearData(): RenameAssetResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): RenameAssetResponse.AsObject;
  static toObject(includeInstance: boolean, msg: RenameAssetResponse): RenameAssetResponse.AsObject;
  static serializeBinaryToWriter(message: RenameAssetResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): RenameAssetResponse;
  static deserializeBinaryFromReader(message: RenameAssetResponse, reader: jspb.BinaryReader): RenameAssetResponse;
}

export namespace RenameAssetResponse {
  export type AsObject = {
    status: boolean,
    data?: AssetModel.AsObject,
  }
}


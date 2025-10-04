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

export class VisitByYear extends jspb.Message {
  getVisits(): number;
  setVisits(value: number): VisitByYear;

  getMonth(): string;
  setMonth(value: string): VisitByYear;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): VisitByYear.AsObject;
  static toObject(includeInstance: boolean, msg: VisitByYear): VisitByYear.AsObject;
  static serializeBinaryToWriter(message: VisitByYear, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): VisitByYear;
  static deserializeBinaryFromReader(message: VisitByYear, reader: jspb.BinaryReader): VisitByYear;
}

export namespace VisitByYear {
  export type AsObject = {
    visits: number,
    month: string,
  }
}

export class VisitByContentType extends jspb.Message {
  getVisits(): number;
  setVisits(value: number): VisitByContentType;

  getMonth(): string;
  setMonth(value: string): VisitByContentType;

  getContentType(): string;
  setContentType(value: string): VisitByContentType;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): VisitByContentType.AsObject;
  static toObject(includeInstance: boolean, msg: VisitByContentType): VisitByContentType.AsObject;
  static serializeBinaryToWriter(message: VisitByContentType, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): VisitByContentType;
  static deserializeBinaryFromReader(message: VisitByContentType, reader: jspb.BinaryReader): VisitByContentType;
}

export namespace VisitByContentType {
  export type AsObject = {
    visits: number,
    month: string,
    contentType: string,
  }
}

export class VisitByYearRequest extends jspb.Message {
  getYear(): number;
  setYear(value: number): VisitByYearRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): VisitByYearRequest.AsObject;
  static toObject(includeInstance: boolean, msg: VisitByYearRequest): VisitByYearRequest.AsObject;
  static serializeBinaryToWriter(message: VisitByYearRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): VisitByYearRequest;
  static deserializeBinaryFromReader(message: VisitByYearRequest, reader: jspb.BinaryReader): VisitByYearRequest;
}

export namespace VisitByYearRequest {
  export type AsObject = {
    year: number,
  }
}

export class VisitByYearResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): VisitByYearResponse;

  getDataList(): Array<VisitByYear>;
  setDataList(value: Array<VisitByYear>): VisitByYearResponse;
  clearDataList(): VisitByYearResponse;
  addData(value?: VisitByYear, index?: number): VisitByYear;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): VisitByYearResponse.AsObject;
  static toObject(includeInstance: boolean, msg: VisitByYearResponse): VisitByYearResponse.AsObject;
  static serializeBinaryToWriter(message: VisitByYearResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): VisitByYearResponse;
  static deserializeBinaryFromReader(message: VisitByYearResponse, reader: jspb.BinaryReader): VisitByYearResponse;
}

export namespace VisitByYearResponse {
  export type AsObject = {
    status: boolean,
    dataList: Array<VisitByYear.AsObject>,
  }
}

export class VisitByContentTypeRequest extends jspb.Message {
  getContentType(): string;
  setContentType(value: string): VisitByContentTypeRequest;

  getYear(): number;
  setYear(value: number): VisitByContentTypeRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): VisitByContentTypeRequest.AsObject;
  static toObject(includeInstance: boolean, msg: VisitByContentTypeRequest): VisitByContentTypeRequest.AsObject;
  static serializeBinaryToWriter(message: VisitByContentTypeRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): VisitByContentTypeRequest;
  static deserializeBinaryFromReader(message: VisitByContentTypeRequest, reader: jspb.BinaryReader): VisitByContentTypeRequest;
}

export namespace VisitByContentTypeRequest {
  export type AsObject = {
    contentType: string,
    year: number,
  }
}

export class VisitByContentTypeResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): VisitByContentTypeResponse;

  getDataList(): Array<VisitByContentType>;
  setDataList(value: Array<VisitByContentType>): VisitByContentTypeResponse;
  clearDataList(): VisitByContentTypeResponse;
  addData(value?: VisitByContentType, index?: number): VisitByContentType;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): VisitByContentTypeResponse.AsObject;
  static toObject(includeInstance: boolean, msg: VisitByContentTypeResponse): VisitByContentTypeResponse.AsObject;
  static serializeBinaryToWriter(message: VisitByContentTypeResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): VisitByContentTypeResponse;
  static deserializeBinaryFromReader(message: VisitByContentTypeResponse, reader: jspb.BinaryReader): VisitByContentTypeResponse;
}

export namespace VisitByContentTypeResponse {
  export type AsObject = {
    status: boolean,
    dataList: Array<VisitByContentType.AsObject>,
  }
}


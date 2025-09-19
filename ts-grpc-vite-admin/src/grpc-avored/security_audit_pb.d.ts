import * as jspb from 'google-protobuf'

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb'; // proto import: "google/protobuf/timestamp.proto"


export class SecurityAuditModel extends jspb.Message {
  getId(): string;
  setId(value: string): SecurityAuditModel;

  getSecurityAuditId(): string;
  setSecurityAuditId(value: string): SecurityAuditModel;

  getAdminUserId(): string;
  setAdminUserId(value: string): SecurityAuditModel;
  hasAdminUserId(): boolean;
  clearAdminUserId(): SecurityAuditModel;

  getSessionId(): string;
  setSessionId(value: string): SecurityAuditModel;
  hasSessionId(): boolean;
  clearSessionId(): SecurityAuditModel;

  getIpAddress(): string;
  setIpAddress(value: string): SecurityAuditModel;

  getUserAgent(): string;
  setUserAgent(value: string): SecurityAuditModel;
  hasUserAgent(): boolean;
  clearUserAgent(): SecurityAuditModel;

  getEndpoint(): string;
  setEndpoint(value: string): SecurityAuditModel;
  hasEndpoint(): boolean;
  clearEndpoint(): SecurityAuditModel;

  getRequestMethod(): string;
  setRequestMethod(value: string): SecurityAuditModel;
  hasRequestMethod(): boolean;
  clearRequestMethod(): SecurityAuditModel;

  getTotalAuthenticationAttempts(): number;
  setTotalAuthenticationAttempts(value: number): SecurityAuditModel;

  getFailedAuthenticationAttempts(): number;
  setFailedAuthenticationAttempts(value: number): SecurityAuditModel;

  getBlockedInjectionAttempts(): number;
  setBlockedInjectionAttempts(value: number): SecurityAuditModel;

  getRateLimitedRequests(): number;
  setRateLimitedRequests(value: number): SecurityAuditModel;

  getSuspiciousActivitiesDetected(): number;
  setSuspiciousActivitiesDetected(value: number): SecurityAuditModel;

  getSecurityViolations(): number;
  setSecurityViolations(value: number): SecurityAuditModel;

  getUptimeSeconds(): number;
  setUptimeSeconds(value: number): SecurityAuditModel;

  getSecurityHealthScore(): number;
  setSecurityHealthScore(value: number): SecurityAuditModel;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): SecurityAuditModel;
  hasCreatedAt(): boolean;
  clearCreatedAt(): SecurityAuditModel;

  getUpdatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setUpdatedAt(value?: google_protobuf_timestamp_pb.Timestamp): SecurityAuditModel;
  hasUpdatedAt(): boolean;
  clearUpdatedAt(): SecurityAuditModel;

  getMetadataJson(): string;
  setMetadataJson(value: string): SecurityAuditModel;
  hasMetadataJson(): boolean;
  clearMetadataJson(): SecurityAuditModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SecurityAuditModel.AsObject;
  static toObject(includeInstance: boolean, msg: SecurityAuditModel): SecurityAuditModel.AsObject;
  static serializeBinaryToWriter(message: SecurityAuditModel, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SecurityAuditModel;
  static deserializeBinaryFromReader(message: SecurityAuditModel, reader: jspb.BinaryReader): SecurityAuditModel;
}

export namespace SecurityAuditModel {
  export type AsObject = {
    id: string,
    securityAuditId: string,
    adminUserId?: string,
    sessionId?: string,
    ipAddress: string,
    userAgent?: string,
    endpoint?: string,
    requestMethod?: string,
    totalAuthenticationAttempts: number,
    failedAuthenticationAttempts: number,
    blockedInjectionAttempts: number,
    rateLimitedRequests: number,
    suspiciousActivitiesDetected: number,
    securityViolations: number,
    uptimeSeconds: number,
    securityHealthScore: number,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    updatedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    metadataJson?: string,
  }

  export enum AdminUserIdCase { 
    _ADMIN_USER_ID_NOT_SET = 0,
    ADMIN_USER_ID = 3,
  }

  export enum SessionIdCase { 
    _SESSION_ID_NOT_SET = 0,
    SESSION_ID = 4,
  }

  export enum UserAgentCase { 
    _USER_AGENT_NOT_SET = 0,
    USER_AGENT = 6,
  }

  export enum EndpointCase { 
    _ENDPOINT_NOT_SET = 0,
    ENDPOINT = 7,
  }

  export enum RequestMethodCase { 
    _REQUEST_METHOD_NOT_SET = 0,
    REQUEST_METHOD = 8,
  }

  export enum MetadataJsonCase { 
    _METADATA_JSON_NOT_SET = 0,
    METADATA_JSON = 19,
  }
}

export class CreateSecurityAuditModel extends jspb.Message {
  getSecurityAuditId(): string;
  setSecurityAuditId(value: string): CreateSecurityAuditModel;

  getAdminUserId(): string;
  setAdminUserId(value: string): CreateSecurityAuditModel;
  hasAdminUserId(): boolean;
  clearAdminUserId(): CreateSecurityAuditModel;

  getSessionId(): string;
  setSessionId(value: string): CreateSecurityAuditModel;
  hasSessionId(): boolean;
  clearSessionId(): CreateSecurityAuditModel;

  getIpAddress(): string;
  setIpAddress(value: string): CreateSecurityAuditModel;

  getUserAgent(): string;
  setUserAgent(value: string): CreateSecurityAuditModel;
  hasUserAgent(): boolean;
  clearUserAgent(): CreateSecurityAuditModel;

  getEndpoint(): string;
  setEndpoint(value: string): CreateSecurityAuditModel;
  hasEndpoint(): boolean;
  clearEndpoint(): CreateSecurityAuditModel;

  getRequestMethod(): string;
  setRequestMethod(value: string): CreateSecurityAuditModel;
  hasRequestMethod(): boolean;
  clearRequestMethod(): CreateSecurityAuditModel;

  getTotalAuthenticationAttempts(): number;
  setTotalAuthenticationAttempts(value: number): CreateSecurityAuditModel;
  hasTotalAuthenticationAttempts(): boolean;
  clearTotalAuthenticationAttempts(): CreateSecurityAuditModel;

  getFailedAuthenticationAttempts(): number;
  setFailedAuthenticationAttempts(value: number): CreateSecurityAuditModel;
  hasFailedAuthenticationAttempts(): boolean;
  clearFailedAuthenticationAttempts(): CreateSecurityAuditModel;

  getBlockedInjectionAttempts(): number;
  setBlockedInjectionAttempts(value: number): CreateSecurityAuditModel;
  hasBlockedInjectionAttempts(): boolean;
  clearBlockedInjectionAttempts(): CreateSecurityAuditModel;

  getRateLimitedRequests(): number;
  setRateLimitedRequests(value: number): CreateSecurityAuditModel;
  hasRateLimitedRequests(): boolean;
  clearRateLimitedRequests(): CreateSecurityAuditModel;

  getSuspiciousActivitiesDetected(): number;
  setSuspiciousActivitiesDetected(value: number): CreateSecurityAuditModel;
  hasSuspiciousActivitiesDetected(): boolean;
  clearSuspiciousActivitiesDetected(): CreateSecurityAuditModel;

  getSecurityViolations(): number;
  setSecurityViolations(value: number): CreateSecurityAuditModel;
  hasSecurityViolations(): boolean;
  clearSecurityViolations(): CreateSecurityAuditModel;

  getUptimeSeconds(): number;
  setUptimeSeconds(value: number): CreateSecurityAuditModel;
  hasUptimeSeconds(): boolean;
  clearUptimeSeconds(): CreateSecurityAuditModel;

  getSecurityHealthScore(): number;
  setSecurityHealthScore(value: number): CreateSecurityAuditModel;
  hasSecurityHealthScore(): boolean;
  clearSecurityHealthScore(): CreateSecurityAuditModel;

  getMetadataJson(): string;
  setMetadataJson(value: string): CreateSecurityAuditModel;
  hasMetadataJson(): boolean;
  clearMetadataJson(): CreateSecurityAuditModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateSecurityAuditModel.AsObject;
  static toObject(includeInstance: boolean, msg: CreateSecurityAuditModel): CreateSecurityAuditModel.AsObject;
  static serializeBinaryToWriter(message: CreateSecurityAuditModel, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateSecurityAuditModel;
  static deserializeBinaryFromReader(message: CreateSecurityAuditModel, reader: jspb.BinaryReader): CreateSecurityAuditModel;
}

export namespace CreateSecurityAuditModel {
  export type AsObject = {
    securityAuditId: string,
    adminUserId?: string,
    sessionId?: string,
    ipAddress: string,
    userAgent?: string,
    endpoint?: string,
    requestMethod?: string,
    totalAuthenticationAttempts?: number,
    failedAuthenticationAttempts?: number,
    blockedInjectionAttempts?: number,
    rateLimitedRequests?: number,
    suspiciousActivitiesDetected?: number,
    securityViolations?: number,
    uptimeSeconds?: number,
    securityHealthScore?: number,
    metadataJson?: string,
  }

  export enum AdminUserIdCase { 
    _ADMIN_USER_ID_NOT_SET = 0,
    ADMIN_USER_ID = 2,
  }

  export enum SessionIdCase { 
    _SESSION_ID_NOT_SET = 0,
    SESSION_ID = 3,
  }

  export enum UserAgentCase { 
    _USER_AGENT_NOT_SET = 0,
    USER_AGENT = 5,
  }

  export enum EndpointCase { 
    _ENDPOINT_NOT_SET = 0,
    ENDPOINT = 6,
  }

  export enum RequestMethodCase { 
    _REQUEST_METHOD_NOT_SET = 0,
    REQUEST_METHOD = 7,
  }

  export enum TotalAuthenticationAttemptsCase { 
    _TOTAL_AUTHENTICATION_ATTEMPTS_NOT_SET = 0,
    TOTAL_AUTHENTICATION_ATTEMPTS = 8,
  }

  export enum FailedAuthenticationAttemptsCase { 
    _FAILED_AUTHENTICATION_ATTEMPTS_NOT_SET = 0,
    FAILED_AUTHENTICATION_ATTEMPTS = 9,
  }

  export enum BlockedInjectionAttemptsCase { 
    _BLOCKED_INJECTION_ATTEMPTS_NOT_SET = 0,
    BLOCKED_INJECTION_ATTEMPTS = 10,
  }

  export enum RateLimitedRequestsCase { 
    _RATE_LIMITED_REQUESTS_NOT_SET = 0,
    RATE_LIMITED_REQUESTS = 11,
  }

  export enum SuspiciousActivitiesDetectedCase { 
    _SUSPICIOUS_ACTIVITIES_DETECTED_NOT_SET = 0,
    SUSPICIOUS_ACTIVITIES_DETECTED = 12,
  }

  export enum SecurityViolationsCase { 
    _SECURITY_VIOLATIONS_NOT_SET = 0,
    SECURITY_VIOLATIONS = 13,
  }

  export enum UptimeSecondsCase { 
    _UPTIME_SECONDS_NOT_SET = 0,
    UPTIME_SECONDS = 14,
  }

  export enum SecurityHealthScoreCase { 
    _SECURITY_HEALTH_SCORE_NOT_SET = 0,
    SECURITY_HEALTH_SCORE = 15,
  }

  export enum MetadataJsonCase { 
    _METADATA_JSON_NOT_SET = 0,
    METADATA_JSON = 16,
  }
}

export class UpdateSecurityAuditModel extends jspb.Message {
  getTotalAuthenticationAttempts(): number;
  setTotalAuthenticationAttempts(value: number): UpdateSecurityAuditModel;
  hasTotalAuthenticationAttempts(): boolean;
  clearTotalAuthenticationAttempts(): UpdateSecurityAuditModel;

  getFailedAuthenticationAttempts(): number;
  setFailedAuthenticationAttempts(value: number): UpdateSecurityAuditModel;
  hasFailedAuthenticationAttempts(): boolean;
  clearFailedAuthenticationAttempts(): UpdateSecurityAuditModel;

  getBlockedInjectionAttempts(): number;
  setBlockedInjectionAttempts(value: number): UpdateSecurityAuditModel;
  hasBlockedInjectionAttempts(): boolean;
  clearBlockedInjectionAttempts(): UpdateSecurityAuditModel;

  getRateLimitedRequests(): number;
  setRateLimitedRequests(value: number): UpdateSecurityAuditModel;
  hasRateLimitedRequests(): boolean;
  clearRateLimitedRequests(): UpdateSecurityAuditModel;

  getSuspiciousActivitiesDetected(): number;
  setSuspiciousActivitiesDetected(value: number): UpdateSecurityAuditModel;
  hasSuspiciousActivitiesDetected(): boolean;
  clearSuspiciousActivitiesDetected(): UpdateSecurityAuditModel;

  getSecurityViolations(): number;
  setSecurityViolations(value: number): UpdateSecurityAuditModel;
  hasSecurityViolations(): boolean;
  clearSecurityViolations(): UpdateSecurityAuditModel;

  getUptimeSeconds(): number;
  setUptimeSeconds(value: number): UpdateSecurityAuditModel;
  hasUptimeSeconds(): boolean;
  clearUptimeSeconds(): UpdateSecurityAuditModel;

  getSecurityHealthScore(): number;
  setSecurityHealthScore(value: number): UpdateSecurityAuditModel;
  hasSecurityHealthScore(): boolean;
  clearSecurityHealthScore(): UpdateSecurityAuditModel;

  getMetadataJson(): string;
  setMetadataJson(value: string): UpdateSecurityAuditModel;
  hasMetadataJson(): boolean;
  clearMetadataJson(): UpdateSecurityAuditModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateSecurityAuditModel.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateSecurityAuditModel): UpdateSecurityAuditModel.AsObject;
  static serializeBinaryToWriter(message: UpdateSecurityAuditModel, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateSecurityAuditModel;
  static deserializeBinaryFromReader(message: UpdateSecurityAuditModel, reader: jspb.BinaryReader): UpdateSecurityAuditModel;
}

export namespace UpdateSecurityAuditModel {
  export type AsObject = {
    totalAuthenticationAttempts?: number,
    failedAuthenticationAttempts?: number,
    blockedInjectionAttempts?: number,
    rateLimitedRequests?: number,
    suspiciousActivitiesDetected?: number,
    securityViolations?: number,
    uptimeSeconds?: number,
    securityHealthScore?: number,
    metadataJson?: string,
  }

  export enum TotalAuthenticationAttemptsCase { 
    _TOTAL_AUTHENTICATION_ATTEMPTS_NOT_SET = 0,
    TOTAL_AUTHENTICATION_ATTEMPTS = 1,
  }

  export enum FailedAuthenticationAttemptsCase { 
    _FAILED_AUTHENTICATION_ATTEMPTS_NOT_SET = 0,
    FAILED_AUTHENTICATION_ATTEMPTS = 2,
  }

  export enum BlockedInjectionAttemptsCase { 
    _BLOCKED_INJECTION_ATTEMPTS_NOT_SET = 0,
    BLOCKED_INJECTION_ATTEMPTS = 3,
  }

  export enum RateLimitedRequestsCase { 
    _RATE_LIMITED_REQUESTS_NOT_SET = 0,
    RATE_LIMITED_REQUESTS = 4,
  }

  export enum SuspiciousActivitiesDetectedCase { 
    _SUSPICIOUS_ACTIVITIES_DETECTED_NOT_SET = 0,
    SUSPICIOUS_ACTIVITIES_DETECTED = 5,
  }

  export enum SecurityViolationsCase { 
    _SECURITY_VIOLATIONS_NOT_SET = 0,
    SECURITY_VIOLATIONS = 6,
  }

  export enum UptimeSecondsCase { 
    _UPTIME_SECONDS_NOT_SET = 0,
    UPTIME_SECONDS = 7,
  }

  export enum SecurityHealthScoreCase { 
    _SECURITY_HEALTH_SCORE_NOT_SET = 0,
    SECURITY_HEALTH_SCORE = 8,
  }

  export enum MetadataJsonCase { 
    _METADATA_JSON_NOT_SET = 0,
    METADATA_JSON = 9,
  }
}

export class SecurityAlertModel extends jspb.Message {
  getId(): string;
  setId(value: string): SecurityAlertModel;

  getAlertId(): string;
  setAlertId(value: string): SecurityAlertModel;

  getAlertType(): AlertType;
  setAlertType(value: AlertType): SecurityAlertModel;

  getSeverity(): AlertSeverity;
  setSeverity(value: AlertSeverity): SecurityAlertModel;

  getMessage(): string;
  setMessage(value: string): SecurityAlertModel;

  getSource(): string;
  setSource(value: string): SecurityAlertModel;

  getAffectedResource(): string;
  setAffectedResource(value: string): SecurityAlertModel;
  hasAffectedResource(): boolean;
  clearAffectedResource(): SecurityAlertModel;

  getMetadataJson(): string;
  setMetadataJson(value: string): SecurityAlertModel;
  hasMetadataJson(): boolean;
  clearMetadataJson(): SecurityAlertModel;

  getResolved(): boolean;
  setResolved(value: boolean): SecurityAlertModel;

  getResolvedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setResolvedAt(value?: google_protobuf_timestamp_pb.Timestamp): SecurityAlertModel;
  hasResolvedAt(): boolean;
  clearResolvedAt(): SecurityAlertModel;

  getResolvedBy(): string;
  setResolvedBy(value: string): SecurityAlertModel;
  hasResolvedBy(): boolean;
  clearResolvedBy(): SecurityAlertModel;

  getCreatedAt(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setCreatedAt(value?: google_protobuf_timestamp_pb.Timestamp): SecurityAlertModel;
  hasCreatedAt(): boolean;
  clearCreatedAt(): SecurityAlertModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SecurityAlertModel.AsObject;
  static toObject(includeInstance: boolean, msg: SecurityAlertModel): SecurityAlertModel.AsObject;
  static serializeBinaryToWriter(message: SecurityAlertModel, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SecurityAlertModel;
  static deserializeBinaryFromReader(message: SecurityAlertModel, reader: jspb.BinaryReader): SecurityAlertModel;
}

export namespace SecurityAlertModel {
  export type AsObject = {
    id: string,
    alertId: string,
    alertType: AlertType,
    severity: AlertSeverity,
    message: string,
    source: string,
    affectedResource?: string,
    metadataJson?: string,
    resolved: boolean,
    resolvedAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    resolvedBy?: string,
    createdAt?: google_protobuf_timestamp_pb.Timestamp.AsObject,
  }

  export enum AffectedResourceCase { 
    _AFFECTED_RESOURCE_NOT_SET = 0,
    AFFECTED_RESOURCE = 7,
  }

  export enum MetadataJsonCase { 
    _METADATA_JSON_NOT_SET = 0,
    METADATA_JSON = 8,
  }

  export enum ResolvedAtCase { 
    _RESOLVED_AT_NOT_SET = 0,
    RESOLVED_AT = 10,
  }

  export enum ResolvedByCase { 
    _RESOLVED_BY_NOT_SET = 0,
    RESOLVED_BY = 11,
  }
}

export class CreateSecurityAlertModel extends jspb.Message {
  getAlertId(): string;
  setAlertId(value: string): CreateSecurityAlertModel;

  getAlertType(): AlertType;
  setAlertType(value: AlertType): CreateSecurityAlertModel;

  getSeverity(): AlertSeverity;
  setSeverity(value: AlertSeverity): CreateSecurityAlertModel;

  getMessage(): string;
  setMessage(value: string): CreateSecurityAlertModel;

  getSource(): string;
  setSource(value: string): CreateSecurityAlertModel;

  getAffectedResource(): string;
  setAffectedResource(value: string): CreateSecurityAlertModel;
  hasAffectedResource(): boolean;
  clearAffectedResource(): CreateSecurityAlertModel;

  getMetadataJson(): string;
  setMetadataJson(value: string): CreateSecurityAlertModel;
  hasMetadataJson(): boolean;
  clearMetadataJson(): CreateSecurityAlertModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateSecurityAlertModel.AsObject;
  static toObject(includeInstance: boolean, msg: CreateSecurityAlertModel): CreateSecurityAlertModel.AsObject;
  static serializeBinaryToWriter(message: CreateSecurityAlertModel, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateSecurityAlertModel;
  static deserializeBinaryFromReader(message: CreateSecurityAlertModel, reader: jspb.BinaryReader): CreateSecurityAlertModel;
}

export namespace CreateSecurityAlertModel {
  export type AsObject = {
    alertId: string,
    alertType: AlertType,
    severity: AlertSeverity,
    message: string,
    source: string,
    affectedResource?: string,
    metadataJson?: string,
  }

  export enum AffectedResourceCase { 
    _AFFECTED_RESOURCE_NOT_SET = 0,
    AFFECTED_RESOURCE = 6,
  }

  export enum MetadataJsonCase { 
    _METADATA_JSON_NOT_SET = 0,
    METADATA_JSON = 7,
  }
}

export class UpdateSecurityAlertModel extends jspb.Message {
  getResolved(): boolean;
  setResolved(value: boolean): UpdateSecurityAlertModel;
  hasResolved(): boolean;
  clearResolved(): UpdateSecurityAlertModel;

  getResolvedBy(): string;
  setResolvedBy(value: string): UpdateSecurityAlertModel;
  hasResolvedBy(): boolean;
  clearResolvedBy(): UpdateSecurityAlertModel;

  getMetadataJson(): string;
  setMetadataJson(value: string): UpdateSecurityAlertModel;
  hasMetadataJson(): boolean;
  clearMetadataJson(): UpdateSecurityAlertModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateSecurityAlertModel.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateSecurityAlertModel): UpdateSecurityAlertModel.AsObject;
  static serializeBinaryToWriter(message: UpdateSecurityAlertModel, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateSecurityAlertModel;
  static deserializeBinaryFromReader(message: UpdateSecurityAlertModel, reader: jspb.BinaryReader): UpdateSecurityAlertModel;
}

export namespace UpdateSecurityAlertModel {
  export type AsObject = {
    resolved?: boolean,
    resolvedBy?: string,
    metadataJson?: string,
  }

  export enum ResolvedCase { 
    _RESOLVED_NOT_SET = 0,
    RESOLVED = 1,
  }

  export enum ResolvedByCase { 
    _RESOLVED_BY_NOT_SET = 0,
    RESOLVED_BY = 2,
  }

  export enum MetadataJsonCase { 
    _METADATA_JSON_NOT_SET = 0,
    METADATA_JSON = 3,
  }
}

export class Pagination extends jspb.Message {
  getTotal(): number;
  setTotal(value: number): Pagination;

  getPerPage(): number;
  setPerPage(value: number): Pagination;

  getCurrentPage(): number;
  setCurrentPage(value: number): Pagination;

  getFrom(): number;
  setFrom(value: number): Pagination;

  getTo(): number;
  setTo(value: number): Pagination;

  getHasNextPage(): boolean;
  setHasNextPage(value: boolean): Pagination;

  getHasPreviousPage(): boolean;
  setHasPreviousPage(value: boolean): Pagination;

  getNextPageNumber(): number;
  setNextPageNumber(value: number): Pagination;

  getPreviousPageNumber(): number;
  setPreviousPageNumber(value: number): Pagination;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Pagination.AsObject;
  static toObject(includeInstance: boolean, msg: Pagination): Pagination.AsObject;
  static serializeBinaryToWriter(message: Pagination, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Pagination;
  static deserializeBinaryFromReader(message: Pagination, reader: jspb.BinaryReader): Pagination;
}

export namespace Pagination {
  export type AsObject = {
    total: number,
    perPage: number,
    currentPage: number,
    from: number,
    to: number,
    hasNextPage: boolean,
    hasPreviousPage: boolean,
    nextPageNumber: number,
    previousPageNumber: number,
  }
}

export class SecurityAuditPaginationModel extends jspb.Message {
  getDataList(): Array<SecurityAuditModel>;
  setDataList(value: Array<SecurityAuditModel>): SecurityAuditPaginationModel;
  clearDataList(): SecurityAuditPaginationModel;
  addData(value?: SecurityAuditModel, index?: number): SecurityAuditModel;

  getPagination(): Pagination | undefined;
  setPagination(value?: Pagination): SecurityAuditPaginationModel;
  hasPagination(): boolean;
  clearPagination(): SecurityAuditPaginationModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SecurityAuditPaginationModel.AsObject;
  static toObject(includeInstance: boolean, msg: SecurityAuditPaginationModel): SecurityAuditPaginationModel.AsObject;
  static serializeBinaryToWriter(message: SecurityAuditPaginationModel, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SecurityAuditPaginationModel;
  static deserializeBinaryFromReader(message: SecurityAuditPaginationModel, reader: jspb.BinaryReader): SecurityAuditPaginationModel;
}

export namespace SecurityAuditPaginationModel {
  export type AsObject = {
    dataList: Array<SecurityAuditModel.AsObject>,
    pagination?: Pagination.AsObject,
  }
}

export class SecurityAlertPaginationModel extends jspb.Message {
  getDataList(): Array<SecurityAlertModel>;
  setDataList(value: Array<SecurityAlertModel>): SecurityAlertPaginationModel;
  clearDataList(): SecurityAlertPaginationModel;
  addData(value?: SecurityAlertModel, index?: number): SecurityAlertModel;

  getPagination(): Pagination | undefined;
  setPagination(value?: Pagination): SecurityAlertPaginationModel;
  hasPagination(): boolean;
  clearPagination(): SecurityAlertPaginationModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SecurityAlertPaginationModel.AsObject;
  static toObject(includeInstance: boolean, msg: SecurityAlertPaginationModel): SecurityAlertPaginationModel.AsObject;
  static serializeBinaryToWriter(message: SecurityAlertPaginationModel, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SecurityAlertPaginationModel;
  static deserializeBinaryFromReader(message: SecurityAlertPaginationModel, reader: jspb.BinaryReader): SecurityAlertPaginationModel;
}

export namespace SecurityAlertPaginationModel {
  export type AsObject = {
    dataList: Array<SecurityAlertModel.AsObject>,
    pagination?: Pagination.AsObject,
  }
}

export class SecuritySummary extends jspb.Message {
  getIpAddress(): string;
  setIpAddress(value: string): SecuritySummary;

  getTotalRecords(): number;
  setTotalRecords(value: number): SecuritySummary;

  getTotalAuthenticationAttempts(): number;
  setTotalAuthenticationAttempts(value: number): SecuritySummary;

  getFailedAuthenticationAttempts(): number;
  setFailedAuthenticationAttempts(value: number): SecuritySummary;

  getBlockedInjectionAttempts(): number;
  setBlockedInjectionAttempts(value: number): SecuritySummary;

  getRateLimitedRequests(): number;
  setRateLimitedRequests(value: number): SecuritySummary;

  getSuspiciousActivitiesDetected(): number;
  setSuspiciousActivitiesDetected(value: number): SecuritySummary;

  getSecurityViolations(): number;
  setSecurityViolations(value: number): SecuritySummary;

  getLowestHealthScore(): number;
  setLowestHealthScore(value: number): SecuritySummary;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SecuritySummary.AsObject;
  static toObject(includeInstance: boolean, msg: SecuritySummary): SecuritySummary.AsObject;
  static serializeBinaryToWriter(message: SecuritySummary, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SecuritySummary;
  static deserializeBinaryFromReader(message: SecuritySummary, reader: jspb.BinaryReader): SecuritySummary;
}

export namespace SecuritySummary {
  export type AsObject = {
    ipAddress: string,
    totalRecords: number,
    totalAuthenticationAttempts: number,
    failedAuthenticationAttempts: number,
    blockedInjectionAttempts: number,
    rateLimitedRequests: number,
    suspiciousActivitiesDetected: number,
    securityViolations: number,
    lowestHealthScore: number,
  }
}

export class AlertStatistics extends jspb.Message {
  getTotalAlerts(): number;
  setTotalAlerts(value: number): AlertStatistics;

  getTotalUnresolved(): number;
  setTotalUnresolved(value: number): AlertStatistics;

  getTotalCriticalUnresolved(): number;
  setTotalCriticalUnresolved(value: number): AlertStatistics;

  getTotalLow(): number;
  setTotalLow(value: number): AlertStatistics;

  getTotalMedium(): number;
  setTotalMedium(value: number): AlertStatistics;

  getTotalHigh(): number;
  setTotalHigh(value: number): AlertStatistics;

  getTotalCritical(): number;
  setTotalCritical(value: number): AlertStatistics;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AlertStatistics.AsObject;
  static toObject(includeInstance: boolean, msg: AlertStatistics): AlertStatistics.AsObject;
  static serializeBinaryToWriter(message: AlertStatistics, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AlertStatistics;
  static deserializeBinaryFromReader(message: AlertStatistics, reader: jspb.BinaryReader): AlertStatistics;
}

export namespace AlertStatistics {
  export type AsObject = {
    totalAlerts: number,
    totalUnresolved: number,
    totalCriticalUnresolved: number,
    totalLow: number,
    totalMedium: number,
    totalHigh: number,
    totalCritical: number,
  }
}

export class CreateSecurityAuditRequest extends jspb.Message {
  getAudit(): CreateSecurityAuditModel | undefined;
  setAudit(value?: CreateSecurityAuditModel): CreateSecurityAuditRequest;
  hasAudit(): boolean;
  clearAudit(): CreateSecurityAuditRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateSecurityAuditRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateSecurityAuditRequest): CreateSecurityAuditRequest.AsObject;
  static serializeBinaryToWriter(message: CreateSecurityAuditRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateSecurityAuditRequest;
  static deserializeBinaryFromReader(message: CreateSecurityAuditRequest, reader: jspb.BinaryReader): CreateSecurityAuditRequest;
}

export namespace CreateSecurityAuditRequest {
  export type AsObject = {
    audit?: CreateSecurityAuditModel.AsObject,
  }
}

export class CreateSecurityAuditResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): CreateSecurityAuditResponse;

  getData(): SecurityAuditModel | undefined;
  setData(value?: SecurityAuditModel): CreateSecurityAuditResponse;
  hasData(): boolean;
  clearData(): CreateSecurityAuditResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateSecurityAuditResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CreateSecurityAuditResponse): CreateSecurityAuditResponse.AsObject;
  static serializeBinaryToWriter(message: CreateSecurityAuditResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateSecurityAuditResponse;
  static deserializeBinaryFromReader(message: CreateSecurityAuditResponse, reader: jspb.BinaryReader): CreateSecurityAuditResponse;
}

export namespace CreateSecurityAuditResponse {
  export type AsObject = {
    status: boolean,
    data?: SecurityAuditModel.AsObject,
  }
}

export class LogSecurityEventRequest extends jspb.Message {
  getAdminUserId(): string;
  setAdminUserId(value: string): LogSecurityEventRequest;
  hasAdminUserId(): boolean;
  clearAdminUserId(): LogSecurityEventRequest;

  getSessionId(): string;
  setSessionId(value: string): LogSecurityEventRequest;
  hasSessionId(): boolean;
  clearSessionId(): LogSecurityEventRequest;

  getIpAddress(): string;
  setIpAddress(value: string): LogSecurityEventRequest;

  getUserAgent(): string;
  setUserAgent(value: string): LogSecurityEventRequest;
  hasUserAgent(): boolean;
  clearUserAgent(): LogSecurityEventRequest;

  getEndpoint(): string;
  setEndpoint(value: string): LogSecurityEventRequest;
  hasEndpoint(): boolean;
  clearEndpoint(): LogSecurityEventRequest;

  getRequestMethod(): string;
  setRequestMethod(value: string): LogSecurityEventRequest;
  hasRequestMethod(): boolean;
  clearRequestMethod(): LogSecurityEventRequest;

  getEventType(): SecurityEventType;
  setEventType(value: SecurityEventType): LogSecurityEventRequest;

  getMetadataJson(): string;
  setMetadataJson(value: string): LogSecurityEventRequest;
  hasMetadataJson(): boolean;
  clearMetadataJson(): LogSecurityEventRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LogSecurityEventRequest.AsObject;
  static toObject(includeInstance: boolean, msg: LogSecurityEventRequest): LogSecurityEventRequest.AsObject;
  static serializeBinaryToWriter(message: LogSecurityEventRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LogSecurityEventRequest;
  static deserializeBinaryFromReader(message: LogSecurityEventRequest, reader: jspb.BinaryReader): LogSecurityEventRequest;
}

export namespace LogSecurityEventRequest {
  export type AsObject = {
    adminUserId?: string,
    sessionId?: string,
    ipAddress: string,
    userAgent?: string,
    endpoint?: string,
    requestMethod?: string,
    eventType: SecurityEventType,
    metadataJson?: string,
  }

  export enum AdminUserIdCase { 
    _ADMIN_USER_ID_NOT_SET = 0,
    ADMIN_USER_ID = 1,
  }

  export enum SessionIdCase { 
    _SESSION_ID_NOT_SET = 0,
    SESSION_ID = 2,
  }

  export enum UserAgentCase { 
    _USER_AGENT_NOT_SET = 0,
    USER_AGENT = 4,
  }

  export enum EndpointCase { 
    _ENDPOINT_NOT_SET = 0,
    ENDPOINT = 5,
  }

  export enum RequestMethodCase { 
    _REQUEST_METHOD_NOT_SET = 0,
    REQUEST_METHOD = 6,
  }

  export enum MetadataJsonCase { 
    _METADATA_JSON_NOT_SET = 0,
    METADATA_JSON = 8,
  }
}

export class LogSecurityEventResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): LogSecurityEventResponse;

  getData(): SecurityAuditModel | undefined;
  setData(value?: SecurityAuditModel): LogSecurityEventResponse;
  hasData(): boolean;
  clearData(): LogSecurityEventResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): LogSecurityEventResponse.AsObject;
  static toObject(includeInstance: boolean, msg: LogSecurityEventResponse): LogSecurityEventResponse.AsObject;
  static serializeBinaryToWriter(message: LogSecurityEventResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): LogSecurityEventResponse;
  static deserializeBinaryFromReader(message: LogSecurityEventResponse, reader: jspb.BinaryReader): LogSecurityEventResponse;
}

export namespace LogSecurityEventResponse {
  export type AsObject = {
    status: boolean,
    data?: SecurityAuditModel.AsObject,
  }
}

export class GetSecurityAuditRequest extends jspb.Message {
  getId(): string;
  setId(value: string): GetSecurityAuditRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetSecurityAuditRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetSecurityAuditRequest): GetSecurityAuditRequest.AsObject;
  static serializeBinaryToWriter(message: GetSecurityAuditRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetSecurityAuditRequest;
  static deserializeBinaryFromReader(message: GetSecurityAuditRequest, reader: jspb.BinaryReader): GetSecurityAuditRequest;
}

export namespace GetSecurityAuditRequest {
  export type AsObject = {
    id: string,
  }
}

export class GetSecurityAuditResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): GetSecurityAuditResponse;

  getData(): SecurityAuditModel | undefined;
  setData(value?: SecurityAuditModel): GetSecurityAuditResponse;
  hasData(): boolean;
  clearData(): GetSecurityAuditResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetSecurityAuditResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetSecurityAuditResponse): GetSecurityAuditResponse.AsObject;
  static serializeBinaryToWriter(message: GetSecurityAuditResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetSecurityAuditResponse;
  static deserializeBinaryFromReader(message: GetSecurityAuditResponse, reader: jspb.BinaryReader): GetSecurityAuditResponse;
}

export namespace GetSecurityAuditResponse {
  export type AsObject = {
    status: boolean,
    data?: SecurityAuditModel.AsObject,
  }
}

export class GetSecurityAuditsByUserRequest extends jspb.Message {
  getAdminUserId(): string;
  setAdminUserId(value: string): GetSecurityAuditsByUserRequest;

  getPage(): number;
  setPage(value: number): GetSecurityAuditsByUserRequest;

  getPerPage(): number;
  setPerPage(value: number): GetSecurityAuditsByUserRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetSecurityAuditsByUserRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetSecurityAuditsByUserRequest): GetSecurityAuditsByUserRequest.AsObject;
  static serializeBinaryToWriter(message: GetSecurityAuditsByUserRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetSecurityAuditsByUserRequest;
  static deserializeBinaryFromReader(message: GetSecurityAuditsByUserRequest, reader: jspb.BinaryReader): GetSecurityAuditsByUserRequest;
}

export namespace GetSecurityAuditsByUserRequest {
  export type AsObject = {
    adminUserId: string,
    page: number,
    perPage: number,
  }
}

export class GetSecurityAuditsByUserResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): GetSecurityAuditsByUserResponse;

  getData(): SecurityAuditPaginationModel | undefined;
  setData(value?: SecurityAuditPaginationModel): GetSecurityAuditsByUserResponse;
  hasData(): boolean;
  clearData(): GetSecurityAuditsByUserResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetSecurityAuditsByUserResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetSecurityAuditsByUserResponse): GetSecurityAuditsByUserResponse.AsObject;
  static serializeBinaryToWriter(message: GetSecurityAuditsByUserResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetSecurityAuditsByUserResponse;
  static deserializeBinaryFromReader(message: GetSecurityAuditsByUserResponse, reader: jspb.BinaryReader): GetSecurityAuditsByUserResponse;
}

export namespace GetSecurityAuditsByUserResponse {
  export type AsObject = {
    status: boolean,
    data?: SecurityAuditPaginationModel.AsObject,
  }
}

export class GetSecurityAuditsByIpRequest extends jspb.Message {
  getIpAddress(): string;
  setIpAddress(value: string): GetSecurityAuditsByIpRequest;

  getPage(): number;
  setPage(value: number): GetSecurityAuditsByIpRequest;

  getPerPage(): number;
  setPerPage(value: number): GetSecurityAuditsByIpRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetSecurityAuditsByIpRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetSecurityAuditsByIpRequest): GetSecurityAuditsByIpRequest.AsObject;
  static serializeBinaryToWriter(message: GetSecurityAuditsByIpRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetSecurityAuditsByIpRequest;
  static deserializeBinaryFromReader(message: GetSecurityAuditsByIpRequest, reader: jspb.BinaryReader): GetSecurityAuditsByIpRequest;
}

export namespace GetSecurityAuditsByIpRequest {
  export type AsObject = {
    ipAddress: string,
    page: number,
    perPage: number,
  }
}

export class GetSecurityAuditsByIpResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): GetSecurityAuditsByIpResponse;

  getData(): SecurityAuditPaginationModel | undefined;
  setData(value?: SecurityAuditPaginationModel): GetSecurityAuditsByIpResponse;
  hasData(): boolean;
  clearData(): GetSecurityAuditsByIpResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetSecurityAuditsByIpResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetSecurityAuditsByIpResponse): GetSecurityAuditsByIpResponse.AsObject;
  static serializeBinaryToWriter(message: GetSecurityAuditsByIpResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetSecurityAuditsByIpResponse;
  static deserializeBinaryFromReader(message: GetSecurityAuditsByIpResponse, reader: jspb.BinaryReader): GetSecurityAuditsByIpResponse;
}

export namespace GetSecurityAuditsByIpResponse {
  export type AsObject = {
    status: boolean,
    data?: SecurityAuditPaginationModel.AsObject,
  }
}

export class GetSecurityAuditsPaginatedRequest extends jspb.Message {
  getPage(): number;
  setPage(value: number): GetSecurityAuditsPaginatedRequest;

  getPerPage(): number;
  setPerPage(value: number): GetSecurityAuditsPaginatedRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetSecurityAuditsPaginatedRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetSecurityAuditsPaginatedRequest): GetSecurityAuditsPaginatedRequest.AsObject;
  static serializeBinaryToWriter(message: GetSecurityAuditsPaginatedRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetSecurityAuditsPaginatedRequest;
  static deserializeBinaryFromReader(message: GetSecurityAuditsPaginatedRequest, reader: jspb.BinaryReader): GetSecurityAuditsPaginatedRequest;
}

export namespace GetSecurityAuditsPaginatedRequest {
  export type AsObject = {
    page: number,
    perPage: number,
  }
}

export class GetSecurityAuditsPaginatedResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): GetSecurityAuditsPaginatedResponse;

  getData(): SecurityAuditPaginationModel | undefined;
  setData(value?: SecurityAuditPaginationModel): GetSecurityAuditsPaginatedResponse;
  hasData(): boolean;
  clearData(): GetSecurityAuditsPaginatedResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetSecurityAuditsPaginatedResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetSecurityAuditsPaginatedResponse): GetSecurityAuditsPaginatedResponse.AsObject;
  static serializeBinaryToWriter(message: GetSecurityAuditsPaginatedResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetSecurityAuditsPaginatedResponse;
  static deserializeBinaryFromReader(message: GetSecurityAuditsPaginatedResponse, reader: jspb.BinaryReader): GetSecurityAuditsPaginatedResponse;
}

export namespace GetSecurityAuditsPaginatedResponse {
  export type AsObject = {
    status: boolean,
    data?: SecurityAuditPaginationModel.AsObject,
  }
}

export class UpdateSecurityAuditRequest extends jspb.Message {
  getId(): string;
  setId(value: string): UpdateSecurityAuditRequest;

  getAudit(): UpdateSecurityAuditModel | undefined;
  setAudit(value?: UpdateSecurityAuditModel): UpdateSecurityAuditRequest;
  hasAudit(): boolean;
  clearAudit(): UpdateSecurityAuditRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateSecurityAuditRequest.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateSecurityAuditRequest): UpdateSecurityAuditRequest.AsObject;
  static serializeBinaryToWriter(message: UpdateSecurityAuditRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateSecurityAuditRequest;
  static deserializeBinaryFromReader(message: UpdateSecurityAuditRequest, reader: jspb.BinaryReader): UpdateSecurityAuditRequest;
}

export namespace UpdateSecurityAuditRequest {
  export type AsObject = {
    id: string,
    audit?: UpdateSecurityAuditModel.AsObject,
  }
}

export class UpdateSecurityAuditResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): UpdateSecurityAuditResponse;

  getData(): SecurityAuditModel | undefined;
  setData(value?: SecurityAuditModel): UpdateSecurityAuditResponse;
  hasData(): boolean;
  clearData(): UpdateSecurityAuditResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UpdateSecurityAuditResponse.AsObject;
  static toObject(includeInstance: boolean, msg: UpdateSecurityAuditResponse): UpdateSecurityAuditResponse.AsObject;
  static serializeBinaryToWriter(message: UpdateSecurityAuditResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UpdateSecurityAuditResponse;
  static deserializeBinaryFromReader(message: UpdateSecurityAuditResponse, reader: jspb.BinaryReader): UpdateSecurityAuditResponse;
}

export namespace UpdateSecurityAuditResponse {
  export type AsObject = {
    status: boolean,
    data?: SecurityAuditModel.AsObject,
  }
}

export class DeleteSecurityAuditRequest extends jspb.Message {
  getId(): string;
  setId(value: string): DeleteSecurityAuditRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteSecurityAuditRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteSecurityAuditRequest): DeleteSecurityAuditRequest.AsObject;
  static serializeBinaryToWriter(message: DeleteSecurityAuditRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteSecurityAuditRequest;
  static deserializeBinaryFromReader(message: DeleteSecurityAuditRequest, reader: jspb.BinaryReader): DeleteSecurityAuditRequest;
}

export namespace DeleteSecurityAuditRequest {
  export type AsObject = {
    id: string,
  }
}

export class DeleteSecurityAuditResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): DeleteSecurityAuditResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteSecurityAuditResponse.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteSecurityAuditResponse): DeleteSecurityAuditResponse.AsObject;
  static serializeBinaryToWriter(message: DeleteSecurityAuditResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteSecurityAuditResponse;
  static deserializeBinaryFromReader(message: DeleteSecurityAuditResponse, reader: jspb.BinaryReader): DeleteSecurityAuditResponse;
}

export namespace DeleteSecurityAuditResponse {
  export type AsObject = {
    status: boolean,
  }
}

export class GetIpSecuritySummaryRequest extends jspb.Message {
  getIpAddress(): string;
  setIpAddress(value: string): GetIpSecuritySummaryRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetIpSecuritySummaryRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetIpSecuritySummaryRequest): GetIpSecuritySummaryRequest.AsObject;
  static serializeBinaryToWriter(message: GetIpSecuritySummaryRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetIpSecuritySummaryRequest;
  static deserializeBinaryFromReader(message: GetIpSecuritySummaryRequest, reader: jspb.BinaryReader): GetIpSecuritySummaryRequest;
}

export namespace GetIpSecuritySummaryRequest {
  export type AsObject = {
    ipAddress: string,
  }
}

export class GetIpSecuritySummaryResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): GetIpSecuritySummaryResponse;

  getData(): SecuritySummary | undefined;
  setData(value?: SecuritySummary): GetIpSecuritySummaryResponse;
  hasData(): boolean;
  clearData(): GetIpSecuritySummaryResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetIpSecuritySummaryResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetIpSecuritySummaryResponse): GetIpSecuritySummaryResponse.AsObject;
  static serializeBinaryToWriter(message: GetIpSecuritySummaryResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetIpSecuritySummaryResponse;
  static deserializeBinaryFromReader(message: GetIpSecuritySummaryResponse, reader: jspb.BinaryReader): GetIpSecuritySummaryResponse;
}

export namespace GetIpSecuritySummaryResponse {
  export type AsObject = {
    status: boolean,
    data?: SecuritySummary.AsObject,
  }
}

export class CreateSecurityAlertRequest extends jspb.Message {
  getAlert(): CreateSecurityAlertModel | undefined;
  setAlert(value?: CreateSecurityAlertModel): CreateSecurityAlertRequest;
  hasAlert(): boolean;
  clearAlert(): CreateSecurityAlertRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateSecurityAlertRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateSecurityAlertRequest): CreateSecurityAlertRequest.AsObject;
  static serializeBinaryToWriter(message: CreateSecurityAlertRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateSecurityAlertRequest;
  static deserializeBinaryFromReader(message: CreateSecurityAlertRequest, reader: jspb.BinaryReader): CreateSecurityAlertRequest;
}

export namespace CreateSecurityAlertRequest {
  export type AsObject = {
    alert?: CreateSecurityAlertModel.AsObject,
  }
}

export class CreateSecurityAlertResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): CreateSecurityAlertResponse;

  getData(): SecurityAlertModel | undefined;
  setData(value?: SecurityAlertModel): CreateSecurityAlertResponse;
  hasData(): boolean;
  clearData(): CreateSecurityAlertResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateSecurityAlertResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CreateSecurityAlertResponse): CreateSecurityAlertResponse.AsObject;
  static serializeBinaryToWriter(message: CreateSecurityAlertResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateSecurityAlertResponse;
  static deserializeBinaryFromReader(message: CreateSecurityAlertResponse, reader: jspb.BinaryReader): CreateSecurityAlertResponse;
}

export namespace CreateSecurityAlertResponse {
  export type AsObject = {
    status: boolean,
    data?: SecurityAlertModel.AsObject,
  }
}

export class CreateSecurityAlertAutoIdRequest extends jspb.Message {
  getAlertType(): AlertType;
  setAlertType(value: AlertType): CreateSecurityAlertAutoIdRequest;

  getSeverity(): AlertSeverity;
  setSeverity(value: AlertSeverity): CreateSecurityAlertAutoIdRequest;

  getMessage(): string;
  setMessage(value: string): CreateSecurityAlertAutoIdRequest;

  getSource(): string;
  setSource(value: string): CreateSecurityAlertAutoIdRequest;

  getAffectedResource(): string;
  setAffectedResource(value: string): CreateSecurityAlertAutoIdRequest;
  hasAffectedResource(): boolean;
  clearAffectedResource(): CreateSecurityAlertAutoIdRequest;

  getMetadataJson(): string;
  setMetadataJson(value: string): CreateSecurityAlertAutoIdRequest;
  hasMetadataJson(): boolean;
  clearMetadataJson(): CreateSecurityAlertAutoIdRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateSecurityAlertAutoIdRequest.AsObject;
  static toObject(includeInstance: boolean, msg: CreateSecurityAlertAutoIdRequest): CreateSecurityAlertAutoIdRequest.AsObject;
  static serializeBinaryToWriter(message: CreateSecurityAlertAutoIdRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateSecurityAlertAutoIdRequest;
  static deserializeBinaryFromReader(message: CreateSecurityAlertAutoIdRequest, reader: jspb.BinaryReader): CreateSecurityAlertAutoIdRequest;
}

export namespace CreateSecurityAlertAutoIdRequest {
  export type AsObject = {
    alertType: AlertType,
    severity: AlertSeverity,
    message: string,
    source: string,
    affectedResource?: string,
    metadataJson?: string,
  }

  export enum AffectedResourceCase { 
    _AFFECTED_RESOURCE_NOT_SET = 0,
    AFFECTED_RESOURCE = 5,
  }

  export enum MetadataJsonCase { 
    _METADATA_JSON_NOT_SET = 0,
    METADATA_JSON = 6,
  }
}

export class CreateSecurityAlertAutoIdResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): CreateSecurityAlertAutoIdResponse;

  getData(): SecurityAlertModel | undefined;
  setData(value?: SecurityAlertModel): CreateSecurityAlertAutoIdResponse;
  hasData(): boolean;
  clearData(): CreateSecurityAlertAutoIdResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CreateSecurityAlertAutoIdResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CreateSecurityAlertAutoIdResponse): CreateSecurityAlertAutoIdResponse.AsObject;
  static serializeBinaryToWriter(message: CreateSecurityAlertAutoIdResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CreateSecurityAlertAutoIdResponse;
  static deserializeBinaryFromReader(message: CreateSecurityAlertAutoIdResponse, reader: jspb.BinaryReader): CreateSecurityAlertAutoIdResponse;
}

export namespace CreateSecurityAlertAutoIdResponse {
  export type AsObject = {
    status: boolean,
    data?: SecurityAlertModel.AsObject,
  }
}

export class GetSecurityAlertRequest extends jspb.Message {
  getId(): string;
  setId(value: string): GetSecurityAlertRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetSecurityAlertRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetSecurityAlertRequest): GetSecurityAlertRequest.AsObject;
  static serializeBinaryToWriter(message: GetSecurityAlertRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetSecurityAlertRequest;
  static deserializeBinaryFromReader(message: GetSecurityAlertRequest, reader: jspb.BinaryReader): GetSecurityAlertRequest;
}

export namespace GetSecurityAlertRequest {
  export type AsObject = {
    id: string,
  }
}

export class GetSecurityAlertResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): GetSecurityAlertResponse;

  getData(): SecurityAlertModel | undefined;
  setData(value?: SecurityAlertModel): GetSecurityAlertResponse;
  hasData(): boolean;
  clearData(): GetSecurityAlertResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetSecurityAlertResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetSecurityAlertResponse): GetSecurityAlertResponse.AsObject;
  static serializeBinaryToWriter(message: GetSecurityAlertResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetSecurityAlertResponse;
  static deserializeBinaryFromReader(message: GetSecurityAlertResponse, reader: jspb.BinaryReader): GetSecurityAlertResponse;
}

export namespace GetSecurityAlertResponse {
  export type AsObject = {
    status: boolean,
    data?: SecurityAlertModel.AsObject,
  }
}

export class GetUnresolvedAlertsBySeverityRequest extends jspb.Message {
  getSeverity(): AlertSeverity;
  setSeverity(value: AlertSeverity): GetUnresolvedAlertsBySeverityRequest;

  getPage(): number;
  setPage(value: number): GetUnresolvedAlertsBySeverityRequest;

  getPerPage(): number;
  setPerPage(value: number): GetUnresolvedAlertsBySeverityRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetUnresolvedAlertsBySeverityRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetUnresolvedAlertsBySeverityRequest): GetUnresolvedAlertsBySeverityRequest.AsObject;
  static serializeBinaryToWriter(message: GetUnresolvedAlertsBySeverityRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetUnresolvedAlertsBySeverityRequest;
  static deserializeBinaryFromReader(message: GetUnresolvedAlertsBySeverityRequest, reader: jspb.BinaryReader): GetUnresolvedAlertsBySeverityRequest;
}

export namespace GetUnresolvedAlertsBySeverityRequest {
  export type AsObject = {
    severity: AlertSeverity,
    page: number,
    perPage: number,
  }
}

export class GetUnresolvedAlertsBySeverityResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): GetUnresolvedAlertsBySeverityResponse;

  getData(): SecurityAlertPaginationModel | undefined;
  setData(value?: SecurityAlertPaginationModel): GetUnresolvedAlertsBySeverityResponse;
  hasData(): boolean;
  clearData(): GetUnresolvedAlertsBySeverityResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetUnresolvedAlertsBySeverityResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetUnresolvedAlertsBySeverityResponse): GetUnresolvedAlertsBySeverityResponse.AsObject;
  static serializeBinaryToWriter(message: GetUnresolvedAlertsBySeverityResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetUnresolvedAlertsBySeverityResponse;
  static deserializeBinaryFromReader(message: GetUnresolvedAlertsBySeverityResponse, reader: jspb.BinaryReader): GetUnresolvedAlertsBySeverityResponse;
}

export namespace GetUnresolvedAlertsBySeverityResponse {
  export type AsObject = {
    status: boolean,
    data?: SecurityAlertPaginationModel.AsObject,
  }
}

export class GetAlertsByTypeRequest extends jspb.Message {
  getAlertType(): AlertType;
  setAlertType(value: AlertType): GetAlertsByTypeRequest;

  getPage(): number;
  setPage(value: number): GetAlertsByTypeRequest;

  getPerPage(): number;
  setPerPage(value: number): GetAlertsByTypeRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetAlertsByTypeRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetAlertsByTypeRequest): GetAlertsByTypeRequest.AsObject;
  static serializeBinaryToWriter(message: GetAlertsByTypeRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetAlertsByTypeRequest;
  static deserializeBinaryFromReader(message: GetAlertsByTypeRequest, reader: jspb.BinaryReader): GetAlertsByTypeRequest;
}

export namespace GetAlertsByTypeRequest {
  export type AsObject = {
    alertType: AlertType,
    page: number,
    perPage: number,
  }
}

export class GetAlertsByTypeResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): GetAlertsByTypeResponse;

  getData(): SecurityAlertPaginationModel | undefined;
  setData(value?: SecurityAlertPaginationModel): GetAlertsByTypeResponse;
  hasData(): boolean;
  clearData(): GetAlertsByTypeResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetAlertsByTypeResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetAlertsByTypeResponse): GetAlertsByTypeResponse.AsObject;
  static serializeBinaryToWriter(message: GetAlertsByTypeResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetAlertsByTypeResponse;
  static deserializeBinaryFromReader(message: GetAlertsByTypeResponse, reader: jspb.BinaryReader): GetAlertsByTypeResponse;
}

export namespace GetAlertsByTypeResponse {
  export type AsObject = {
    status: boolean,
    data?: SecurityAlertPaginationModel.AsObject,
  }
}

export class GetAlertsBySourceRequest extends jspb.Message {
  getSource(): string;
  setSource(value: string): GetAlertsBySourceRequest;

  getPage(): number;
  setPage(value: number): GetAlertsBySourceRequest;

  getPerPage(): number;
  setPerPage(value: number): GetAlertsBySourceRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetAlertsBySourceRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetAlertsBySourceRequest): GetAlertsBySourceRequest.AsObject;
  static serializeBinaryToWriter(message: GetAlertsBySourceRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetAlertsBySourceRequest;
  static deserializeBinaryFromReader(message: GetAlertsBySourceRequest, reader: jspb.BinaryReader): GetAlertsBySourceRequest;
}

export namespace GetAlertsBySourceRequest {
  export type AsObject = {
    source: string,
    page: number,
    perPage: number,
  }
}

export class GetAlertsBySourceResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): GetAlertsBySourceResponse;

  getData(): SecurityAlertPaginationModel | undefined;
  setData(value?: SecurityAlertPaginationModel): GetAlertsBySourceResponse;
  hasData(): boolean;
  clearData(): GetAlertsBySourceResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetAlertsBySourceResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetAlertsBySourceResponse): GetAlertsBySourceResponse.AsObject;
  static serializeBinaryToWriter(message: GetAlertsBySourceResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetAlertsBySourceResponse;
  static deserializeBinaryFromReader(message: GetAlertsBySourceResponse, reader: jspb.BinaryReader): GetAlertsBySourceResponse;
}

export namespace GetAlertsBySourceResponse {
  export type AsObject = {
    status: boolean,
    data?: SecurityAlertPaginationModel.AsObject,
  }
}

export class ResolveSecurityAlertRequest extends jspb.Message {
  getId(): string;
  setId(value: string): ResolveSecurityAlertRequest;

  getResolvedBy(): string;
  setResolvedBy(value: string): ResolveSecurityAlertRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ResolveSecurityAlertRequest.AsObject;
  static toObject(includeInstance: boolean, msg: ResolveSecurityAlertRequest): ResolveSecurityAlertRequest.AsObject;
  static serializeBinaryToWriter(message: ResolveSecurityAlertRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ResolveSecurityAlertRequest;
  static deserializeBinaryFromReader(message: ResolveSecurityAlertRequest, reader: jspb.BinaryReader): ResolveSecurityAlertRequest;
}

export namespace ResolveSecurityAlertRequest {
  export type AsObject = {
    id: string,
    resolvedBy: string,
  }
}

export class ResolveSecurityAlertResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): ResolveSecurityAlertResponse;

  getData(): SecurityAlertModel | undefined;
  setData(value?: SecurityAlertModel): ResolveSecurityAlertResponse;
  hasData(): boolean;
  clearData(): ResolveSecurityAlertResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ResolveSecurityAlertResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ResolveSecurityAlertResponse): ResolveSecurityAlertResponse.AsObject;
  static serializeBinaryToWriter(message: ResolveSecurityAlertResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ResolveSecurityAlertResponse;
  static deserializeBinaryFromReader(message: ResolveSecurityAlertResponse, reader: jspb.BinaryReader): ResolveSecurityAlertResponse;
}

export namespace ResolveSecurityAlertResponse {
  export type AsObject = {
    status: boolean,
    data?: SecurityAlertModel.AsObject,
  }
}

export class GetSecurityAlertsPaginatedRequest extends jspb.Message {
  getPage(): number;
  setPage(value: number): GetSecurityAlertsPaginatedRequest;

  getPerPage(): number;
  setPerPage(value: number): GetSecurityAlertsPaginatedRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetSecurityAlertsPaginatedRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetSecurityAlertsPaginatedRequest): GetSecurityAlertsPaginatedRequest.AsObject;
  static serializeBinaryToWriter(message: GetSecurityAlertsPaginatedRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetSecurityAlertsPaginatedRequest;
  static deserializeBinaryFromReader(message: GetSecurityAlertsPaginatedRequest, reader: jspb.BinaryReader): GetSecurityAlertsPaginatedRequest;
}

export namespace GetSecurityAlertsPaginatedRequest {
  export type AsObject = {
    page: number,
    perPage: number,
  }
}

export class GetSecurityAlertsPaginatedResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): GetSecurityAlertsPaginatedResponse;

  getData(): SecurityAlertPaginationModel | undefined;
  setData(value?: SecurityAlertPaginationModel): GetSecurityAlertsPaginatedResponse;
  hasData(): boolean;
  clearData(): GetSecurityAlertsPaginatedResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetSecurityAlertsPaginatedResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetSecurityAlertsPaginatedResponse): GetSecurityAlertsPaginatedResponse.AsObject;
  static serializeBinaryToWriter(message: GetSecurityAlertsPaginatedResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetSecurityAlertsPaginatedResponse;
  static deserializeBinaryFromReader(message: GetSecurityAlertsPaginatedResponse, reader: jspb.BinaryReader): GetSecurityAlertsPaginatedResponse;
}

export namespace GetSecurityAlertsPaginatedResponse {
  export type AsObject = {
    status: boolean,
    data?: SecurityAlertPaginationModel.AsObject,
  }
}

export class GetCriticalUnresolvedAlertsRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetCriticalUnresolvedAlertsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetCriticalUnresolvedAlertsRequest): GetCriticalUnresolvedAlertsRequest.AsObject;
  static serializeBinaryToWriter(message: GetCriticalUnresolvedAlertsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetCriticalUnresolvedAlertsRequest;
  static deserializeBinaryFromReader(message: GetCriticalUnresolvedAlertsRequest, reader: jspb.BinaryReader): GetCriticalUnresolvedAlertsRequest;
}

export namespace GetCriticalUnresolvedAlertsRequest {
  export type AsObject = {
  }
}

export class GetCriticalUnresolvedAlertsResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): GetCriticalUnresolvedAlertsResponse;

  getDataList(): Array<SecurityAlertModel>;
  setDataList(value: Array<SecurityAlertModel>): GetCriticalUnresolvedAlertsResponse;
  clearDataList(): GetCriticalUnresolvedAlertsResponse;
  addData(value?: SecurityAlertModel, index?: number): SecurityAlertModel;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetCriticalUnresolvedAlertsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetCriticalUnresolvedAlertsResponse): GetCriticalUnresolvedAlertsResponse.AsObject;
  static serializeBinaryToWriter(message: GetCriticalUnresolvedAlertsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetCriticalUnresolvedAlertsResponse;
  static deserializeBinaryFromReader(message: GetCriticalUnresolvedAlertsResponse, reader: jspb.BinaryReader): GetCriticalUnresolvedAlertsResponse;
}

export namespace GetCriticalUnresolvedAlertsResponse {
  export type AsObject = {
    status: boolean,
    dataList: Array<SecurityAlertModel.AsObject>,
  }
}

export class DeleteSecurityAlertRequest extends jspb.Message {
  getId(): string;
  setId(value: string): DeleteSecurityAlertRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteSecurityAlertRequest.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteSecurityAlertRequest): DeleteSecurityAlertRequest.AsObject;
  static serializeBinaryToWriter(message: DeleteSecurityAlertRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteSecurityAlertRequest;
  static deserializeBinaryFromReader(message: DeleteSecurityAlertRequest, reader: jspb.BinaryReader): DeleteSecurityAlertRequest;
}

export namespace DeleteSecurityAlertRequest {
  export type AsObject = {
    id: string,
  }
}

export class DeleteSecurityAlertResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): DeleteSecurityAlertResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): DeleteSecurityAlertResponse.AsObject;
  static toObject(includeInstance: boolean, msg: DeleteSecurityAlertResponse): DeleteSecurityAlertResponse.AsObject;
  static serializeBinaryToWriter(message: DeleteSecurityAlertResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): DeleteSecurityAlertResponse;
  static deserializeBinaryFromReader(message: DeleteSecurityAlertResponse, reader: jspb.BinaryReader): DeleteSecurityAlertResponse;
}

export namespace DeleteSecurityAlertResponse {
  export type AsObject = {
    status: boolean,
  }
}

export class GetAlertStatisticsRequest extends jspb.Message {
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetAlertStatisticsRequest.AsObject;
  static toObject(includeInstance: boolean, msg: GetAlertStatisticsRequest): GetAlertStatisticsRequest.AsObject;
  static serializeBinaryToWriter(message: GetAlertStatisticsRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetAlertStatisticsRequest;
  static deserializeBinaryFromReader(message: GetAlertStatisticsRequest, reader: jspb.BinaryReader): GetAlertStatisticsRequest;
}

export namespace GetAlertStatisticsRequest {
  export type AsObject = {
  }
}

export class GetAlertStatisticsResponse extends jspb.Message {
  getStatus(): boolean;
  setStatus(value: boolean): GetAlertStatisticsResponse;

  getData(): AlertStatistics | undefined;
  setData(value?: AlertStatistics): GetAlertStatisticsResponse;
  hasData(): boolean;
  clearData(): GetAlertStatisticsResponse;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GetAlertStatisticsResponse.AsObject;
  static toObject(includeInstance: boolean, msg: GetAlertStatisticsResponse): GetAlertStatisticsResponse.AsObject;
  static serializeBinaryToWriter(message: GetAlertStatisticsResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GetAlertStatisticsResponse;
  static deserializeBinaryFromReader(message: GetAlertStatisticsResponse, reader: jspb.BinaryReader): GetAlertStatisticsResponse;
}

export namespace GetAlertStatisticsResponse {
  export type AsObject = {
    status: boolean,
    data?: AlertStatistics.AsObject,
  }
}

export enum AlertType { 
  ALERT_TYPE_UNSPECIFIED = 0,
  AUTHENTICATION_FAILURE = 1,
  INJECTION_ATTEMPT = 2,
  RATE_LIMIT_EXCEEDED = 3,
  SUSPICIOUS_ACTIVITY = 4,
  PRIVILEGE_ESCALATION = 5,
  DATA_BREACH_ATTEMPT = 6,
  UNAUTHORIZED_ACCESS = 7,
  MALFORMED_REQUEST = 8,
  BRUTE_FORCE_ATTACK = 9,
  SESSION_HIJACKING = 10,
}
export enum AlertSeverity { 
  ALERT_SEVERITY_UNSPECIFIED = 0,
  LOW = 1,
  MEDIUM = 2,
  HIGH = 3,
  CRITICAL = 4,
}
export enum SecurityEventType { 
  SECURITY_EVENT_TYPE_UNSPECIFIED = 0,
  AUTHENTICATION_SUCCESS = 1,
  AUTHENTICATION_FAILURE_EVENT = 2,
  INJECTION_ATTEMPT_EVENT = 3,
  RATE_LIMIT_EXCEEDED_EVENT = 4,
  SUSPICIOUS_ACTIVITY_EVENT = 5,
  SECURITY_VIOLATION_EVENT = 6,
}

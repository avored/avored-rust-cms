/**
 * @fileoverview gRPC-Web generated client stub for admin_user
 * @enhanceable
 * @public
 */

// Code generated by protoc-gen-grpc-web. DO NOT EDIT.
// versions:
// 	protoc-gen-grpc-web v1.5.0
// 	protoc              v5.29.3
// source: admin_user.proto


/* eslint-disable */
// @ts-nocheck


import * as grpcWeb from 'grpc-web';

import * as admin_user_pb from './admin_user_pb'; // proto import: "admin_user.proto"


export class AdminUserClient {
  client_: grpcWeb.AbstractClientBase;
  hostname_: string;
  credentials_: null | { [index: string]: string; };
  options_: null | { [index: string]: any; };

  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; }) {
    if (!options) options = {};
    if (!credentials) credentials = {};
    options['format'] = 'binary';

    this.client_ = new grpcWeb.GrpcWebClientBase(options);
    this.hostname_ = hostname.replace(/\/+$/, '');
    this.credentials_ = credentials;
    this.options_ = options;
  }

  methodDescriptorPaginate = new grpcWeb.MethodDescriptor(
    '/admin_user.AdminUser/Paginate',
    grpcWeb.MethodType.UNARY,
    admin_user_pb.AdminUserPaginateRequest,
    admin_user_pb.AdminUserPaginateResponse,
    (request: admin_user_pb.AdminUserPaginateRequest) => {
      return request.serializeBinary();
    },
    admin_user_pb.AdminUserPaginateResponse.deserializeBinary
  );

  paginate(
    request: admin_user_pb.AdminUserPaginateRequest,
    metadata?: grpcWeb.Metadata | null): Promise<admin_user_pb.AdminUserPaginateResponse>;

  paginate(
    request: admin_user_pb.AdminUserPaginateRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.RpcError,
               response: admin_user_pb.AdminUserPaginateResponse) => void): grpcWeb.ClientReadableStream<admin_user_pb.AdminUserPaginateResponse>;

  paginate(
    request: admin_user_pb.AdminUserPaginateRequest,
    metadata?: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.RpcError,
               response: admin_user_pb.AdminUserPaginateResponse) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        this.hostname_ +
          '/admin_user.AdminUser/Paginate',
        request,
        metadata || {},
        this.methodDescriptorPaginate,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/admin_user.AdminUser/Paginate',
    request,
    metadata || {},
    this.methodDescriptorPaginate);
  }

  methodDescriptorStoreAdminUser = new grpcWeb.MethodDescriptor(
    '/admin_user.AdminUser/StoreAdminUser',
    grpcWeb.MethodType.UNARY,
    admin_user_pb.StoreAdminUserRequest,
    admin_user_pb.StoreAdminUserResponse,
    (request: admin_user_pb.StoreAdminUserRequest) => {
      return request.serializeBinary();
    },
    admin_user_pb.StoreAdminUserResponse.deserializeBinary
  );

  storeAdminUser(
    request: admin_user_pb.StoreAdminUserRequest,
    metadata?: grpcWeb.Metadata | null): Promise<admin_user_pb.StoreAdminUserResponse>;

  storeAdminUser(
    request: admin_user_pb.StoreAdminUserRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.RpcError,
               response: admin_user_pb.StoreAdminUserResponse) => void): grpcWeb.ClientReadableStream<admin_user_pb.StoreAdminUserResponse>;

  storeAdminUser(
    request: admin_user_pb.StoreAdminUserRequest,
    metadata?: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.RpcError,
               response: admin_user_pb.StoreAdminUserResponse) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        this.hostname_ +
          '/admin_user.AdminUser/StoreAdminUser',
        request,
        metadata || {},
        this.methodDescriptorStoreAdminUser,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/admin_user.AdminUser/StoreAdminUser',
    request,
    metadata || {},
    this.methodDescriptorStoreAdminUser);
  }

  methodDescriptorGetAdminUser = new grpcWeb.MethodDescriptor(
    '/admin_user.AdminUser/GetAdminUser',
    grpcWeb.MethodType.UNARY,
    admin_user_pb.GetAdminUserRequest,
    admin_user_pb.GetAdminUserResponse,
    (request: admin_user_pb.GetAdminUserRequest) => {
      return request.serializeBinary();
    },
    admin_user_pb.GetAdminUserResponse.deserializeBinary
  );

  getAdminUser(
    request: admin_user_pb.GetAdminUserRequest,
    metadata?: grpcWeb.Metadata | null): Promise<admin_user_pb.GetAdminUserResponse>;

  getAdminUser(
    request: admin_user_pb.GetAdminUserRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.RpcError,
               response: admin_user_pb.GetAdminUserResponse) => void): grpcWeb.ClientReadableStream<admin_user_pb.GetAdminUserResponse>;

  getAdminUser(
    request: admin_user_pb.GetAdminUserRequest,
    metadata?: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.RpcError,
               response: admin_user_pb.GetAdminUserResponse) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        this.hostname_ +
          '/admin_user.AdminUser/GetAdminUser',
        request,
        metadata || {},
        this.methodDescriptorGetAdminUser,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/admin_user.AdminUser/GetAdminUser',
    request,
    metadata || {},
    this.methodDescriptorGetAdminUser);
  }

  methodDescriptorUpdateAdminUser = new grpcWeb.MethodDescriptor(
    '/admin_user.AdminUser/UpdateAdminUser',
    grpcWeb.MethodType.UNARY,
    admin_user_pb.UpdateAdminUserRequest,
    admin_user_pb.UpdateAdminUserResponse,
    (request: admin_user_pb.UpdateAdminUserRequest) => {
      return request.serializeBinary();
    },
    admin_user_pb.UpdateAdminUserResponse.deserializeBinary
  );

  updateAdminUser(
    request: admin_user_pb.UpdateAdminUserRequest,
    metadata?: grpcWeb.Metadata | null): Promise<admin_user_pb.UpdateAdminUserResponse>;

  updateAdminUser(
    request: admin_user_pb.UpdateAdminUserRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.RpcError,
               response: admin_user_pb.UpdateAdminUserResponse) => void): grpcWeb.ClientReadableStream<admin_user_pb.UpdateAdminUserResponse>;

  updateAdminUser(
    request: admin_user_pb.UpdateAdminUserRequest,
    metadata?: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.RpcError,
               response: admin_user_pb.UpdateAdminUserResponse) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        this.hostname_ +
          '/admin_user.AdminUser/UpdateAdminUser',
        request,
        metadata || {},
        this.methodDescriptorUpdateAdminUser,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/admin_user.AdminUser/UpdateAdminUser',
    request,
    metadata || {},
    this.methodDescriptorUpdateAdminUser);
  }

}


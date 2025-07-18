/**
 * @fileoverview gRPC-Web generated client stub for cms
 * @enhanceable
 * @public
 */

// Code generated by protoc-gen-grpc-web. DO NOT EDIT.
// versions:
// 	protoc-gen-grpc-web v1.5.0
// 	protoc              v5.29.3
// source: cms.proto


/* eslint-disable */
// @ts-nocheck


import * as grpcWeb from 'grpc-web';

import * as cms_pb from './cms_pb'; // proto import: "cms.proto"


export class CmsClient {
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

  methodDescriptorGetCmsContent = new grpcWeb.MethodDescriptor(
    '/cms.Cms/GetCmsContent',
    grpcWeb.MethodType.UNARY,
    cms_pb.GetCmsContentRequest,
    cms_pb.GetCmsContentResponse,
    (request: cms_pb.GetCmsContentRequest) => {
      return request.serializeBinary();
    },
    cms_pb.GetCmsContentResponse.deserializeBinary
  );

  getCmsContent(
    request: cms_pb.GetCmsContentRequest,
    metadata?: grpcWeb.Metadata | null): Promise<cms_pb.GetCmsContentResponse>;

  getCmsContent(
    request: cms_pb.GetCmsContentRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.RpcError,
               response: cms_pb.GetCmsContentResponse) => void): grpcWeb.ClientReadableStream<cms_pb.GetCmsContentResponse>;

  getCmsContent(
    request: cms_pb.GetCmsContentRequest,
    metadata?: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.RpcError,
               response: cms_pb.GetCmsContentResponse) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        this.hostname_ +
          '/cms.Cms/GetCmsContent',
        request,
        metadata || {},
        this.methodDescriptorGetCmsContent,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/cms.Cms/GetCmsContent',
    request,
    metadata || {},
    this.methodDescriptorGetCmsContent);
  }

  methodDescriptorSentContactForm = new grpcWeb.MethodDescriptor(
    '/cms.Cms/SentContactForm',
    grpcWeb.MethodType.UNARY,
    cms_pb.SentContactFormRequest,
    cms_pb.SentContactFormResponse,
    (request: cms_pb.SentContactFormRequest) => {
      return request.serializeBinary();
    },
    cms_pb.SentContactFormResponse.deserializeBinary
  );

  sentContactForm(
    request: cms_pb.SentContactFormRequest,
    metadata?: grpcWeb.Metadata | null): Promise<cms_pb.SentContactFormResponse>;

  sentContactForm(
    request: cms_pb.SentContactFormRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.RpcError,
               response: cms_pb.SentContactFormResponse) => void): grpcWeb.ClientReadableStream<cms_pb.SentContactFormResponse>;

  sentContactForm(
    request: cms_pb.SentContactFormRequest,
    metadata?: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.RpcError,
               response: cms_pb.SentContactFormResponse) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        this.hostname_ +
          '/cms.Cms/SentContactForm',
        request,
        metadata || {},
        this.methodDescriptorSentContactForm,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/cms.Cms/SentContactForm',
    request,
    metadata || {},
    this.methodDescriptorSentContactForm);
  }

}


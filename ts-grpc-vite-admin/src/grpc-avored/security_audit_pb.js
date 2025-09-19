// source: security_audit.proto
/**
 * @fileoverview
 * @enhanceable
 * @suppress {missingRequire} reports error on implicit type usages.
 * @suppress {messageConventions} JS Compiler reports an error if a variable or
 *     field starts with 'MSG_' and isn't a translatable message.
 * @public
 */
// GENERATED CODE -- DO NOT EDIT!
/* eslint-disable */
// @ts-nocheck

var jspb = require('google-protobuf');
var goog = jspb;
var global =
    (typeof globalThis !== 'undefined' && globalThis) ||
    (typeof window !== 'undefined' && window) ||
    (typeof global !== 'undefined' && global) ||
    (typeof self !== 'undefined' && self) ||
    (function () { return this; }).call(null) ||
    Function('return this')();

var google_protobuf_timestamp_pb = require('google-protobuf/google/protobuf/timestamp_pb.js');
goog.object.extend(proto, google_protobuf_timestamp_pb);
goog.exportSymbol('proto.security_audit.AlertSeverity', null, global);
goog.exportSymbol('proto.security_audit.AlertStatistics', null, global);
goog.exportSymbol('proto.security_audit.AlertType', null, global);
goog.exportSymbol('proto.security_audit.CreateSecurityAlertAutoIdRequest', null, global);
goog.exportSymbol('proto.security_audit.CreateSecurityAlertAutoIdResponse', null, global);
goog.exportSymbol('proto.security_audit.CreateSecurityAlertModel', null, global);
goog.exportSymbol('proto.security_audit.CreateSecurityAlertRequest', null, global);
goog.exportSymbol('proto.security_audit.CreateSecurityAlertResponse', null, global);
goog.exportSymbol('proto.security_audit.CreateSecurityAuditModel', null, global);
goog.exportSymbol('proto.security_audit.CreateSecurityAuditRequest', null, global);
goog.exportSymbol('proto.security_audit.CreateSecurityAuditResponse', null, global);
goog.exportSymbol('proto.security_audit.DeleteSecurityAlertRequest', null, global);
goog.exportSymbol('proto.security_audit.DeleteSecurityAlertResponse', null, global);
goog.exportSymbol('proto.security_audit.DeleteSecurityAuditRequest', null, global);
goog.exportSymbol('proto.security_audit.DeleteSecurityAuditResponse', null, global);
goog.exportSymbol('proto.security_audit.GetAlertStatisticsRequest', null, global);
goog.exportSymbol('proto.security_audit.GetAlertStatisticsResponse', null, global);
goog.exportSymbol('proto.security_audit.GetAlertsBySourceRequest', null, global);
goog.exportSymbol('proto.security_audit.GetAlertsBySourceResponse', null, global);
goog.exportSymbol('proto.security_audit.GetAlertsByTypeRequest', null, global);
goog.exportSymbol('proto.security_audit.GetAlertsByTypeResponse', null, global);
goog.exportSymbol('proto.security_audit.GetCriticalUnresolvedAlertsRequest', null, global);
goog.exportSymbol('proto.security_audit.GetCriticalUnresolvedAlertsResponse', null, global);
goog.exportSymbol('proto.security_audit.GetIpSecuritySummaryRequest', null, global);
goog.exportSymbol('proto.security_audit.GetIpSecuritySummaryResponse', null, global);
goog.exportSymbol('proto.security_audit.GetSecurityAlertRequest', null, global);
goog.exportSymbol('proto.security_audit.GetSecurityAlertResponse', null, global);
goog.exportSymbol('proto.security_audit.GetSecurityAlertsPaginatedRequest', null, global);
goog.exportSymbol('proto.security_audit.GetSecurityAlertsPaginatedResponse', null, global);
goog.exportSymbol('proto.security_audit.GetSecurityAuditRequest', null, global);
goog.exportSymbol('proto.security_audit.GetSecurityAuditResponse', null, global);
goog.exportSymbol('proto.security_audit.GetSecurityAuditsByIpRequest', null, global);
goog.exportSymbol('proto.security_audit.GetSecurityAuditsByIpResponse', null, global);
goog.exportSymbol('proto.security_audit.GetSecurityAuditsByUserRequest', null, global);
goog.exportSymbol('proto.security_audit.GetSecurityAuditsByUserResponse', null, global);
goog.exportSymbol('proto.security_audit.GetSecurityAuditsPaginatedRequest', null, global);
goog.exportSymbol('proto.security_audit.GetSecurityAuditsPaginatedResponse', null, global);
goog.exportSymbol('proto.security_audit.GetUnresolvedAlertsBySeverityRequest', null, global);
goog.exportSymbol('proto.security_audit.GetUnresolvedAlertsBySeverityResponse', null, global);
goog.exportSymbol('proto.security_audit.LogSecurityEventRequest', null, global);
goog.exportSymbol('proto.security_audit.LogSecurityEventResponse', null, global);
goog.exportSymbol('proto.security_audit.Pagination', null, global);
goog.exportSymbol('proto.security_audit.ResolveSecurityAlertRequest', null, global);
goog.exportSymbol('proto.security_audit.ResolveSecurityAlertResponse', null, global);
goog.exportSymbol('proto.security_audit.SecurityAlertModel', null, global);
goog.exportSymbol('proto.security_audit.SecurityAlertPaginationModel', null, global);
goog.exportSymbol('proto.security_audit.SecurityAuditModel', null, global);
goog.exportSymbol('proto.security_audit.SecurityAuditPaginationModel', null, global);
goog.exportSymbol('proto.security_audit.SecurityEventType', null, global);
goog.exportSymbol('proto.security_audit.SecuritySummary', null, global);
goog.exportSymbol('proto.security_audit.UpdateSecurityAlertModel', null, global);
goog.exportSymbol('proto.security_audit.UpdateSecurityAuditModel', null, global);
goog.exportSymbol('proto.security_audit.UpdateSecurityAuditRequest', null, global);
goog.exportSymbol('proto.security_audit.UpdateSecurityAuditResponse', null, global);
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.SecurityAuditModel = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.SecurityAuditModel, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.SecurityAuditModel.displayName = 'proto.security_audit.SecurityAuditModel';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.CreateSecurityAuditModel = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.CreateSecurityAuditModel, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.CreateSecurityAuditModel.displayName = 'proto.security_audit.CreateSecurityAuditModel';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.UpdateSecurityAuditModel = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.UpdateSecurityAuditModel, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.UpdateSecurityAuditModel.displayName = 'proto.security_audit.UpdateSecurityAuditModel';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.SecurityAlertModel = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.SecurityAlertModel, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.SecurityAlertModel.displayName = 'proto.security_audit.SecurityAlertModel';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.CreateSecurityAlertModel = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.CreateSecurityAlertModel, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.CreateSecurityAlertModel.displayName = 'proto.security_audit.CreateSecurityAlertModel';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.UpdateSecurityAlertModel = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.UpdateSecurityAlertModel, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.UpdateSecurityAlertModel.displayName = 'proto.security_audit.UpdateSecurityAlertModel';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.Pagination = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.Pagination, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.Pagination.displayName = 'proto.security_audit.Pagination';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.SecurityAuditPaginationModel = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.security_audit.SecurityAuditPaginationModel.repeatedFields_, null);
};
goog.inherits(proto.security_audit.SecurityAuditPaginationModel, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.SecurityAuditPaginationModel.displayName = 'proto.security_audit.SecurityAuditPaginationModel';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.SecurityAlertPaginationModel = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.security_audit.SecurityAlertPaginationModel.repeatedFields_, null);
};
goog.inherits(proto.security_audit.SecurityAlertPaginationModel, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.SecurityAlertPaginationModel.displayName = 'proto.security_audit.SecurityAlertPaginationModel';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.SecuritySummary = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.SecuritySummary, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.SecuritySummary.displayName = 'proto.security_audit.SecuritySummary';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.AlertStatistics = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.AlertStatistics, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.AlertStatistics.displayName = 'proto.security_audit.AlertStatistics';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.CreateSecurityAuditRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.CreateSecurityAuditRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.CreateSecurityAuditRequest.displayName = 'proto.security_audit.CreateSecurityAuditRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.CreateSecurityAuditResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.CreateSecurityAuditResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.CreateSecurityAuditResponse.displayName = 'proto.security_audit.CreateSecurityAuditResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.LogSecurityEventRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.LogSecurityEventRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.LogSecurityEventRequest.displayName = 'proto.security_audit.LogSecurityEventRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.LogSecurityEventResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.LogSecurityEventResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.LogSecurityEventResponse.displayName = 'proto.security_audit.LogSecurityEventResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetSecurityAuditRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetSecurityAuditRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetSecurityAuditRequest.displayName = 'proto.security_audit.GetSecurityAuditRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetSecurityAuditResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetSecurityAuditResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetSecurityAuditResponse.displayName = 'proto.security_audit.GetSecurityAuditResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetSecurityAuditsByUserRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetSecurityAuditsByUserRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetSecurityAuditsByUserRequest.displayName = 'proto.security_audit.GetSecurityAuditsByUserRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetSecurityAuditsByUserResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetSecurityAuditsByUserResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetSecurityAuditsByUserResponse.displayName = 'proto.security_audit.GetSecurityAuditsByUserResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetSecurityAuditsByIpRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetSecurityAuditsByIpRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetSecurityAuditsByIpRequest.displayName = 'proto.security_audit.GetSecurityAuditsByIpRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetSecurityAuditsByIpResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetSecurityAuditsByIpResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetSecurityAuditsByIpResponse.displayName = 'proto.security_audit.GetSecurityAuditsByIpResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetSecurityAuditsPaginatedRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetSecurityAuditsPaginatedRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetSecurityAuditsPaginatedRequest.displayName = 'proto.security_audit.GetSecurityAuditsPaginatedRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetSecurityAuditsPaginatedResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetSecurityAuditsPaginatedResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetSecurityAuditsPaginatedResponse.displayName = 'proto.security_audit.GetSecurityAuditsPaginatedResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.UpdateSecurityAuditRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.UpdateSecurityAuditRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.UpdateSecurityAuditRequest.displayName = 'proto.security_audit.UpdateSecurityAuditRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.UpdateSecurityAuditResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.UpdateSecurityAuditResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.UpdateSecurityAuditResponse.displayName = 'proto.security_audit.UpdateSecurityAuditResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.DeleteSecurityAuditRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.DeleteSecurityAuditRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.DeleteSecurityAuditRequest.displayName = 'proto.security_audit.DeleteSecurityAuditRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.DeleteSecurityAuditResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.DeleteSecurityAuditResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.DeleteSecurityAuditResponse.displayName = 'proto.security_audit.DeleteSecurityAuditResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetIpSecuritySummaryRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetIpSecuritySummaryRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetIpSecuritySummaryRequest.displayName = 'proto.security_audit.GetIpSecuritySummaryRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetIpSecuritySummaryResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetIpSecuritySummaryResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetIpSecuritySummaryResponse.displayName = 'proto.security_audit.GetIpSecuritySummaryResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.CreateSecurityAlertRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.CreateSecurityAlertRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.CreateSecurityAlertRequest.displayName = 'proto.security_audit.CreateSecurityAlertRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.CreateSecurityAlertResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.CreateSecurityAlertResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.CreateSecurityAlertResponse.displayName = 'proto.security_audit.CreateSecurityAlertResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.CreateSecurityAlertAutoIdRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.CreateSecurityAlertAutoIdRequest.displayName = 'proto.security_audit.CreateSecurityAlertAutoIdRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.CreateSecurityAlertAutoIdResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.CreateSecurityAlertAutoIdResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.CreateSecurityAlertAutoIdResponse.displayName = 'proto.security_audit.CreateSecurityAlertAutoIdResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetSecurityAlertRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetSecurityAlertRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetSecurityAlertRequest.displayName = 'proto.security_audit.GetSecurityAlertRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetSecurityAlertResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetSecurityAlertResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetSecurityAlertResponse.displayName = 'proto.security_audit.GetSecurityAlertResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetUnresolvedAlertsBySeverityRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetUnresolvedAlertsBySeverityRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetUnresolvedAlertsBySeverityRequest.displayName = 'proto.security_audit.GetUnresolvedAlertsBySeverityRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetUnresolvedAlertsBySeverityResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetUnresolvedAlertsBySeverityResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetUnresolvedAlertsBySeverityResponse.displayName = 'proto.security_audit.GetUnresolvedAlertsBySeverityResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetAlertsByTypeRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetAlertsByTypeRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetAlertsByTypeRequest.displayName = 'proto.security_audit.GetAlertsByTypeRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetAlertsByTypeResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetAlertsByTypeResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetAlertsByTypeResponse.displayName = 'proto.security_audit.GetAlertsByTypeResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetAlertsBySourceRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetAlertsBySourceRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetAlertsBySourceRequest.displayName = 'proto.security_audit.GetAlertsBySourceRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetAlertsBySourceResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetAlertsBySourceResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetAlertsBySourceResponse.displayName = 'proto.security_audit.GetAlertsBySourceResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.ResolveSecurityAlertRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.ResolveSecurityAlertRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.ResolveSecurityAlertRequest.displayName = 'proto.security_audit.ResolveSecurityAlertRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.ResolveSecurityAlertResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.ResolveSecurityAlertResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.ResolveSecurityAlertResponse.displayName = 'proto.security_audit.ResolveSecurityAlertResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetSecurityAlertsPaginatedRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetSecurityAlertsPaginatedRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetSecurityAlertsPaginatedRequest.displayName = 'proto.security_audit.GetSecurityAlertsPaginatedRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetSecurityAlertsPaginatedResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetSecurityAlertsPaginatedResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetSecurityAlertsPaginatedResponse.displayName = 'proto.security_audit.GetSecurityAlertsPaginatedResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetCriticalUnresolvedAlertsRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetCriticalUnresolvedAlertsRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetCriticalUnresolvedAlertsRequest.displayName = 'proto.security_audit.GetCriticalUnresolvedAlertsRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetCriticalUnresolvedAlertsResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, proto.security_audit.GetCriticalUnresolvedAlertsResponse.repeatedFields_, null);
};
goog.inherits(proto.security_audit.GetCriticalUnresolvedAlertsResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetCriticalUnresolvedAlertsResponse.displayName = 'proto.security_audit.GetCriticalUnresolvedAlertsResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.DeleteSecurityAlertRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.DeleteSecurityAlertRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.DeleteSecurityAlertRequest.displayName = 'proto.security_audit.DeleteSecurityAlertRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.DeleteSecurityAlertResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.DeleteSecurityAlertResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.DeleteSecurityAlertResponse.displayName = 'proto.security_audit.DeleteSecurityAlertResponse';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetAlertStatisticsRequest = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetAlertStatisticsRequest, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetAlertStatisticsRequest.displayName = 'proto.security_audit.GetAlertStatisticsRequest';
}
/**
 * Generated by JsPbCodeGenerator.
 * @param {Array=} opt_data Optional initial data array, typically from a
 * server response, or constructed directly in Javascript. The array is used
 * in place and becomes part of the constructed object. It is not cloned.
 * If no data is provided, the constructed object will be empty, but still
 * valid.
 * @extends {jspb.Message}
 * @constructor
 */
proto.security_audit.GetAlertStatisticsResponse = function(opt_data) {
  jspb.Message.initialize(this, opt_data, 0, -1, null, null);
};
goog.inherits(proto.security_audit.GetAlertStatisticsResponse, jspb.Message);
if (goog.DEBUG && !COMPILED) {
  /**
   * @public
   * @override
   */
  proto.security_audit.GetAlertStatisticsResponse.displayName = 'proto.security_audit.GetAlertStatisticsResponse';
}



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.SecurityAuditModel.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.SecurityAuditModel.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.SecurityAuditModel} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.SecurityAuditModel.toObject = function(includeInstance, msg) {
  var f, obj = {
id: jspb.Message.getFieldWithDefault(msg, 1, ""),
securityAuditId: jspb.Message.getFieldWithDefault(msg, 2, ""),
adminUserId: (f = jspb.Message.getField(msg, 3)) == null ? undefined : f,
sessionId: (f = jspb.Message.getField(msg, 4)) == null ? undefined : f,
ipAddress: jspb.Message.getFieldWithDefault(msg, 5, ""),
userAgent: (f = jspb.Message.getField(msg, 6)) == null ? undefined : f,
endpoint: (f = jspb.Message.getField(msg, 7)) == null ? undefined : f,
requestMethod: (f = jspb.Message.getField(msg, 8)) == null ? undefined : f,
totalAuthenticationAttempts: jspb.Message.getFieldWithDefault(msg, 9, 0),
failedAuthenticationAttempts: jspb.Message.getFieldWithDefault(msg, 10, 0),
blockedInjectionAttempts: jspb.Message.getFieldWithDefault(msg, 11, 0),
rateLimitedRequests: jspb.Message.getFieldWithDefault(msg, 12, 0),
suspiciousActivitiesDetected: jspb.Message.getFieldWithDefault(msg, 13, 0),
securityViolations: jspb.Message.getFieldWithDefault(msg, 14, 0),
uptimeSeconds: jspb.Message.getFieldWithDefault(msg, 15, 0),
securityHealthScore: jspb.Message.getFloatingPointFieldWithDefault(msg, 16, 0.0),
createdAt: (f = msg.getCreatedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f),
updatedAt: (f = msg.getUpdatedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f),
metadataJson: (f = jspb.Message.getField(msg, 19)) == null ? undefined : f
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.SecurityAuditModel}
 */
proto.security_audit.SecurityAuditModel.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.SecurityAuditModel;
  return proto.security_audit.SecurityAuditModel.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.SecurityAuditModel} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.SecurityAuditModel}
 */
proto.security_audit.SecurityAuditModel.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setSecurityAuditId(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setAdminUserId(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setSessionId(value);
      break;
    case 5:
      var value = /** @type {string} */ (reader.readString());
      msg.setIpAddress(value);
      break;
    case 6:
      var value = /** @type {string} */ (reader.readString());
      msg.setUserAgent(value);
      break;
    case 7:
      var value = /** @type {string} */ (reader.readString());
      msg.setEndpoint(value);
      break;
    case 8:
      var value = /** @type {string} */ (reader.readString());
      msg.setRequestMethod(value);
      break;
    case 9:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setTotalAuthenticationAttempts(value);
      break;
    case 10:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setFailedAuthenticationAttempts(value);
      break;
    case 11:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setBlockedInjectionAttempts(value);
      break;
    case 12:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setRateLimitedRequests(value);
      break;
    case 13:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setSuspiciousActivitiesDetected(value);
      break;
    case 14:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setSecurityViolations(value);
      break;
    case 15:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setUptimeSeconds(value);
      break;
    case 16:
      var value = /** @type {number} */ (reader.readDouble());
      msg.setSecurityHealthScore(value);
      break;
    case 17:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setCreatedAt(value);
      break;
    case 18:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setUpdatedAt(value);
      break;
    case 19:
      var value = /** @type {string} */ (reader.readString());
      msg.setMetadataJson(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.SecurityAuditModel.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.SecurityAuditModel.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.SecurityAuditModel} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.SecurityAuditModel.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getSecurityAuditId();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 3));
  if (f != null) {
    writer.writeString(
      3,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 4));
  if (f != null) {
    writer.writeString(
      4,
      f
    );
  }
  f = message.getIpAddress();
  if (f.length > 0) {
    writer.writeString(
      5,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 6));
  if (f != null) {
    writer.writeString(
      6,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 7));
  if (f != null) {
    writer.writeString(
      7,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 8));
  if (f != null) {
    writer.writeString(
      8,
      f
    );
  }
  f = message.getTotalAuthenticationAttempts();
  if (f !== 0) {
    writer.writeInt32(
      9,
      f
    );
  }
  f = message.getFailedAuthenticationAttempts();
  if (f !== 0) {
    writer.writeInt32(
      10,
      f
    );
  }
  f = message.getBlockedInjectionAttempts();
  if (f !== 0) {
    writer.writeInt32(
      11,
      f
    );
  }
  f = message.getRateLimitedRequests();
  if (f !== 0) {
    writer.writeInt32(
      12,
      f
    );
  }
  f = message.getSuspiciousActivitiesDetected();
  if (f !== 0) {
    writer.writeInt32(
      13,
      f
    );
  }
  f = message.getSecurityViolations();
  if (f !== 0) {
    writer.writeInt32(
      14,
      f
    );
  }
  f = message.getUptimeSeconds();
  if (f !== 0) {
    writer.writeInt32(
      15,
      f
    );
  }
  f = message.getSecurityHealthScore();
  if (f !== 0.0) {
    writer.writeDouble(
      16,
      f
    );
  }
  f = message.getCreatedAt();
  if (f != null) {
    writer.writeMessage(
      17,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
  f = message.getUpdatedAt();
  if (f != null) {
    writer.writeMessage(
      18,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 19));
  if (f != null) {
    writer.writeString(
      19,
      f
    );
  }
};


/**
 * optional string id = 1;
 * @return {string}
 */
proto.security_audit.SecurityAuditModel.prototype.getId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.setId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string security_audit_id = 2;
 * @return {string}
 */
proto.security_audit.SecurityAuditModel.prototype.getSecurityAuditId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.setSecurityAuditId = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional string admin_user_id = 3;
 * @return {string}
 */
proto.security_audit.SecurityAuditModel.prototype.getAdminUserId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.setAdminUserId = function(value) {
  return jspb.Message.setField(this, 3, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.clearAdminUserId = function() {
  return jspb.Message.setField(this, 3, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.SecurityAuditModel.prototype.hasAdminUserId = function() {
  return jspb.Message.getField(this, 3) != null;
};


/**
 * optional string session_id = 4;
 * @return {string}
 */
proto.security_audit.SecurityAuditModel.prototype.getSessionId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.setSessionId = function(value) {
  return jspb.Message.setField(this, 4, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.clearSessionId = function() {
  return jspb.Message.setField(this, 4, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.SecurityAuditModel.prototype.hasSessionId = function() {
  return jspb.Message.getField(this, 4) != null;
};


/**
 * optional string ip_address = 5;
 * @return {string}
 */
proto.security_audit.SecurityAuditModel.prototype.getIpAddress = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 5, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.setIpAddress = function(value) {
  return jspb.Message.setProto3StringField(this, 5, value);
};


/**
 * optional string user_agent = 6;
 * @return {string}
 */
proto.security_audit.SecurityAuditModel.prototype.getUserAgent = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 6, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.setUserAgent = function(value) {
  return jspb.Message.setField(this, 6, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.clearUserAgent = function() {
  return jspb.Message.setField(this, 6, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.SecurityAuditModel.prototype.hasUserAgent = function() {
  return jspb.Message.getField(this, 6) != null;
};


/**
 * optional string endpoint = 7;
 * @return {string}
 */
proto.security_audit.SecurityAuditModel.prototype.getEndpoint = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 7, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.setEndpoint = function(value) {
  return jspb.Message.setField(this, 7, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.clearEndpoint = function() {
  return jspb.Message.setField(this, 7, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.SecurityAuditModel.prototype.hasEndpoint = function() {
  return jspb.Message.getField(this, 7) != null;
};


/**
 * optional string request_method = 8;
 * @return {string}
 */
proto.security_audit.SecurityAuditModel.prototype.getRequestMethod = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 8, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.setRequestMethod = function(value) {
  return jspb.Message.setField(this, 8, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.clearRequestMethod = function() {
  return jspb.Message.setField(this, 8, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.SecurityAuditModel.prototype.hasRequestMethod = function() {
  return jspb.Message.getField(this, 8) != null;
};


/**
 * optional int32 total_authentication_attempts = 9;
 * @return {number}
 */
proto.security_audit.SecurityAuditModel.prototype.getTotalAuthenticationAttempts = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 9, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.setTotalAuthenticationAttempts = function(value) {
  return jspb.Message.setProto3IntField(this, 9, value);
};


/**
 * optional int32 failed_authentication_attempts = 10;
 * @return {number}
 */
proto.security_audit.SecurityAuditModel.prototype.getFailedAuthenticationAttempts = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 10, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.setFailedAuthenticationAttempts = function(value) {
  return jspb.Message.setProto3IntField(this, 10, value);
};


/**
 * optional int32 blocked_injection_attempts = 11;
 * @return {number}
 */
proto.security_audit.SecurityAuditModel.prototype.getBlockedInjectionAttempts = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 11, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.setBlockedInjectionAttempts = function(value) {
  return jspb.Message.setProto3IntField(this, 11, value);
};


/**
 * optional int32 rate_limited_requests = 12;
 * @return {number}
 */
proto.security_audit.SecurityAuditModel.prototype.getRateLimitedRequests = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 12, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.setRateLimitedRequests = function(value) {
  return jspb.Message.setProto3IntField(this, 12, value);
};


/**
 * optional int32 suspicious_activities_detected = 13;
 * @return {number}
 */
proto.security_audit.SecurityAuditModel.prototype.getSuspiciousActivitiesDetected = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 13, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.setSuspiciousActivitiesDetected = function(value) {
  return jspb.Message.setProto3IntField(this, 13, value);
};


/**
 * optional int32 security_violations = 14;
 * @return {number}
 */
proto.security_audit.SecurityAuditModel.prototype.getSecurityViolations = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 14, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.setSecurityViolations = function(value) {
  return jspb.Message.setProto3IntField(this, 14, value);
};


/**
 * optional int32 uptime_seconds = 15;
 * @return {number}
 */
proto.security_audit.SecurityAuditModel.prototype.getUptimeSeconds = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 15, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.setUptimeSeconds = function(value) {
  return jspb.Message.setProto3IntField(this, 15, value);
};


/**
 * optional double security_health_score = 16;
 * @return {number}
 */
proto.security_audit.SecurityAuditModel.prototype.getSecurityHealthScore = function() {
  return /** @type {number} */ (jspb.Message.getFloatingPointFieldWithDefault(this, 16, 0.0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.setSecurityHealthScore = function(value) {
  return jspb.Message.setProto3FloatField(this, 16, value);
};


/**
 * optional google.protobuf.Timestamp created_at = 17;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.security_audit.SecurityAuditModel.prototype.getCreatedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 17));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.security_audit.SecurityAuditModel} returns this
*/
proto.security_audit.SecurityAuditModel.prototype.setCreatedAt = function(value) {
  return jspb.Message.setWrapperField(this, 17, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.clearCreatedAt = function() {
  return this.setCreatedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.SecurityAuditModel.prototype.hasCreatedAt = function() {
  return jspb.Message.getField(this, 17) != null;
};


/**
 * optional google.protobuf.Timestamp updated_at = 18;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.security_audit.SecurityAuditModel.prototype.getUpdatedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 18));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.security_audit.SecurityAuditModel} returns this
*/
proto.security_audit.SecurityAuditModel.prototype.setUpdatedAt = function(value) {
  return jspb.Message.setWrapperField(this, 18, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.clearUpdatedAt = function() {
  return this.setUpdatedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.SecurityAuditModel.prototype.hasUpdatedAt = function() {
  return jspb.Message.getField(this, 18) != null;
};


/**
 * optional string metadata_json = 19;
 * @return {string}
 */
proto.security_audit.SecurityAuditModel.prototype.getMetadataJson = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 19, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.setMetadataJson = function(value) {
  return jspb.Message.setField(this, 19, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.SecurityAuditModel} returns this
 */
proto.security_audit.SecurityAuditModel.prototype.clearMetadataJson = function() {
  return jspb.Message.setField(this, 19, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.SecurityAuditModel.prototype.hasMetadataJson = function() {
  return jspb.Message.getField(this, 19) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.CreateSecurityAuditModel.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.CreateSecurityAuditModel} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.CreateSecurityAuditModel.toObject = function(includeInstance, msg) {
  var f, obj = {
securityAuditId: jspb.Message.getFieldWithDefault(msg, 1, ""),
adminUserId: (f = jspb.Message.getField(msg, 2)) == null ? undefined : f,
sessionId: (f = jspb.Message.getField(msg, 3)) == null ? undefined : f,
ipAddress: jspb.Message.getFieldWithDefault(msg, 4, ""),
userAgent: (f = jspb.Message.getField(msg, 5)) == null ? undefined : f,
endpoint: (f = jspb.Message.getField(msg, 6)) == null ? undefined : f,
requestMethod: (f = jspb.Message.getField(msg, 7)) == null ? undefined : f,
totalAuthenticationAttempts: (f = jspb.Message.getField(msg, 8)) == null ? undefined : f,
failedAuthenticationAttempts: (f = jspb.Message.getField(msg, 9)) == null ? undefined : f,
blockedInjectionAttempts: (f = jspb.Message.getField(msg, 10)) == null ? undefined : f,
rateLimitedRequests: (f = jspb.Message.getField(msg, 11)) == null ? undefined : f,
suspiciousActivitiesDetected: (f = jspb.Message.getField(msg, 12)) == null ? undefined : f,
securityViolations: (f = jspb.Message.getField(msg, 13)) == null ? undefined : f,
uptimeSeconds: (f = jspb.Message.getField(msg, 14)) == null ? undefined : f,
securityHealthScore: (f = jspb.Message.getOptionalFloatingPointField(msg, 15)) == null ? undefined : f,
metadataJson: (f = jspb.Message.getField(msg, 16)) == null ? undefined : f
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.CreateSecurityAuditModel}
 */
proto.security_audit.CreateSecurityAuditModel.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.CreateSecurityAuditModel;
  return proto.security_audit.CreateSecurityAuditModel.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.CreateSecurityAuditModel} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.CreateSecurityAuditModel}
 */
proto.security_audit.CreateSecurityAuditModel.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setSecurityAuditId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setAdminUserId(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setSessionId(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setIpAddress(value);
      break;
    case 5:
      var value = /** @type {string} */ (reader.readString());
      msg.setUserAgent(value);
      break;
    case 6:
      var value = /** @type {string} */ (reader.readString());
      msg.setEndpoint(value);
      break;
    case 7:
      var value = /** @type {string} */ (reader.readString());
      msg.setRequestMethod(value);
      break;
    case 8:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setTotalAuthenticationAttempts(value);
      break;
    case 9:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setFailedAuthenticationAttempts(value);
      break;
    case 10:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setBlockedInjectionAttempts(value);
      break;
    case 11:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setRateLimitedRequests(value);
      break;
    case 12:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setSuspiciousActivitiesDetected(value);
      break;
    case 13:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setSecurityViolations(value);
      break;
    case 14:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setUptimeSeconds(value);
      break;
    case 15:
      var value = /** @type {number} */ (reader.readDouble());
      msg.setSecurityHealthScore(value);
      break;
    case 16:
      var value = /** @type {string} */ (reader.readString());
      msg.setMetadataJson(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.CreateSecurityAuditModel.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.CreateSecurityAuditModel} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.CreateSecurityAuditModel.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getSecurityAuditId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 2));
  if (f != null) {
    writer.writeString(
      2,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 3));
  if (f != null) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getIpAddress();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 5));
  if (f != null) {
    writer.writeString(
      5,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 6));
  if (f != null) {
    writer.writeString(
      6,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 7));
  if (f != null) {
    writer.writeString(
      7,
      f
    );
  }
  f = /** @type {number} */ (jspb.Message.getField(message, 8));
  if (f != null) {
    writer.writeInt32(
      8,
      f
    );
  }
  f = /** @type {number} */ (jspb.Message.getField(message, 9));
  if (f != null) {
    writer.writeInt32(
      9,
      f
    );
  }
  f = /** @type {number} */ (jspb.Message.getField(message, 10));
  if (f != null) {
    writer.writeInt32(
      10,
      f
    );
  }
  f = /** @type {number} */ (jspb.Message.getField(message, 11));
  if (f != null) {
    writer.writeInt32(
      11,
      f
    );
  }
  f = /** @type {number} */ (jspb.Message.getField(message, 12));
  if (f != null) {
    writer.writeInt32(
      12,
      f
    );
  }
  f = /** @type {number} */ (jspb.Message.getField(message, 13));
  if (f != null) {
    writer.writeInt32(
      13,
      f
    );
  }
  f = /** @type {number} */ (jspb.Message.getField(message, 14));
  if (f != null) {
    writer.writeInt32(
      14,
      f
    );
  }
  f = /** @type {number} */ (jspb.Message.getField(message, 15));
  if (f != null) {
    writer.writeDouble(
      15,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 16));
  if (f != null) {
    writer.writeString(
      16,
      f
    );
  }
};


/**
 * optional string security_audit_id = 1;
 * @return {string}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.getSecurityAuditId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.setSecurityAuditId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string admin_user_id = 2;
 * @return {string}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.getAdminUserId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.setAdminUserId = function(value) {
  return jspb.Message.setField(this, 2, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.clearAdminUserId = function() {
  return jspb.Message.setField(this, 2, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.hasAdminUserId = function() {
  return jspb.Message.getField(this, 2) != null;
};


/**
 * optional string session_id = 3;
 * @return {string}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.getSessionId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.setSessionId = function(value) {
  return jspb.Message.setField(this, 3, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.clearSessionId = function() {
  return jspb.Message.setField(this, 3, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.hasSessionId = function() {
  return jspb.Message.getField(this, 3) != null;
};


/**
 * optional string ip_address = 4;
 * @return {string}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.getIpAddress = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.setIpAddress = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional string user_agent = 5;
 * @return {string}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.getUserAgent = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 5, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.setUserAgent = function(value) {
  return jspb.Message.setField(this, 5, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.clearUserAgent = function() {
  return jspb.Message.setField(this, 5, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.hasUserAgent = function() {
  return jspb.Message.getField(this, 5) != null;
};


/**
 * optional string endpoint = 6;
 * @return {string}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.getEndpoint = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 6, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.setEndpoint = function(value) {
  return jspb.Message.setField(this, 6, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.clearEndpoint = function() {
  return jspb.Message.setField(this, 6, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.hasEndpoint = function() {
  return jspb.Message.getField(this, 6) != null;
};


/**
 * optional string request_method = 7;
 * @return {string}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.getRequestMethod = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 7, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.setRequestMethod = function(value) {
  return jspb.Message.setField(this, 7, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.clearRequestMethod = function() {
  return jspb.Message.setField(this, 7, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.hasRequestMethod = function() {
  return jspb.Message.getField(this, 7) != null;
};


/**
 * optional int32 total_authentication_attempts = 8;
 * @return {number}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.getTotalAuthenticationAttempts = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 8, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.setTotalAuthenticationAttempts = function(value) {
  return jspb.Message.setField(this, 8, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.clearTotalAuthenticationAttempts = function() {
  return jspb.Message.setField(this, 8, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.hasTotalAuthenticationAttempts = function() {
  return jspb.Message.getField(this, 8) != null;
};


/**
 * optional int32 failed_authentication_attempts = 9;
 * @return {number}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.getFailedAuthenticationAttempts = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 9, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.setFailedAuthenticationAttempts = function(value) {
  return jspb.Message.setField(this, 9, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.clearFailedAuthenticationAttempts = function() {
  return jspb.Message.setField(this, 9, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.hasFailedAuthenticationAttempts = function() {
  return jspb.Message.getField(this, 9) != null;
};


/**
 * optional int32 blocked_injection_attempts = 10;
 * @return {number}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.getBlockedInjectionAttempts = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 10, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.setBlockedInjectionAttempts = function(value) {
  return jspb.Message.setField(this, 10, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.clearBlockedInjectionAttempts = function() {
  return jspb.Message.setField(this, 10, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.hasBlockedInjectionAttempts = function() {
  return jspb.Message.getField(this, 10) != null;
};


/**
 * optional int32 rate_limited_requests = 11;
 * @return {number}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.getRateLimitedRequests = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 11, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.setRateLimitedRequests = function(value) {
  return jspb.Message.setField(this, 11, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.clearRateLimitedRequests = function() {
  return jspb.Message.setField(this, 11, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.hasRateLimitedRequests = function() {
  return jspb.Message.getField(this, 11) != null;
};


/**
 * optional int32 suspicious_activities_detected = 12;
 * @return {number}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.getSuspiciousActivitiesDetected = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 12, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.setSuspiciousActivitiesDetected = function(value) {
  return jspb.Message.setField(this, 12, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.clearSuspiciousActivitiesDetected = function() {
  return jspb.Message.setField(this, 12, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.hasSuspiciousActivitiesDetected = function() {
  return jspb.Message.getField(this, 12) != null;
};


/**
 * optional int32 security_violations = 13;
 * @return {number}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.getSecurityViolations = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 13, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.setSecurityViolations = function(value) {
  return jspb.Message.setField(this, 13, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.clearSecurityViolations = function() {
  return jspb.Message.setField(this, 13, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.hasSecurityViolations = function() {
  return jspb.Message.getField(this, 13) != null;
};


/**
 * optional int32 uptime_seconds = 14;
 * @return {number}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.getUptimeSeconds = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 14, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.setUptimeSeconds = function(value) {
  return jspb.Message.setField(this, 14, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.clearUptimeSeconds = function() {
  return jspb.Message.setField(this, 14, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.hasUptimeSeconds = function() {
  return jspb.Message.getField(this, 14) != null;
};


/**
 * optional double security_health_score = 15;
 * @return {number}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.getSecurityHealthScore = function() {
  return /** @type {number} */ (jspb.Message.getFloatingPointFieldWithDefault(this, 15, 0.0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.setSecurityHealthScore = function(value) {
  return jspb.Message.setField(this, 15, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.clearSecurityHealthScore = function() {
  return jspb.Message.setField(this, 15, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.hasSecurityHealthScore = function() {
  return jspb.Message.getField(this, 15) != null;
};


/**
 * optional string metadata_json = 16;
 * @return {string}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.getMetadataJson = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 16, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.setMetadataJson = function(value) {
  return jspb.Message.setField(this, 16, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAuditModel} returns this
 */
proto.security_audit.CreateSecurityAuditModel.prototype.clearMetadataJson = function() {
  return jspb.Message.setField(this, 16, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAuditModel.prototype.hasMetadataJson = function() {
  return jspb.Message.getField(this, 16) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.UpdateSecurityAuditModel.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.UpdateSecurityAuditModel} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.UpdateSecurityAuditModel.toObject = function(includeInstance, msg) {
  var f, obj = {
totalAuthenticationAttempts: (f = jspb.Message.getField(msg, 1)) == null ? undefined : f,
failedAuthenticationAttempts: (f = jspb.Message.getField(msg, 2)) == null ? undefined : f,
blockedInjectionAttempts: (f = jspb.Message.getField(msg, 3)) == null ? undefined : f,
rateLimitedRequests: (f = jspb.Message.getField(msg, 4)) == null ? undefined : f,
suspiciousActivitiesDetected: (f = jspb.Message.getField(msg, 5)) == null ? undefined : f,
securityViolations: (f = jspb.Message.getField(msg, 6)) == null ? undefined : f,
uptimeSeconds: (f = jspb.Message.getField(msg, 7)) == null ? undefined : f,
securityHealthScore: (f = jspb.Message.getOptionalFloatingPointField(msg, 8)) == null ? undefined : f,
metadataJson: (f = jspb.Message.getField(msg, 9)) == null ? undefined : f
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.UpdateSecurityAuditModel}
 */
proto.security_audit.UpdateSecurityAuditModel.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.UpdateSecurityAuditModel;
  return proto.security_audit.UpdateSecurityAuditModel.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.UpdateSecurityAuditModel} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.UpdateSecurityAuditModel}
 */
proto.security_audit.UpdateSecurityAuditModel.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setTotalAuthenticationAttempts(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setFailedAuthenticationAttempts(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setBlockedInjectionAttempts(value);
      break;
    case 4:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setRateLimitedRequests(value);
      break;
    case 5:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setSuspiciousActivitiesDetected(value);
      break;
    case 6:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setSecurityViolations(value);
      break;
    case 7:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setUptimeSeconds(value);
      break;
    case 8:
      var value = /** @type {number} */ (reader.readDouble());
      msg.setSecurityHealthScore(value);
      break;
    case 9:
      var value = /** @type {string} */ (reader.readString());
      msg.setMetadataJson(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.UpdateSecurityAuditModel.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.UpdateSecurityAuditModel} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.UpdateSecurityAuditModel.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = /** @type {number} */ (jspb.Message.getField(message, 1));
  if (f != null) {
    writer.writeInt32(
      1,
      f
    );
  }
  f = /** @type {number} */ (jspb.Message.getField(message, 2));
  if (f != null) {
    writer.writeInt32(
      2,
      f
    );
  }
  f = /** @type {number} */ (jspb.Message.getField(message, 3));
  if (f != null) {
    writer.writeInt32(
      3,
      f
    );
  }
  f = /** @type {number} */ (jspb.Message.getField(message, 4));
  if (f != null) {
    writer.writeInt32(
      4,
      f
    );
  }
  f = /** @type {number} */ (jspb.Message.getField(message, 5));
  if (f != null) {
    writer.writeInt32(
      5,
      f
    );
  }
  f = /** @type {number} */ (jspb.Message.getField(message, 6));
  if (f != null) {
    writer.writeInt32(
      6,
      f
    );
  }
  f = /** @type {number} */ (jspb.Message.getField(message, 7));
  if (f != null) {
    writer.writeInt32(
      7,
      f
    );
  }
  f = /** @type {number} */ (jspb.Message.getField(message, 8));
  if (f != null) {
    writer.writeDouble(
      8,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 9));
  if (f != null) {
    writer.writeString(
      9,
      f
    );
  }
};


/**
 * optional int32 total_authentication_attempts = 1;
 * @return {number}
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.getTotalAuthenticationAttempts = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.UpdateSecurityAuditModel} returns this
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.setTotalAuthenticationAttempts = function(value) {
  return jspb.Message.setField(this, 1, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.UpdateSecurityAuditModel} returns this
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.clearTotalAuthenticationAttempts = function() {
  return jspb.Message.setField(this, 1, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.hasTotalAuthenticationAttempts = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * optional int32 failed_authentication_attempts = 2;
 * @return {number}
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.getFailedAuthenticationAttempts = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.UpdateSecurityAuditModel} returns this
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.setFailedAuthenticationAttempts = function(value) {
  return jspb.Message.setField(this, 2, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.UpdateSecurityAuditModel} returns this
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.clearFailedAuthenticationAttempts = function() {
  return jspb.Message.setField(this, 2, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.hasFailedAuthenticationAttempts = function() {
  return jspb.Message.getField(this, 2) != null;
};


/**
 * optional int32 blocked_injection_attempts = 3;
 * @return {number}
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.getBlockedInjectionAttempts = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.UpdateSecurityAuditModel} returns this
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.setBlockedInjectionAttempts = function(value) {
  return jspb.Message.setField(this, 3, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.UpdateSecurityAuditModel} returns this
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.clearBlockedInjectionAttempts = function() {
  return jspb.Message.setField(this, 3, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.hasBlockedInjectionAttempts = function() {
  return jspb.Message.getField(this, 3) != null;
};


/**
 * optional int32 rate_limited_requests = 4;
 * @return {number}
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.getRateLimitedRequests = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 4, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.UpdateSecurityAuditModel} returns this
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.setRateLimitedRequests = function(value) {
  return jspb.Message.setField(this, 4, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.UpdateSecurityAuditModel} returns this
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.clearRateLimitedRequests = function() {
  return jspb.Message.setField(this, 4, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.hasRateLimitedRequests = function() {
  return jspb.Message.getField(this, 4) != null;
};


/**
 * optional int32 suspicious_activities_detected = 5;
 * @return {number}
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.getSuspiciousActivitiesDetected = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 5, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.UpdateSecurityAuditModel} returns this
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.setSuspiciousActivitiesDetected = function(value) {
  return jspb.Message.setField(this, 5, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.UpdateSecurityAuditModel} returns this
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.clearSuspiciousActivitiesDetected = function() {
  return jspb.Message.setField(this, 5, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.hasSuspiciousActivitiesDetected = function() {
  return jspb.Message.getField(this, 5) != null;
};


/**
 * optional int32 security_violations = 6;
 * @return {number}
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.getSecurityViolations = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 6, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.UpdateSecurityAuditModel} returns this
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.setSecurityViolations = function(value) {
  return jspb.Message.setField(this, 6, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.UpdateSecurityAuditModel} returns this
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.clearSecurityViolations = function() {
  return jspb.Message.setField(this, 6, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.hasSecurityViolations = function() {
  return jspb.Message.getField(this, 6) != null;
};


/**
 * optional int32 uptime_seconds = 7;
 * @return {number}
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.getUptimeSeconds = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 7, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.UpdateSecurityAuditModel} returns this
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.setUptimeSeconds = function(value) {
  return jspb.Message.setField(this, 7, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.UpdateSecurityAuditModel} returns this
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.clearUptimeSeconds = function() {
  return jspb.Message.setField(this, 7, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.hasUptimeSeconds = function() {
  return jspb.Message.getField(this, 7) != null;
};


/**
 * optional double security_health_score = 8;
 * @return {number}
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.getSecurityHealthScore = function() {
  return /** @type {number} */ (jspb.Message.getFloatingPointFieldWithDefault(this, 8, 0.0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.UpdateSecurityAuditModel} returns this
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.setSecurityHealthScore = function(value) {
  return jspb.Message.setField(this, 8, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.UpdateSecurityAuditModel} returns this
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.clearSecurityHealthScore = function() {
  return jspb.Message.setField(this, 8, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.hasSecurityHealthScore = function() {
  return jspb.Message.getField(this, 8) != null;
};


/**
 * optional string metadata_json = 9;
 * @return {string}
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.getMetadataJson = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 9, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.UpdateSecurityAuditModel} returns this
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.setMetadataJson = function(value) {
  return jspb.Message.setField(this, 9, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.UpdateSecurityAuditModel} returns this
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.clearMetadataJson = function() {
  return jspb.Message.setField(this, 9, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.UpdateSecurityAuditModel.prototype.hasMetadataJson = function() {
  return jspb.Message.getField(this, 9) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.SecurityAlertModel.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.SecurityAlertModel.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.SecurityAlertModel} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.SecurityAlertModel.toObject = function(includeInstance, msg) {
  var f, obj = {
id: jspb.Message.getFieldWithDefault(msg, 1, ""),
alertId: jspb.Message.getFieldWithDefault(msg, 2, ""),
alertType: jspb.Message.getFieldWithDefault(msg, 3, 0),
severity: jspb.Message.getFieldWithDefault(msg, 4, 0),
message: jspb.Message.getFieldWithDefault(msg, 5, ""),
source: jspb.Message.getFieldWithDefault(msg, 6, ""),
affectedResource: (f = jspb.Message.getField(msg, 7)) == null ? undefined : f,
metadataJson: (f = jspb.Message.getField(msg, 8)) == null ? undefined : f,
resolved: jspb.Message.getBooleanFieldWithDefault(msg, 9, false),
resolvedAt: (f = msg.getResolvedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f),
resolvedBy: (f = jspb.Message.getField(msg, 11)) == null ? undefined : f,
createdAt: (f = msg.getCreatedAt()) && google_protobuf_timestamp_pb.Timestamp.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.SecurityAlertModel}
 */
proto.security_audit.SecurityAlertModel.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.SecurityAlertModel;
  return proto.security_audit.SecurityAlertModel.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.SecurityAlertModel} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.SecurityAlertModel}
 */
proto.security_audit.SecurityAlertModel.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setAlertId(value);
      break;
    case 3:
      var value = /** @type {!proto.security_audit.AlertType} */ (reader.readEnum());
      msg.setAlertType(value);
      break;
    case 4:
      var value = /** @type {!proto.security_audit.AlertSeverity} */ (reader.readEnum());
      msg.setSeverity(value);
      break;
    case 5:
      var value = /** @type {string} */ (reader.readString());
      msg.setMessage(value);
      break;
    case 6:
      var value = /** @type {string} */ (reader.readString());
      msg.setSource(value);
      break;
    case 7:
      var value = /** @type {string} */ (reader.readString());
      msg.setAffectedResource(value);
      break;
    case 8:
      var value = /** @type {string} */ (reader.readString());
      msg.setMetadataJson(value);
      break;
    case 9:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setResolved(value);
      break;
    case 10:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setResolvedAt(value);
      break;
    case 11:
      var value = /** @type {string} */ (reader.readString());
      msg.setResolvedBy(value);
      break;
    case 12:
      var value = new google_protobuf_timestamp_pb.Timestamp;
      reader.readMessage(value,google_protobuf_timestamp_pb.Timestamp.deserializeBinaryFromReader);
      msg.setCreatedAt(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.SecurityAlertModel.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.SecurityAlertModel.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.SecurityAlertModel} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.SecurityAlertModel.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getAlertId();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getAlertType();
  if (f !== 0.0) {
    writer.writeEnum(
      3,
      f
    );
  }
  f = message.getSeverity();
  if (f !== 0.0) {
    writer.writeEnum(
      4,
      f
    );
  }
  f = message.getMessage();
  if (f.length > 0) {
    writer.writeString(
      5,
      f
    );
  }
  f = message.getSource();
  if (f.length > 0) {
    writer.writeString(
      6,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 7));
  if (f != null) {
    writer.writeString(
      7,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 8));
  if (f != null) {
    writer.writeString(
      8,
      f
    );
  }
  f = message.getResolved();
  if (f) {
    writer.writeBool(
      9,
      f
    );
  }
  f = message.getResolvedAt();
  if (f != null) {
    writer.writeMessage(
      10,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 11));
  if (f != null) {
    writer.writeString(
      11,
      f
    );
  }
  f = message.getCreatedAt();
  if (f != null) {
    writer.writeMessage(
      12,
      f,
      google_protobuf_timestamp_pb.Timestamp.serializeBinaryToWriter
    );
  }
};


/**
 * optional string id = 1;
 * @return {string}
 */
proto.security_audit.SecurityAlertModel.prototype.getId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.SecurityAlertModel} returns this
 */
proto.security_audit.SecurityAlertModel.prototype.setId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string alert_id = 2;
 * @return {string}
 */
proto.security_audit.SecurityAlertModel.prototype.getAlertId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.SecurityAlertModel} returns this
 */
proto.security_audit.SecurityAlertModel.prototype.setAlertId = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};


/**
 * optional AlertType alert_type = 3;
 * @return {!proto.security_audit.AlertType}
 */
proto.security_audit.SecurityAlertModel.prototype.getAlertType = function() {
  return /** @type {!proto.security_audit.AlertType} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {!proto.security_audit.AlertType} value
 * @return {!proto.security_audit.SecurityAlertModel} returns this
 */
proto.security_audit.SecurityAlertModel.prototype.setAlertType = function(value) {
  return jspb.Message.setProto3EnumField(this, 3, value);
};


/**
 * optional AlertSeverity severity = 4;
 * @return {!proto.security_audit.AlertSeverity}
 */
proto.security_audit.SecurityAlertModel.prototype.getSeverity = function() {
  return /** @type {!proto.security_audit.AlertSeverity} */ (jspb.Message.getFieldWithDefault(this, 4, 0));
};


/**
 * @param {!proto.security_audit.AlertSeverity} value
 * @return {!proto.security_audit.SecurityAlertModel} returns this
 */
proto.security_audit.SecurityAlertModel.prototype.setSeverity = function(value) {
  return jspb.Message.setProto3EnumField(this, 4, value);
};


/**
 * optional string message = 5;
 * @return {string}
 */
proto.security_audit.SecurityAlertModel.prototype.getMessage = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 5, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.SecurityAlertModel} returns this
 */
proto.security_audit.SecurityAlertModel.prototype.setMessage = function(value) {
  return jspb.Message.setProto3StringField(this, 5, value);
};


/**
 * optional string source = 6;
 * @return {string}
 */
proto.security_audit.SecurityAlertModel.prototype.getSource = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 6, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.SecurityAlertModel} returns this
 */
proto.security_audit.SecurityAlertModel.prototype.setSource = function(value) {
  return jspb.Message.setProto3StringField(this, 6, value);
};


/**
 * optional string affected_resource = 7;
 * @return {string}
 */
proto.security_audit.SecurityAlertModel.prototype.getAffectedResource = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 7, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.SecurityAlertModel} returns this
 */
proto.security_audit.SecurityAlertModel.prototype.setAffectedResource = function(value) {
  return jspb.Message.setField(this, 7, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.SecurityAlertModel} returns this
 */
proto.security_audit.SecurityAlertModel.prototype.clearAffectedResource = function() {
  return jspb.Message.setField(this, 7, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.SecurityAlertModel.prototype.hasAffectedResource = function() {
  return jspb.Message.getField(this, 7) != null;
};


/**
 * optional string metadata_json = 8;
 * @return {string}
 */
proto.security_audit.SecurityAlertModel.prototype.getMetadataJson = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 8, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.SecurityAlertModel} returns this
 */
proto.security_audit.SecurityAlertModel.prototype.setMetadataJson = function(value) {
  return jspb.Message.setField(this, 8, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.SecurityAlertModel} returns this
 */
proto.security_audit.SecurityAlertModel.prototype.clearMetadataJson = function() {
  return jspb.Message.setField(this, 8, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.SecurityAlertModel.prototype.hasMetadataJson = function() {
  return jspb.Message.getField(this, 8) != null;
};


/**
 * optional bool resolved = 9;
 * @return {boolean}
 */
proto.security_audit.SecurityAlertModel.prototype.getResolved = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 9, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.SecurityAlertModel} returns this
 */
proto.security_audit.SecurityAlertModel.prototype.setResolved = function(value) {
  return jspb.Message.setProto3BooleanField(this, 9, value);
};


/**
 * optional google.protobuf.Timestamp resolved_at = 10;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.security_audit.SecurityAlertModel.prototype.getResolvedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 10));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.security_audit.SecurityAlertModel} returns this
*/
proto.security_audit.SecurityAlertModel.prototype.setResolvedAt = function(value) {
  return jspb.Message.setWrapperField(this, 10, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.SecurityAlertModel} returns this
 */
proto.security_audit.SecurityAlertModel.prototype.clearResolvedAt = function() {
  return this.setResolvedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.SecurityAlertModel.prototype.hasResolvedAt = function() {
  return jspb.Message.getField(this, 10) != null;
};


/**
 * optional string resolved_by = 11;
 * @return {string}
 */
proto.security_audit.SecurityAlertModel.prototype.getResolvedBy = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 11, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.SecurityAlertModel} returns this
 */
proto.security_audit.SecurityAlertModel.prototype.setResolvedBy = function(value) {
  return jspb.Message.setField(this, 11, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.SecurityAlertModel} returns this
 */
proto.security_audit.SecurityAlertModel.prototype.clearResolvedBy = function() {
  return jspb.Message.setField(this, 11, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.SecurityAlertModel.prototype.hasResolvedBy = function() {
  return jspb.Message.getField(this, 11) != null;
};


/**
 * optional google.protobuf.Timestamp created_at = 12;
 * @return {?proto.google.protobuf.Timestamp}
 */
proto.security_audit.SecurityAlertModel.prototype.getCreatedAt = function() {
  return /** @type{?proto.google.protobuf.Timestamp} */ (
    jspb.Message.getWrapperField(this, google_protobuf_timestamp_pb.Timestamp, 12));
};


/**
 * @param {?proto.google.protobuf.Timestamp|undefined} value
 * @return {!proto.security_audit.SecurityAlertModel} returns this
*/
proto.security_audit.SecurityAlertModel.prototype.setCreatedAt = function(value) {
  return jspb.Message.setWrapperField(this, 12, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.SecurityAlertModel} returns this
 */
proto.security_audit.SecurityAlertModel.prototype.clearCreatedAt = function() {
  return this.setCreatedAt(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.SecurityAlertModel.prototype.hasCreatedAt = function() {
  return jspb.Message.getField(this, 12) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.CreateSecurityAlertModel.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.CreateSecurityAlertModel.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.CreateSecurityAlertModel} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.CreateSecurityAlertModel.toObject = function(includeInstance, msg) {
  var f, obj = {
alertId: jspb.Message.getFieldWithDefault(msg, 1, ""),
alertType: jspb.Message.getFieldWithDefault(msg, 2, 0),
severity: jspb.Message.getFieldWithDefault(msg, 3, 0),
message: jspb.Message.getFieldWithDefault(msg, 4, ""),
source: jspb.Message.getFieldWithDefault(msg, 5, ""),
affectedResource: (f = jspb.Message.getField(msg, 6)) == null ? undefined : f,
metadataJson: (f = jspb.Message.getField(msg, 7)) == null ? undefined : f
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.CreateSecurityAlertModel}
 */
proto.security_audit.CreateSecurityAlertModel.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.CreateSecurityAlertModel;
  return proto.security_audit.CreateSecurityAlertModel.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.CreateSecurityAlertModel} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.CreateSecurityAlertModel}
 */
proto.security_audit.CreateSecurityAlertModel.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setAlertId(value);
      break;
    case 2:
      var value = /** @type {!proto.security_audit.AlertType} */ (reader.readEnum());
      msg.setAlertType(value);
      break;
    case 3:
      var value = /** @type {!proto.security_audit.AlertSeverity} */ (reader.readEnum());
      msg.setSeverity(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setMessage(value);
      break;
    case 5:
      var value = /** @type {string} */ (reader.readString());
      msg.setSource(value);
      break;
    case 6:
      var value = /** @type {string} */ (reader.readString());
      msg.setAffectedResource(value);
      break;
    case 7:
      var value = /** @type {string} */ (reader.readString());
      msg.setMetadataJson(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.CreateSecurityAlertModel.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.CreateSecurityAlertModel.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.CreateSecurityAlertModel} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.CreateSecurityAlertModel.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getAlertId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getAlertType();
  if (f !== 0.0) {
    writer.writeEnum(
      2,
      f
    );
  }
  f = message.getSeverity();
  if (f !== 0.0) {
    writer.writeEnum(
      3,
      f
    );
  }
  f = message.getMessage();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
  f = message.getSource();
  if (f.length > 0) {
    writer.writeString(
      5,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 6));
  if (f != null) {
    writer.writeString(
      6,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 7));
  if (f != null) {
    writer.writeString(
      7,
      f
    );
  }
};


/**
 * optional string alert_id = 1;
 * @return {string}
 */
proto.security_audit.CreateSecurityAlertModel.prototype.getAlertId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.CreateSecurityAlertModel} returns this
 */
proto.security_audit.CreateSecurityAlertModel.prototype.setAlertId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional AlertType alert_type = 2;
 * @return {!proto.security_audit.AlertType}
 */
proto.security_audit.CreateSecurityAlertModel.prototype.getAlertType = function() {
  return /** @type {!proto.security_audit.AlertType} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {!proto.security_audit.AlertType} value
 * @return {!proto.security_audit.CreateSecurityAlertModel} returns this
 */
proto.security_audit.CreateSecurityAlertModel.prototype.setAlertType = function(value) {
  return jspb.Message.setProto3EnumField(this, 2, value);
};


/**
 * optional AlertSeverity severity = 3;
 * @return {!proto.security_audit.AlertSeverity}
 */
proto.security_audit.CreateSecurityAlertModel.prototype.getSeverity = function() {
  return /** @type {!proto.security_audit.AlertSeverity} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {!proto.security_audit.AlertSeverity} value
 * @return {!proto.security_audit.CreateSecurityAlertModel} returns this
 */
proto.security_audit.CreateSecurityAlertModel.prototype.setSeverity = function(value) {
  return jspb.Message.setProto3EnumField(this, 3, value);
};


/**
 * optional string message = 4;
 * @return {string}
 */
proto.security_audit.CreateSecurityAlertModel.prototype.getMessage = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.CreateSecurityAlertModel} returns this
 */
proto.security_audit.CreateSecurityAlertModel.prototype.setMessage = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional string source = 5;
 * @return {string}
 */
proto.security_audit.CreateSecurityAlertModel.prototype.getSource = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 5, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.CreateSecurityAlertModel} returns this
 */
proto.security_audit.CreateSecurityAlertModel.prototype.setSource = function(value) {
  return jspb.Message.setProto3StringField(this, 5, value);
};


/**
 * optional string affected_resource = 6;
 * @return {string}
 */
proto.security_audit.CreateSecurityAlertModel.prototype.getAffectedResource = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 6, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.CreateSecurityAlertModel} returns this
 */
proto.security_audit.CreateSecurityAlertModel.prototype.setAffectedResource = function(value) {
  return jspb.Message.setField(this, 6, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAlertModel} returns this
 */
proto.security_audit.CreateSecurityAlertModel.prototype.clearAffectedResource = function() {
  return jspb.Message.setField(this, 6, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAlertModel.prototype.hasAffectedResource = function() {
  return jspb.Message.getField(this, 6) != null;
};


/**
 * optional string metadata_json = 7;
 * @return {string}
 */
proto.security_audit.CreateSecurityAlertModel.prototype.getMetadataJson = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 7, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.CreateSecurityAlertModel} returns this
 */
proto.security_audit.CreateSecurityAlertModel.prototype.setMetadataJson = function(value) {
  return jspb.Message.setField(this, 7, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAlertModel} returns this
 */
proto.security_audit.CreateSecurityAlertModel.prototype.clearMetadataJson = function() {
  return jspb.Message.setField(this, 7, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAlertModel.prototype.hasMetadataJson = function() {
  return jspb.Message.getField(this, 7) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.UpdateSecurityAlertModel.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.UpdateSecurityAlertModel.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.UpdateSecurityAlertModel} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.UpdateSecurityAlertModel.toObject = function(includeInstance, msg) {
  var f, obj = {
resolved: (f = jspb.Message.getBooleanField(msg, 1)) == null ? undefined : f,
resolvedBy: (f = jspb.Message.getField(msg, 2)) == null ? undefined : f,
metadataJson: (f = jspb.Message.getField(msg, 3)) == null ? undefined : f
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.UpdateSecurityAlertModel}
 */
proto.security_audit.UpdateSecurityAlertModel.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.UpdateSecurityAlertModel;
  return proto.security_audit.UpdateSecurityAlertModel.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.UpdateSecurityAlertModel} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.UpdateSecurityAlertModel}
 */
proto.security_audit.UpdateSecurityAlertModel.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setResolved(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setResolvedBy(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setMetadataJson(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.UpdateSecurityAlertModel.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.UpdateSecurityAlertModel.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.UpdateSecurityAlertModel} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.UpdateSecurityAlertModel.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = /** @type {boolean} */ (jspb.Message.getField(message, 1));
  if (f != null) {
    writer.writeBool(
      1,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 2));
  if (f != null) {
    writer.writeString(
      2,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 3));
  if (f != null) {
    writer.writeString(
      3,
      f
    );
  }
};


/**
 * optional bool resolved = 1;
 * @return {boolean}
 */
proto.security_audit.UpdateSecurityAlertModel.prototype.getResolved = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 1, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.UpdateSecurityAlertModel} returns this
 */
proto.security_audit.UpdateSecurityAlertModel.prototype.setResolved = function(value) {
  return jspb.Message.setField(this, 1, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.UpdateSecurityAlertModel} returns this
 */
proto.security_audit.UpdateSecurityAlertModel.prototype.clearResolved = function() {
  return jspb.Message.setField(this, 1, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.UpdateSecurityAlertModel.prototype.hasResolved = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * optional string resolved_by = 2;
 * @return {string}
 */
proto.security_audit.UpdateSecurityAlertModel.prototype.getResolvedBy = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.UpdateSecurityAlertModel} returns this
 */
proto.security_audit.UpdateSecurityAlertModel.prototype.setResolvedBy = function(value) {
  return jspb.Message.setField(this, 2, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.UpdateSecurityAlertModel} returns this
 */
proto.security_audit.UpdateSecurityAlertModel.prototype.clearResolvedBy = function() {
  return jspb.Message.setField(this, 2, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.UpdateSecurityAlertModel.prototype.hasResolvedBy = function() {
  return jspb.Message.getField(this, 2) != null;
};


/**
 * optional string metadata_json = 3;
 * @return {string}
 */
proto.security_audit.UpdateSecurityAlertModel.prototype.getMetadataJson = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.UpdateSecurityAlertModel} returns this
 */
proto.security_audit.UpdateSecurityAlertModel.prototype.setMetadataJson = function(value) {
  return jspb.Message.setField(this, 3, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.UpdateSecurityAlertModel} returns this
 */
proto.security_audit.UpdateSecurityAlertModel.prototype.clearMetadataJson = function() {
  return jspb.Message.setField(this, 3, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.UpdateSecurityAlertModel.prototype.hasMetadataJson = function() {
  return jspb.Message.getField(this, 3) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.Pagination.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.Pagination.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.Pagination} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.Pagination.toObject = function(includeInstance, msg) {
  var f, obj = {
total: jspb.Message.getFieldWithDefault(msg, 1, 0),
perPage: jspb.Message.getFieldWithDefault(msg, 2, 0),
currentPage: jspb.Message.getFieldWithDefault(msg, 3, 0),
from: jspb.Message.getFieldWithDefault(msg, 4, 0),
to: jspb.Message.getFieldWithDefault(msg, 5, 0),
hasNextPage: jspb.Message.getBooleanFieldWithDefault(msg, 6, false),
hasPreviousPage: jspb.Message.getBooleanFieldWithDefault(msg, 7, false),
nextPageNumber: jspb.Message.getFieldWithDefault(msg, 8, 0),
previousPageNumber: jspb.Message.getFieldWithDefault(msg, 9, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.Pagination}
 */
proto.security_audit.Pagination.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.Pagination;
  return proto.security_audit.Pagination.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.Pagination} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.Pagination}
 */
proto.security_audit.Pagination.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setTotal(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setPerPage(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setCurrentPage(value);
      break;
    case 4:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setFrom(value);
      break;
    case 5:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setTo(value);
      break;
    case 6:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setHasNextPage(value);
      break;
    case 7:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setHasPreviousPage(value);
      break;
    case 8:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setNextPageNumber(value);
      break;
    case 9:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setPreviousPageNumber(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.Pagination.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.Pagination.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.Pagination} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.Pagination.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getTotal();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
  f = message.getPerPage();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
  f = message.getCurrentPage();
  if (f !== 0) {
    writer.writeInt64(
      3,
      f
    );
  }
  f = message.getFrom();
  if (f !== 0) {
    writer.writeInt64(
      4,
      f
    );
  }
  f = message.getTo();
  if (f !== 0) {
    writer.writeInt64(
      5,
      f
    );
  }
  f = message.getHasNextPage();
  if (f) {
    writer.writeBool(
      6,
      f
    );
  }
  f = message.getHasPreviousPage();
  if (f) {
    writer.writeBool(
      7,
      f
    );
  }
  f = message.getNextPageNumber();
  if (f !== 0) {
    writer.writeInt64(
      8,
      f
    );
  }
  f = message.getPreviousPageNumber();
  if (f !== 0) {
    writer.writeInt64(
      9,
      f
    );
  }
};


/**
 * optional int64 total = 1;
 * @return {number}
 */
proto.security_audit.Pagination.prototype.getTotal = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.Pagination} returns this
 */
proto.security_audit.Pagination.prototype.setTotal = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional int64 per_page = 2;
 * @return {number}
 */
proto.security_audit.Pagination.prototype.getPerPage = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.Pagination} returns this
 */
proto.security_audit.Pagination.prototype.setPerPage = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


/**
 * optional int64 current_page = 3;
 * @return {number}
 */
proto.security_audit.Pagination.prototype.getCurrentPage = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.Pagination} returns this
 */
proto.security_audit.Pagination.prototype.setCurrentPage = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};


/**
 * optional int64 from = 4;
 * @return {number}
 */
proto.security_audit.Pagination.prototype.getFrom = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 4, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.Pagination} returns this
 */
proto.security_audit.Pagination.prototype.setFrom = function(value) {
  return jspb.Message.setProto3IntField(this, 4, value);
};


/**
 * optional int64 to = 5;
 * @return {number}
 */
proto.security_audit.Pagination.prototype.getTo = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 5, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.Pagination} returns this
 */
proto.security_audit.Pagination.prototype.setTo = function(value) {
  return jspb.Message.setProto3IntField(this, 5, value);
};


/**
 * optional bool has_next_page = 6;
 * @return {boolean}
 */
proto.security_audit.Pagination.prototype.getHasNextPage = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 6, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.Pagination} returns this
 */
proto.security_audit.Pagination.prototype.setHasNextPage = function(value) {
  return jspb.Message.setProto3BooleanField(this, 6, value);
};


/**
 * optional bool has_previous_page = 7;
 * @return {boolean}
 */
proto.security_audit.Pagination.prototype.getHasPreviousPage = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 7, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.Pagination} returns this
 */
proto.security_audit.Pagination.prototype.setHasPreviousPage = function(value) {
  return jspb.Message.setProto3BooleanField(this, 7, value);
};


/**
 * optional int64 next_page_number = 8;
 * @return {number}
 */
proto.security_audit.Pagination.prototype.getNextPageNumber = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 8, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.Pagination} returns this
 */
proto.security_audit.Pagination.prototype.setNextPageNumber = function(value) {
  return jspb.Message.setProto3IntField(this, 8, value);
};


/**
 * optional int64 previous_page_number = 9;
 * @return {number}
 */
proto.security_audit.Pagination.prototype.getPreviousPageNumber = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 9, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.Pagination} returns this
 */
proto.security_audit.Pagination.prototype.setPreviousPageNumber = function(value) {
  return jspb.Message.setProto3IntField(this, 9, value);
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.security_audit.SecurityAuditPaginationModel.repeatedFields_ = [1];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.SecurityAuditPaginationModel.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.SecurityAuditPaginationModel.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.SecurityAuditPaginationModel} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.SecurityAuditPaginationModel.toObject = function(includeInstance, msg) {
  var f, obj = {
dataList: jspb.Message.toObjectList(msg.getDataList(),
    proto.security_audit.SecurityAuditModel.toObject, includeInstance),
pagination: (f = msg.getPagination()) && proto.security_audit.Pagination.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.SecurityAuditPaginationModel}
 */
proto.security_audit.SecurityAuditPaginationModel.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.SecurityAuditPaginationModel;
  return proto.security_audit.SecurityAuditPaginationModel.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.SecurityAuditPaginationModel} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.SecurityAuditPaginationModel}
 */
proto.security_audit.SecurityAuditPaginationModel.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.security_audit.SecurityAuditModel;
      reader.readMessage(value,proto.security_audit.SecurityAuditModel.deserializeBinaryFromReader);
      msg.addData(value);
      break;
    case 2:
      var value = new proto.security_audit.Pagination;
      reader.readMessage(value,proto.security_audit.Pagination.deserializeBinaryFromReader);
      msg.setPagination(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.SecurityAuditPaginationModel.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.SecurityAuditPaginationModel.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.SecurityAuditPaginationModel} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.SecurityAuditPaginationModel.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getDataList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.security_audit.SecurityAuditModel.serializeBinaryToWriter
    );
  }
  f = message.getPagination();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.security_audit.Pagination.serializeBinaryToWriter
    );
  }
};


/**
 * repeated SecurityAuditModel data = 1;
 * @return {!Array<!proto.security_audit.SecurityAuditModel>}
 */
proto.security_audit.SecurityAuditPaginationModel.prototype.getDataList = function() {
  return /** @type{!Array<!proto.security_audit.SecurityAuditModel>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.security_audit.SecurityAuditModel, 1));
};


/**
 * @param {!Array<!proto.security_audit.SecurityAuditModel>} value
 * @return {!proto.security_audit.SecurityAuditPaginationModel} returns this
*/
proto.security_audit.SecurityAuditPaginationModel.prototype.setDataList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.security_audit.SecurityAuditModel=} opt_value
 * @param {number=} opt_index
 * @return {!proto.security_audit.SecurityAuditModel}
 */
proto.security_audit.SecurityAuditPaginationModel.prototype.addData = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.security_audit.SecurityAuditModel, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.security_audit.SecurityAuditPaginationModel} returns this
 */
proto.security_audit.SecurityAuditPaginationModel.prototype.clearDataList = function() {
  return this.setDataList([]);
};


/**
 * optional Pagination pagination = 2;
 * @return {?proto.security_audit.Pagination}
 */
proto.security_audit.SecurityAuditPaginationModel.prototype.getPagination = function() {
  return /** @type{?proto.security_audit.Pagination} */ (
    jspb.Message.getWrapperField(this, proto.security_audit.Pagination, 2));
};


/**
 * @param {?proto.security_audit.Pagination|undefined} value
 * @return {!proto.security_audit.SecurityAuditPaginationModel} returns this
*/
proto.security_audit.SecurityAuditPaginationModel.prototype.setPagination = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.SecurityAuditPaginationModel} returns this
 */
proto.security_audit.SecurityAuditPaginationModel.prototype.clearPagination = function() {
  return this.setPagination(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.SecurityAuditPaginationModel.prototype.hasPagination = function() {
  return jspb.Message.getField(this, 2) != null;
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.security_audit.SecurityAlertPaginationModel.repeatedFields_ = [1];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.SecurityAlertPaginationModel.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.SecurityAlertPaginationModel.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.SecurityAlertPaginationModel} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.SecurityAlertPaginationModel.toObject = function(includeInstance, msg) {
  var f, obj = {
dataList: jspb.Message.toObjectList(msg.getDataList(),
    proto.security_audit.SecurityAlertModel.toObject, includeInstance),
pagination: (f = msg.getPagination()) && proto.security_audit.Pagination.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.SecurityAlertPaginationModel}
 */
proto.security_audit.SecurityAlertPaginationModel.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.SecurityAlertPaginationModel;
  return proto.security_audit.SecurityAlertPaginationModel.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.SecurityAlertPaginationModel} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.SecurityAlertPaginationModel}
 */
proto.security_audit.SecurityAlertPaginationModel.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.security_audit.SecurityAlertModel;
      reader.readMessage(value,proto.security_audit.SecurityAlertModel.deserializeBinaryFromReader);
      msg.addData(value);
      break;
    case 2:
      var value = new proto.security_audit.Pagination;
      reader.readMessage(value,proto.security_audit.Pagination.deserializeBinaryFromReader);
      msg.setPagination(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.SecurityAlertPaginationModel.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.SecurityAlertPaginationModel.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.SecurityAlertPaginationModel} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.SecurityAlertPaginationModel.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getDataList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      1,
      f,
      proto.security_audit.SecurityAlertModel.serializeBinaryToWriter
    );
  }
  f = message.getPagination();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.security_audit.Pagination.serializeBinaryToWriter
    );
  }
};


/**
 * repeated SecurityAlertModel data = 1;
 * @return {!Array<!proto.security_audit.SecurityAlertModel>}
 */
proto.security_audit.SecurityAlertPaginationModel.prototype.getDataList = function() {
  return /** @type{!Array<!proto.security_audit.SecurityAlertModel>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.security_audit.SecurityAlertModel, 1));
};


/**
 * @param {!Array<!proto.security_audit.SecurityAlertModel>} value
 * @return {!proto.security_audit.SecurityAlertPaginationModel} returns this
*/
proto.security_audit.SecurityAlertPaginationModel.prototype.setDataList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 1, value);
};


/**
 * @param {!proto.security_audit.SecurityAlertModel=} opt_value
 * @param {number=} opt_index
 * @return {!proto.security_audit.SecurityAlertModel}
 */
proto.security_audit.SecurityAlertPaginationModel.prototype.addData = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 1, opt_value, proto.security_audit.SecurityAlertModel, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.security_audit.SecurityAlertPaginationModel} returns this
 */
proto.security_audit.SecurityAlertPaginationModel.prototype.clearDataList = function() {
  return this.setDataList([]);
};


/**
 * optional Pagination pagination = 2;
 * @return {?proto.security_audit.Pagination}
 */
proto.security_audit.SecurityAlertPaginationModel.prototype.getPagination = function() {
  return /** @type{?proto.security_audit.Pagination} */ (
    jspb.Message.getWrapperField(this, proto.security_audit.Pagination, 2));
};


/**
 * @param {?proto.security_audit.Pagination|undefined} value
 * @return {!proto.security_audit.SecurityAlertPaginationModel} returns this
*/
proto.security_audit.SecurityAlertPaginationModel.prototype.setPagination = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.SecurityAlertPaginationModel} returns this
 */
proto.security_audit.SecurityAlertPaginationModel.prototype.clearPagination = function() {
  return this.setPagination(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.SecurityAlertPaginationModel.prototype.hasPagination = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.SecuritySummary.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.SecuritySummary.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.SecuritySummary} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.SecuritySummary.toObject = function(includeInstance, msg) {
  var f, obj = {
ipAddress: jspb.Message.getFieldWithDefault(msg, 1, ""),
totalRecords: jspb.Message.getFieldWithDefault(msg, 2, 0),
totalAuthenticationAttempts: jspb.Message.getFieldWithDefault(msg, 3, 0),
failedAuthenticationAttempts: jspb.Message.getFieldWithDefault(msg, 4, 0),
blockedInjectionAttempts: jspb.Message.getFieldWithDefault(msg, 5, 0),
rateLimitedRequests: jspb.Message.getFieldWithDefault(msg, 6, 0),
suspiciousActivitiesDetected: jspb.Message.getFieldWithDefault(msg, 7, 0),
securityViolations: jspb.Message.getFieldWithDefault(msg, 8, 0),
lowestHealthScore: jspb.Message.getFloatingPointFieldWithDefault(msg, 9, 0.0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.SecuritySummary}
 */
proto.security_audit.SecuritySummary.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.SecuritySummary;
  return proto.security_audit.SecuritySummary.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.SecuritySummary} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.SecuritySummary}
 */
proto.security_audit.SecuritySummary.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setIpAddress(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setTotalRecords(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setTotalAuthenticationAttempts(value);
      break;
    case 4:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setFailedAuthenticationAttempts(value);
      break;
    case 5:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setBlockedInjectionAttempts(value);
      break;
    case 6:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setRateLimitedRequests(value);
      break;
    case 7:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setSuspiciousActivitiesDetected(value);
      break;
    case 8:
      var value = /** @type {number} */ (reader.readInt32());
      msg.setSecurityViolations(value);
      break;
    case 9:
      var value = /** @type {number} */ (reader.readDouble());
      msg.setLowestHealthScore(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.SecuritySummary.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.SecuritySummary.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.SecuritySummary} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.SecuritySummary.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIpAddress();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getTotalRecords();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
  f = message.getTotalAuthenticationAttempts();
  if (f !== 0) {
    writer.writeInt32(
      3,
      f
    );
  }
  f = message.getFailedAuthenticationAttempts();
  if (f !== 0) {
    writer.writeInt32(
      4,
      f
    );
  }
  f = message.getBlockedInjectionAttempts();
  if (f !== 0) {
    writer.writeInt32(
      5,
      f
    );
  }
  f = message.getRateLimitedRequests();
  if (f !== 0) {
    writer.writeInt32(
      6,
      f
    );
  }
  f = message.getSuspiciousActivitiesDetected();
  if (f !== 0) {
    writer.writeInt32(
      7,
      f
    );
  }
  f = message.getSecurityViolations();
  if (f !== 0) {
    writer.writeInt32(
      8,
      f
    );
  }
  f = message.getLowestHealthScore();
  if (f !== 0.0) {
    writer.writeDouble(
      9,
      f
    );
  }
};


/**
 * optional string ip_address = 1;
 * @return {string}
 */
proto.security_audit.SecuritySummary.prototype.getIpAddress = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.SecuritySummary} returns this
 */
proto.security_audit.SecuritySummary.prototype.setIpAddress = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional int64 total_records = 2;
 * @return {number}
 */
proto.security_audit.SecuritySummary.prototype.getTotalRecords = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.SecuritySummary} returns this
 */
proto.security_audit.SecuritySummary.prototype.setTotalRecords = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


/**
 * optional int32 total_authentication_attempts = 3;
 * @return {number}
 */
proto.security_audit.SecuritySummary.prototype.getTotalAuthenticationAttempts = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.SecuritySummary} returns this
 */
proto.security_audit.SecuritySummary.prototype.setTotalAuthenticationAttempts = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};


/**
 * optional int32 failed_authentication_attempts = 4;
 * @return {number}
 */
proto.security_audit.SecuritySummary.prototype.getFailedAuthenticationAttempts = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 4, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.SecuritySummary} returns this
 */
proto.security_audit.SecuritySummary.prototype.setFailedAuthenticationAttempts = function(value) {
  return jspb.Message.setProto3IntField(this, 4, value);
};


/**
 * optional int32 blocked_injection_attempts = 5;
 * @return {number}
 */
proto.security_audit.SecuritySummary.prototype.getBlockedInjectionAttempts = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 5, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.SecuritySummary} returns this
 */
proto.security_audit.SecuritySummary.prototype.setBlockedInjectionAttempts = function(value) {
  return jspb.Message.setProto3IntField(this, 5, value);
};


/**
 * optional int32 rate_limited_requests = 6;
 * @return {number}
 */
proto.security_audit.SecuritySummary.prototype.getRateLimitedRequests = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 6, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.SecuritySummary} returns this
 */
proto.security_audit.SecuritySummary.prototype.setRateLimitedRequests = function(value) {
  return jspb.Message.setProto3IntField(this, 6, value);
};


/**
 * optional int32 suspicious_activities_detected = 7;
 * @return {number}
 */
proto.security_audit.SecuritySummary.prototype.getSuspiciousActivitiesDetected = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 7, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.SecuritySummary} returns this
 */
proto.security_audit.SecuritySummary.prototype.setSuspiciousActivitiesDetected = function(value) {
  return jspb.Message.setProto3IntField(this, 7, value);
};


/**
 * optional int32 security_violations = 8;
 * @return {number}
 */
proto.security_audit.SecuritySummary.prototype.getSecurityViolations = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 8, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.SecuritySummary} returns this
 */
proto.security_audit.SecuritySummary.prototype.setSecurityViolations = function(value) {
  return jspb.Message.setProto3IntField(this, 8, value);
};


/**
 * optional double lowest_health_score = 9;
 * @return {number}
 */
proto.security_audit.SecuritySummary.prototype.getLowestHealthScore = function() {
  return /** @type {number} */ (jspb.Message.getFloatingPointFieldWithDefault(this, 9, 0.0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.SecuritySummary} returns this
 */
proto.security_audit.SecuritySummary.prototype.setLowestHealthScore = function(value) {
  return jspb.Message.setProto3FloatField(this, 9, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.AlertStatistics.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.AlertStatistics.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.AlertStatistics} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.AlertStatistics.toObject = function(includeInstance, msg) {
  var f, obj = {
totalAlerts: jspb.Message.getFieldWithDefault(msg, 1, 0),
totalUnresolved: jspb.Message.getFieldWithDefault(msg, 2, 0),
totalCriticalUnresolved: jspb.Message.getFieldWithDefault(msg, 3, 0),
totalLow: jspb.Message.getFieldWithDefault(msg, 4, 0),
totalMedium: jspb.Message.getFieldWithDefault(msg, 5, 0),
totalHigh: jspb.Message.getFieldWithDefault(msg, 6, 0),
totalCritical: jspb.Message.getFieldWithDefault(msg, 7, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.AlertStatistics}
 */
proto.security_audit.AlertStatistics.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.AlertStatistics;
  return proto.security_audit.AlertStatistics.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.AlertStatistics} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.AlertStatistics}
 */
proto.security_audit.AlertStatistics.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setTotalAlerts(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setTotalUnresolved(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setTotalCriticalUnresolved(value);
      break;
    case 4:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setTotalLow(value);
      break;
    case 5:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setTotalMedium(value);
      break;
    case 6:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setTotalHigh(value);
      break;
    case 7:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setTotalCritical(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.AlertStatistics.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.AlertStatistics.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.AlertStatistics} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.AlertStatistics.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getTotalAlerts();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
  f = message.getTotalUnresolved();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
  f = message.getTotalCriticalUnresolved();
  if (f !== 0) {
    writer.writeInt64(
      3,
      f
    );
  }
  f = message.getTotalLow();
  if (f !== 0) {
    writer.writeInt64(
      4,
      f
    );
  }
  f = message.getTotalMedium();
  if (f !== 0) {
    writer.writeInt64(
      5,
      f
    );
  }
  f = message.getTotalHigh();
  if (f !== 0) {
    writer.writeInt64(
      6,
      f
    );
  }
  f = message.getTotalCritical();
  if (f !== 0) {
    writer.writeInt64(
      7,
      f
    );
  }
};


/**
 * optional int64 total_alerts = 1;
 * @return {number}
 */
proto.security_audit.AlertStatistics.prototype.getTotalAlerts = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.AlertStatistics} returns this
 */
proto.security_audit.AlertStatistics.prototype.setTotalAlerts = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional int64 total_unresolved = 2;
 * @return {number}
 */
proto.security_audit.AlertStatistics.prototype.getTotalUnresolved = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.AlertStatistics} returns this
 */
proto.security_audit.AlertStatistics.prototype.setTotalUnresolved = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


/**
 * optional int64 total_critical_unresolved = 3;
 * @return {number}
 */
proto.security_audit.AlertStatistics.prototype.getTotalCriticalUnresolved = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.AlertStatistics} returns this
 */
proto.security_audit.AlertStatistics.prototype.setTotalCriticalUnresolved = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};


/**
 * optional int64 total_low = 4;
 * @return {number}
 */
proto.security_audit.AlertStatistics.prototype.getTotalLow = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 4, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.AlertStatistics} returns this
 */
proto.security_audit.AlertStatistics.prototype.setTotalLow = function(value) {
  return jspb.Message.setProto3IntField(this, 4, value);
};


/**
 * optional int64 total_medium = 5;
 * @return {number}
 */
proto.security_audit.AlertStatistics.prototype.getTotalMedium = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 5, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.AlertStatistics} returns this
 */
proto.security_audit.AlertStatistics.prototype.setTotalMedium = function(value) {
  return jspb.Message.setProto3IntField(this, 5, value);
};


/**
 * optional int64 total_high = 6;
 * @return {number}
 */
proto.security_audit.AlertStatistics.prototype.getTotalHigh = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 6, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.AlertStatistics} returns this
 */
proto.security_audit.AlertStatistics.prototype.setTotalHigh = function(value) {
  return jspb.Message.setProto3IntField(this, 6, value);
};


/**
 * optional int64 total_critical = 7;
 * @return {number}
 */
proto.security_audit.AlertStatistics.prototype.getTotalCritical = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 7, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.AlertStatistics} returns this
 */
proto.security_audit.AlertStatistics.prototype.setTotalCritical = function(value) {
  return jspb.Message.setProto3IntField(this, 7, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.CreateSecurityAuditRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.CreateSecurityAuditRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.CreateSecurityAuditRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.CreateSecurityAuditRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
audit: (f = msg.getAudit()) && proto.security_audit.CreateSecurityAuditModel.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.CreateSecurityAuditRequest}
 */
proto.security_audit.CreateSecurityAuditRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.CreateSecurityAuditRequest;
  return proto.security_audit.CreateSecurityAuditRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.CreateSecurityAuditRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.CreateSecurityAuditRequest}
 */
proto.security_audit.CreateSecurityAuditRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.security_audit.CreateSecurityAuditModel;
      reader.readMessage(value,proto.security_audit.CreateSecurityAuditModel.deserializeBinaryFromReader);
      msg.setAudit(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.CreateSecurityAuditRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.CreateSecurityAuditRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.CreateSecurityAuditRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.CreateSecurityAuditRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getAudit();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.security_audit.CreateSecurityAuditModel.serializeBinaryToWriter
    );
  }
};


/**
 * optional CreateSecurityAuditModel audit = 1;
 * @return {?proto.security_audit.CreateSecurityAuditModel}
 */
proto.security_audit.CreateSecurityAuditRequest.prototype.getAudit = function() {
  return /** @type{?proto.security_audit.CreateSecurityAuditModel} */ (
    jspb.Message.getWrapperField(this, proto.security_audit.CreateSecurityAuditModel, 1));
};


/**
 * @param {?proto.security_audit.CreateSecurityAuditModel|undefined} value
 * @return {!proto.security_audit.CreateSecurityAuditRequest} returns this
*/
proto.security_audit.CreateSecurityAuditRequest.prototype.setAudit = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAuditRequest} returns this
 */
proto.security_audit.CreateSecurityAuditRequest.prototype.clearAudit = function() {
  return this.setAudit(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAuditRequest.prototype.hasAudit = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.CreateSecurityAuditResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.CreateSecurityAuditResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.CreateSecurityAuditResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.CreateSecurityAuditResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
status: jspb.Message.getBooleanFieldWithDefault(msg, 1, false),
data: (f = msg.getData()) && proto.security_audit.SecurityAuditModel.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.CreateSecurityAuditResponse}
 */
proto.security_audit.CreateSecurityAuditResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.CreateSecurityAuditResponse;
  return proto.security_audit.CreateSecurityAuditResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.CreateSecurityAuditResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.CreateSecurityAuditResponse}
 */
proto.security_audit.CreateSecurityAuditResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setStatus(value);
      break;
    case 2:
      var value = new proto.security_audit.SecurityAuditModel;
      reader.readMessage(value,proto.security_audit.SecurityAuditModel.deserializeBinaryFromReader);
      msg.setData(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.CreateSecurityAuditResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.CreateSecurityAuditResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.CreateSecurityAuditResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.CreateSecurityAuditResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStatus();
  if (f) {
    writer.writeBool(
      1,
      f
    );
  }
  f = message.getData();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.security_audit.SecurityAuditModel.serializeBinaryToWriter
    );
  }
};


/**
 * optional bool status = 1;
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAuditResponse.prototype.getStatus = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 1, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.CreateSecurityAuditResponse} returns this
 */
proto.security_audit.CreateSecurityAuditResponse.prototype.setStatus = function(value) {
  return jspb.Message.setProto3BooleanField(this, 1, value);
};


/**
 * optional SecurityAuditModel data = 2;
 * @return {?proto.security_audit.SecurityAuditModel}
 */
proto.security_audit.CreateSecurityAuditResponse.prototype.getData = function() {
  return /** @type{?proto.security_audit.SecurityAuditModel} */ (
    jspb.Message.getWrapperField(this, proto.security_audit.SecurityAuditModel, 2));
};


/**
 * @param {?proto.security_audit.SecurityAuditModel|undefined} value
 * @return {!proto.security_audit.CreateSecurityAuditResponse} returns this
*/
proto.security_audit.CreateSecurityAuditResponse.prototype.setData = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAuditResponse} returns this
 */
proto.security_audit.CreateSecurityAuditResponse.prototype.clearData = function() {
  return this.setData(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAuditResponse.prototype.hasData = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.LogSecurityEventRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.LogSecurityEventRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.LogSecurityEventRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.LogSecurityEventRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
adminUserId: (f = jspb.Message.getField(msg, 1)) == null ? undefined : f,
sessionId: (f = jspb.Message.getField(msg, 2)) == null ? undefined : f,
ipAddress: jspb.Message.getFieldWithDefault(msg, 3, ""),
userAgent: (f = jspb.Message.getField(msg, 4)) == null ? undefined : f,
endpoint: (f = jspb.Message.getField(msg, 5)) == null ? undefined : f,
requestMethod: (f = jspb.Message.getField(msg, 6)) == null ? undefined : f,
eventType: jspb.Message.getFieldWithDefault(msg, 7, 0),
metadataJson: (f = jspb.Message.getField(msg, 8)) == null ? undefined : f
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.LogSecurityEventRequest}
 */
proto.security_audit.LogSecurityEventRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.LogSecurityEventRequest;
  return proto.security_audit.LogSecurityEventRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.LogSecurityEventRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.LogSecurityEventRequest}
 */
proto.security_audit.LogSecurityEventRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setAdminUserId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setSessionId(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setIpAddress(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setUserAgent(value);
      break;
    case 5:
      var value = /** @type {string} */ (reader.readString());
      msg.setEndpoint(value);
      break;
    case 6:
      var value = /** @type {string} */ (reader.readString());
      msg.setRequestMethod(value);
      break;
    case 7:
      var value = /** @type {!proto.security_audit.SecurityEventType} */ (reader.readEnum());
      msg.setEventType(value);
      break;
    case 8:
      var value = /** @type {string} */ (reader.readString());
      msg.setMetadataJson(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.LogSecurityEventRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.LogSecurityEventRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.LogSecurityEventRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.LogSecurityEventRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = /** @type {string} */ (jspb.Message.getField(message, 1));
  if (f != null) {
    writer.writeString(
      1,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 2));
  if (f != null) {
    writer.writeString(
      2,
      f
    );
  }
  f = message.getIpAddress();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 4));
  if (f != null) {
    writer.writeString(
      4,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 5));
  if (f != null) {
    writer.writeString(
      5,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 6));
  if (f != null) {
    writer.writeString(
      6,
      f
    );
  }
  f = message.getEventType();
  if (f !== 0.0) {
    writer.writeEnum(
      7,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 8));
  if (f != null) {
    writer.writeString(
      8,
      f
    );
  }
};


/**
 * optional string admin_user_id = 1;
 * @return {string}
 */
proto.security_audit.LogSecurityEventRequest.prototype.getAdminUserId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.LogSecurityEventRequest} returns this
 */
proto.security_audit.LogSecurityEventRequest.prototype.setAdminUserId = function(value) {
  return jspb.Message.setField(this, 1, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.LogSecurityEventRequest} returns this
 */
proto.security_audit.LogSecurityEventRequest.prototype.clearAdminUserId = function() {
  return jspb.Message.setField(this, 1, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.LogSecurityEventRequest.prototype.hasAdminUserId = function() {
  return jspb.Message.getField(this, 1) != null;
};


/**
 * optional string session_id = 2;
 * @return {string}
 */
proto.security_audit.LogSecurityEventRequest.prototype.getSessionId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.LogSecurityEventRequest} returns this
 */
proto.security_audit.LogSecurityEventRequest.prototype.setSessionId = function(value) {
  return jspb.Message.setField(this, 2, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.LogSecurityEventRequest} returns this
 */
proto.security_audit.LogSecurityEventRequest.prototype.clearSessionId = function() {
  return jspb.Message.setField(this, 2, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.LogSecurityEventRequest.prototype.hasSessionId = function() {
  return jspb.Message.getField(this, 2) != null;
};


/**
 * optional string ip_address = 3;
 * @return {string}
 */
proto.security_audit.LogSecurityEventRequest.prototype.getIpAddress = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.LogSecurityEventRequest} returns this
 */
proto.security_audit.LogSecurityEventRequest.prototype.setIpAddress = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string user_agent = 4;
 * @return {string}
 */
proto.security_audit.LogSecurityEventRequest.prototype.getUserAgent = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.LogSecurityEventRequest} returns this
 */
proto.security_audit.LogSecurityEventRequest.prototype.setUserAgent = function(value) {
  return jspb.Message.setField(this, 4, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.LogSecurityEventRequest} returns this
 */
proto.security_audit.LogSecurityEventRequest.prototype.clearUserAgent = function() {
  return jspb.Message.setField(this, 4, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.LogSecurityEventRequest.prototype.hasUserAgent = function() {
  return jspb.Message.getField(this, 4) != null;
};


/**
 * optional string endpoint = 5;
 * @return {string}
 */
proto.security_audit.LogSecurityEventRequest.prototype.getEndpoint = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 5, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.LogSecurityEventRequest} returns this
 */
proto.security_audit.LogSecurityEventRequest.prototype.setEndpoint = function(value) {
  return jspb.Message.setField(this, 5, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.LogSecurityEventRequest} returns this
 */
proto.security_audit.LogSecurityEventRequest.prototype.clearEndpoint = function() {
  return jspb.Message.setField(this, 5, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.LogSecurityEventRequest.prototype.hasEndpoint = function() {
  return jspb.Message.getField(this, 5) != null;
};


/**
 * optional string request_method = 6;
 * @return {string}
 */
proto.security_audit.LogSecurityEventRequest.prototype.getRequestMethod = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 6, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.LogSecurityEventRequest} returns this
 */
proto.security_audit.LogSecurityEventRequest.prototype.setRequestMethod = function(value) {
  return jspb.Message.setField(this, 6, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.LogSecurityEventRequest} returns this
 */
proto.security_audit.LogSecurityEventRequest.prototype.clearRequestMethod = function() {
  return jspb.Message.setField(this, 6, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.LogSecurityEventRequest.prototype.hasRequestMethod = function() {
  return jspb.Message.getField(this, 6) != null;
};


/**
 * optional SecurityEventType event_type = 7;
 * @return {!proto.security_audit.SecurityEventType}
 */
proto.security_audit.LogSecurityEventRequest.prototype.getEventType = function() {
  return /** @type {!proto.security_audit.SecurityEventType} */ (jspb.Message.getFieldWithDefault(this, 7, 0));
};


/**
 * @param {!proto.security_audit.SecurityEventType} value
 * @return {!proto.security_audit.LogSecurityEventRequest} returns this
 */
proto.security_audit.LogSecurityEventRequest.prototype.setEventType = function(value) {
  return jspb.Message.setProto3EnumField(this, 7, value);
};


/**
 * optional string metadata_json = 8;
 * @return {string}
 */
proto.security_audit.LogSecurityEventRequest.prototype.getMetadataJson = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 8, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.LogSecurityEventRequest} returns this
 */
proto.security_audit.LogSecurityEventRequest.prototype.setMetadataJson = function(value) {
  return jspb.Message.setField(this, 8, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.LogSecurityEventRequest} returns this
 */
proto.security_audit.LogSecurityEventRequest.prototype.clearMetadataJson = function() {
  return jspb.Message.setField(this, 8, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.LogSecurityEventRequest.prototype.hasMetadataJson = function() {
  return jspb.Message.getField(this, 8) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.LogSecurityEventResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.LogSecurityEventResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.LogSecurityEventResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.LogSecurityEventResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
status: jspb.Message.getBooleanFieldWithDefault(msg, 1, false),
data: (f = msg.getData()) && proto.security_audit.SecurityAuditModel.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.LogSecurityEventResponse}
 */
proto.security_audit.LogSecurityEventResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.LogSecurityEventResponse;
  return proto.security_audit.LogSecurityEventResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.LogSecurityEventResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.LogSecurityEventResponse}
 */
proto.security_audit.LogSecurityEventResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setStatus(value);
      break;
    case 2:
      var value = new proto.security_audit.SecurityAuditModel;
      reader.readMessage(value,proto.security_audit.SecurityAuditModel.deserializeBinaryFromReader);
      msg.setData(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.LogSecurityEventResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.LogSecurityEventResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.LogSecurityEventResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.LogSecurityEventResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStatus();
  if (f) {
    writer.writeBool(
      1,
      f
    );
  }
  f = message.getData();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.security_audit.SecurityAuditModel.serializeBinaryToWriter
    );
  }
};


/**
 * optional bool status = 1;
 * @return {boolean}
 */
proto.security_audit.LogSecurityEventResponse.prototype.getStatus = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 1, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.LogSecurityEventResponse} returns this
 */
proto.security_audit.LogSecurityEventResponse.prototype.setStatus = function(value) {
  return jspb.Message.setProto3BooleanField(this, 1, value);
};


/**
 * optional SecurityAuditModel data = 2;
 * @return {?proto.security_audit.SecurityAuditModel}
 */
proto.security_audit.LogSecurityEventResponse.prototype.getData = function() {
  return /** @type{?proto.security_audit.SecurityAuditModel} */ (
    jspb.Message.getWrapperField(this, proto.security_audit.SecurityAuditModel, 2));
};


/**
 * @param {?proto.security_audit.SecurityAuditModel|undefined} value
 * @return {!proto.security_audit.LogSecurityEventResponse} returns this
*/
proto.security_audit.LogSecurityEventResponse.prototype.setData = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.LogSecurityEventResponse} returns this
 */
proto.security_audit.LogSecurityEventResponse.prototype.clearData = function() {
  return this.setData(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.LogSecurityEventResponse.prototype.hasData = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetSecurityAuditRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetSecurityAuditRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetSecurityAuditRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAuditRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
id: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetSecurityAuditRequest}
 */
proto.security_audit.GetSecurityAuditRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetSecurityAuditRequest;
  return proto.security_audit.GetSecurityAuditRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetSecurityAuditRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetSecurityAuditRequest}
 */
proto.security_audit.GetSecurityAuditRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetSecurityAuditRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetSecurityAuditRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetSecurityAuditRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAuditRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string id = 1;
 * @return {string}
 */
proto.security_audit.GetSecurityAuditRequest.prototype.getId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.GetSecurityAuditRequest} returns this
 */
proto.security_audit.GetSecurityAuditRequest.prototype.setId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetSecurityAuditResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetSecurityAuditResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetSecurityAuditResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAuditResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
status: jspb.Message.getBooleanFieldWithDefault(msg, 1, false),
data: (f = msg.getData()) && proto.security_audit.SecurityAuditModel.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetSecurityAuditResponse}
 */
proto.security_audit.GetSecurityAuditResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetSecurityAuditResponse;
  return proto.security_audit.GetSecurityAuditResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetSecurityAuditResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetSecurityAuditResponse}
 */
proto.security_audit.GetSecurityAuditResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setStatus(value);
      break;
    case 2:
      var value = new proto.security_audit.SecurityAuditModel;
      reader.readMessage(value,proto.security_audit.SecurityAuditModel.deserializeBinaryFromReader);
      msg.setData(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetSecurityAuditResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetSecurityAuditResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetSecurityAuditResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAuditResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStatus();
  if (f) {
    writer.writeBool(
      1,
      f
    );
  }
  f = message.getData();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.security_audit.SecurityAuditModel.serializeBinaryToWriter
    );
  }
};


/**
 * optional bool status = 1;
 * @return {boolean}
 */
proto.security_audit.GetSecurityAuditResponse.prototype.getStatus = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 1, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.GetSecurityAuditResponse} returns this
 */
proto.security_audit.GetSecurityAuditResponse.prototype.setStatus = function(value) {
  return jspb.Message.setProto3BooleanField(this, 1, value);
};


/**
 * optional SecurityAuditModel data = 2;
 * @return {?proto.security_audit.SecurityAuditModel}
 */
proto.security_audit.GetSecurityAuditResponse.prototype.getData = function() {
  return /** @type{?proto.security_audit.SecurityAuditModel} */ (
    jspb.Message.getWrapperField(this, proto.security_audit.SecurityAuditModel, 2));
};


/**
 * @param {?proto.security_audit.SecurityAuditModel|undefined} value
 * @return {!proto.security_audit.GetSecurityAuditResponse} returns this
*/
proto.security_audit.GetSecurityAuditResponse.prototype.setData = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.GetSecurityAuditResponse} returns this
 */
proto.security_audit.GetSecurityAuditResponse.prototype.clearData = function() {
  return this.setData(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.GetSecurityAuditResponse.prototype.hasData = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetSecurityAuditsByUserRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetSecurityAuditsByUserRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetSecurityAuditsByUserRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAuditsByUserRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
adminUserId: jspb.Message.getFieldWithDefault(msg, 1, ""),
page: jspb.Message.getFieldWithDefault(msg, 2, 0),
perPage: jspb.Message.getFieldWithDefault(msg, 3, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetSecurityAuditsByUserRequest}
 */
proto.security_audit.GetSecurityAuditsByUserRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetSecurityAuditsByUserRequest;
  return proto.security_audit.GetSecurityAuditsByUserRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetSecurityAuditsByUserRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetSecurityAuditsByUserRequest}
 */
proto.security_audit.GetSecurityAuditsByUserRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setAdminUserId(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setPage(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setPerPage(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetSecurityAuditsByUserRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetSecurityAuditsByUserRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetSecurityAuditsByUserRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAuditsByUserRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getAdminUserId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getPage();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
  f = message.getPerPage();
  if (f !== 0) {
    writer.writeInt64(
      3,
      f
    );
  }
};


/**
 * optional string admin_user_id = 1;
 * @return {string}
 */
proto.security_audit.GetSecurityAuditsByUserRequest.prototype.getAdminUserId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.GetSecurityAuditsByUserRequest} returns this
 */
proto.security_audit.GetSecurityAuditsByUserRequest.prototype.setAdminUserId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional int64 page = 2;
 * @return {number}
 */
proto.security_audit.GetSecurityAuditsByUserRequest.prototype.getPage = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.GetSecurityAuditsByUserRequest} returns this
 */
proto.security_audit.GetSecurityAuditsByUserRequest.prototype.setPage = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


/**
 * optional int64 per_page = 3;
 * @return {number}
 */
proto.security_audit.GetSecurityAuditsByUserRequest.prototype.getPerPage = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.GetSecurityAuditsByUserRequest} returns this
 */
proto.security_audit.GetSecurityAuditsByUserRequest.prototype.setPerPage = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetSecurityAuditsByUserResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetSecurityAuditsByUserResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetSecurityAuditsByUserResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAuditsByUserResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
status: jspb.Message.getBooleanFieldWithDefault(msg, 1, false),
data: (f = msg.getData()) && proto.security_audit.SecurityAuditPaginationModel.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetSecurityAuditsByUserResponse}
 */
proto.security_audit.GetSecurityAuditsByUserResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetSecurityAuditsByUserResponse;
  return proto.security_audit.GetSecurityAuditsByUserResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetSecurityAuditsByUserResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetSecurityAuditsByUserResponse}
 */
proto.security_audit.GetSecurityAuditsByUserResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setStatus(value);
      break;
    case 2:
      var value = new proto.security_audit.SecurityAuditPaginationModel;
      reader.readMessage(value,proto.security_audit.SecurityAuditPaginationModel.deserializeBinaryFromReader);
      msg.setData(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetSecurityAuditsByUserResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetSecurityAuditsByUserResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetSecurityAuditsByUserResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAuditsByUserResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStatus();
  if (f) {
    writer.writeBool(
      1,
      f
    );
  }
  f = message.getData();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.security_audit.SecurityAuditPaginationModel.serializeBinaryToWriter
    );
  }
};


/**
 * optional bool status = 1;
 * @return {boolean}
 */
proto.security_audit.GetSecurityAuditsByUserResponse.prototype.getStatus = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 1, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.GetSecurityAuditsByUserResponse} returns this
 */
proto.security_audit.GetSecurityAuditsByUserResponse.prototype.setStatus = function(value) {
  return jspb.Message.setProto3BooleanField(this, 1, value);
};


/**
 * optional SecurityAuditPaginationModel data = 2;
 * @return {?proto.security_audit.SecurityAuditPaginationModel}
 */
proto.security_audit.GetSecurityAuditsByUserResponse.prototype.getData = function() {
  return /** @type{?proto.security_audit.SecurityAuditPaginationModel} */ (
    jspb.Message.getWrapperField(this, proto.security_audit.SecurityAuditPaginationModel, 2));
};


/**
 * @param {?proto.security_audit.SecurityAuditPaginationModel|undefined} value
 * @return {!proto.security_audit.GetSecurityAuditsByUserResponse} returns this
*/
proto.security_audit.GetSecurityAuditsByUserResponse.prototype.setData = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.GetSecurityAuditsByUserResponse} returns this
 */
proto.security_audit.GetSecurityAuditsByUserResponse.prototype.clearData = function() {
  return this.setData(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.GetSecurityAuditsByUserResponse.prototype.hasData = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetSecurityAuditsByIpRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetSecurityAuditsByIpRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetSecurityAuditsByIpRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAuditsByIpRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
ipAddress: jspb.Message.getFieldWithDefault(msg, 1, ""),
page: jspb.Message.getFieldWithDefault(msg, 2, 0),
perPage: jspb.Message.getFieldWithDefault(msg, 3, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetSecurityAuditsByIpRequest}
 */
proto.security_audit.GetSecurityAuditsByIpRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetSecurityAuditsByIpRequest;
  return proto.security_audit.GetSecurityAuditsByIpRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetSecurityAuditsByIpRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetSecurityAuditsByIpRequest}
 */
proto.security_audit.GetSecurityAuditsByIpRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setIpAddress(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setPage(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setPerPage(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetSecurityAuditsByIpRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetSecurityAuditsByIpRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetSecurityAuditsByIpRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAuditsByIpRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIpAddress();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getPage();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
  f = message.getPerPage();
  if (f !== 0) {
    writer.writeInt64(
      3,
      f
    );
  }
};


/**
 * optional string ip_address = 1;
 * @return {string}
 */
proto.security_audit.GetSecurityAuditsByIpRequest.prototype.getIpAddress = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.GetSecurityAuditsByIpRequest} returns this
 */
proto.security_audit.GetSecurityAuditsByIpRequest.prototype.setIpAddress = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional int64 page = 2;
 * @return {number}
 */
proto.security_audit.GetSecurityAuditsByIpRequest.prototype.getPage = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.GetSecurityAuditsByIpRequest} returns this
 */
proto.security_audit.GetSecurityAuditsByIpRequest.prototype.setPage = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


/**
 * optional int64 per_page = 3;
 * @return {number}
 */
proto.security_audit.GetSecurityAuditsByIpRequest.prototype.getPerPage = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.GetSecurityAuditsByIpRequest} returns this
 */
proto.security_audit.GetSecurityAuditsByIpRequest.prototype.setPerPage = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetSecurityAuditsByIpResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetSecurityAuditsByIpResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetSecurityAuditsByIpResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAuditsByIpResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
status: jspb.Message.getBooleanFieldWithDefault(msg, 1, false),
data: (f = msg.getData()) && proto.security_audit.SecurityAuditPaginationModel.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetSecurityAuditsByIpResponse}
 */
proto.security_audit.GetSecurityAuditsByIpResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetSecurityAuditsByIpResponse;
  return proto.security_audit.GetSecurityAuditsByIpResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetSecurityAuditsByIpResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetSecurityAuditsByIpResponse}
 */
proto.security_audit.GetSecurityAuditsByIpResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setStatus(value);
      break;
    case 2:
      var value = new proto.security_audit.SecurityAuditPaginationModel;
      reader.readMessage(value,proto.security_audit.SecurityAuditPaginationModel.deserializeBinaryFromReader);
      msg.setData(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetSecurityAuditsByIpResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetSecurityAuditsByIpResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetSecurityAuditsByIpResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAuditsByIpResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStatus();
  if (f) {
    writer.writeBool(
      1,
      f
    );
  }
  f = message.getData();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.security_audit.SecurityAuditPaginationModel.serializeBinaryToWriter
    );
  }
};


/**
 * optional bool status = 1;
 * @return {boolean}
 */
proto.security_audit.GetSecurityAuditsByIpResponse.prototype.getStatus = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 1, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.GetSecurityAuditsByIpResponse} returns this
 */
proto.security_audit.GetSecurityAuditsByIpResponse.prototype.setStatus = function(value) {
  return jspb.Message.setProto3BooleanField(this, 1, value);
};


/**
 * optional SecurityAuditPaginationModel data = 2;
 * @return {?proto.security_audit.SecurityAuditPaginationModel}
 */
proto.security_audit.GetSecurityAuditsByIpResponse.prototype.getData = function() {
  return /** @type{?proto.security_audit.SecurityAuditPaginationModel} */ (
    jspb.Message.getWrapperField(this, proto.security_audit.SecurityAuditPaginationModel, 2));
};


/**
 * @param {?proto.security_audit.SecurityAuditPaginationModel|undefined} value
 * @return {!proto.security_audit.GetSecurityAuditsByIpResponse} returns this
*/
proto.security_audit.GetSecurityAuditsByIpResponse.prototype.setData = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.GetSecurityAuditsByIpResponse} returns this
 */
proto.security_audit.GetSecurityAuditsByIpResponse.prototype.clearData = function() {
  return this.setData(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.GetSecurityAuditsByIpResponse.prototype.hasData = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetSecurityAuditsPaginatedRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetSecurityAuditsPaginatedRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetSecurityAuditsPaginatedRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAuditsPaginatedRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
page: jspb.Message.getFieldWithDefault(msg, 1, 0),
perPage: jspb.Message.getFieldWithDefault(msg, 2, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetSecurityAuditsPaginatedRequest}
 */
proto.security_audit.GetSecurityAuditsPaginatedRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetSecurityAuditsPaginatedRequest;
  return proto.security_audit.GetSecurityAuditsPaginatedRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetSecurityAuditsPaginatedRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetSecurityAuditsPaginatedRequest}
 */
proto.security_audit.GetSecurityAuditsPaginatedRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setPage(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setPerPage(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetSecurityAuditsPaginatedRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetSecurityAuditsPaginatedRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetSecurityAuditsPaginatedRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAuditsPaginatedRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getPage();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
  f = message.getPerPage();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
};


/**
 * optional int64 page = 1;
 * @return {number}
 */
proto.security_audit.GetSecurityAuditsPaginatedRequest.prototype.getPage = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.GetSecurityAuditsPaginatedRequest} returns this
 */
proto.security_audit.GetSecurityAuditsPaginatedRequest.prototype.setPage = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional int64 per_page = 2;
 * @return {number}
 */
proto.security_audit.GetSecurityAuditsPaginatedRequest.prototype.getPerPage = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.GetSecurityAuditsPaginatedRequest} returns this
 */
proto.security_audit.GetSecurityAuditsPaginatedRequest.prototype.setPerPage = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetSecurityAuditsPaginatedResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetSecurityAuditsPaginatedResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetSecurityAuditsPaginatedResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAuditsPaginatedResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
status: jspb.Message.getBooleanFieldWithDefault(msg, 1, false),
data: (f = msg.getData()) && proto.security_audit.SecurityAuditPaginationModel.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetSecurityAuditsPaginatedResponse}
 */
proto.security_audit.GetSecurityAuditsPaginatedResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetSecurityAuditsPaginatedResponse;
  return proto.security_audit.GetSecurityAuditsPaginatedResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetSecurityAuditsPaginatedResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetSecurityAuditsPaginatedResponse}
 */
proto.security_audit.GetSecurityAuditsPaginatedResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setStatus(value);
      break;
    case 2:
      var value = new proto.security_audit.SecurityAuditPaginationModel;
      reader.readMessage(value,proto.security_audit.SecurityAuditPaginationModel.deserializeBinaryFromReader);
      msg.setData(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetSecurityAuditsPaginatedResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetSecurityAuditsPaginatedResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetSecurityAuditsPaginatedResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAuditsPaginatedResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStatus();
  if (f) {
    writer.writeBool(
      1,
      f
    );
  }
  f = message.getData();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.security_audit.SecurityAuditPaginationModel.serializeBinaryToWriter
    );
  }
};


/**
 * optional bool status = 1;
 * @return {boolean}
 */
proto.security_audit.GetSecurityAuditsPaginatedResponse.prototype.getStatus = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 1, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.GetSecurityAuditsPaginatedResponse} returns this
 */
proto.security_audit.GetSecurityAuditsPaginatedResponse.prototype.setStatus = function(value) {
  return jspb.Message.setProto3BooleanField(this, 1, value);
};


/**
 * optional SecurityAuditPaginationModel data = 2;
 * @return {?proto.security_audit.SecurityAuditPaginationModel}
 */
proto.security_audit.GetSecurityAuditsPaginatedResponse.prototype.getData = function() {
  return /** @type{?proto.security_audit.SecurityAuditPaginationModel} */ (
    jspb.Message.getWrapperField(this, proto.security_audit.SecurityAuditPaginationModel, 2));
};


/**
 * @param {?proto.security_audit.SecurityAuditPaginationModel|undefined} value
 * @return {!proto.security_audit.GetSecurityAuditsPaginatedResponse} returns this
*/
proto.security_audit.GetSecurityAuditsPaginatedResponse.prototype.setData = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.GetSecurityAuditsPaginatedResponse} returns this
 */
proto.security_audit.GetSecurityAuditsPaginatedResponse.prototype.clearData = function() {
  return this.setData(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.GetSecurityAuditsPaginatedResponse.prototype.hasData = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.UpdateSecurityAuditRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.UpdateSecurityAuditRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.UpdateSecurityAuditRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.UpdateSecurityAuditRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
id: jspb.Message.getFieldWithDefault(msg, 1, ""),
audit: (f = msg.getAudit()) && proto.security_audit.UpdateSecurityAuditModel.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.UpdateSecurityAuditRequest}
 */
proto.security_audit.UpdateSecurityAuditRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.UpdateSecurityAuditRequest;
  return proto.security_audit.UpdateSecurityAuditRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.UpdateSecurityAuditRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.UpdateSecurityAuditRequest}
 */
proto.security_audit.UpdateSecurityAuditRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setId(value);
      break;
    case 2:
      var value = new proto.security_audit.UpdateSecurityAuditModel;
      reader.readMessage(value,proto.security_audit.UpdateSecurityAuditModel.deserializeBinaryFromReader);
      msg.setAudit(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.UpdateSecurityAuditRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.UpdateSecurityAuditRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.UpdateSecurityAuditRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.UpdateSecurityAuditRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getAudit();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.security_audit.UpdateSecurityAuditModel.serializeBinaryToWriter
    );
  }
};


/**
 * optional string id = 1;
 * @return {string}
 */
proto.security_audit.UpdateSecurityAuditRequest.prototype.getId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.UpdateSecurityAuditRequest} returns this
 */
proto.security_audit.UpdateSecurityAuditRequest.prototype.setId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional UpdateSecurityAuditModel audit = 2;
 * @return {?proto.security_audit.UpdateSecurityAuditModel}
 */
proto.security_audit.UpdateSecurityAuditRequest.prototype.getAudit = function() {
  return /** @type{?proto.security_audit.UpdateSecurityAuditModel} */ (
    jspb.Message.getWrapperField(this, proto.security_audit.UpdateSecurityAuditModel, 2));
};


/**
 * @param {?proto.security_audit.UpdateSecurityAuditModel|undefined} value
 * @return {!proto.security_audit.UpdateSecurityAuditRequest} returns this
*/
proto.security_audit.UpdateSecurityAuditRequest.prototype.setAudit = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.UpdateSecurityAuditRequest} returns this
 */
proto.security_audit.UpdateSecurityAuditRequest.prototype.clearAudit = function() {
  return this.setAudit(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.UpdateSecurityAuditRequest.prototype.hasAudit = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.UpdateSecurityAuditResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.UpdateSecurityAuditResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.UpdateSecurityAuditResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.UpdateSecurityAuditResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
status: jspb.Message.getBooleanFieldWithDefault(msg, 1, false),
data: (f = msg.getData()) && proto.security_audit.SecurityAuditModel.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.UpdateSecurityAuditResponse}
 */
proto.security_audit.UpdateSecurityAuditResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.UpdateSecurityAuditResponse;
  return proto.security_audit.UpdateSecurityAuditResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.UpdateSecurityAuditResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.UpdateSecurityAuditResponse}
 */
proto.security_audit.UpdateSecurityAuditResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setStatus(value);
      break;
    case 2:
      var value = new proto.security_audit.SecurityAuditModel;
      reader.readMessage(value,proto.security_audit.SecurityAuditModel.deserializeBinaryFromReader);
      msg.setData(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.UpdateSecurityAuditResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.UpdateSecurityAuditResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.UpdateSecurityAuditResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.UpdateSecurityAuditResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStatus();
  if (f) {
    writer.writeBool(
      1,
      f
    );
  }
  f = message.getData();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.security_audit.SecurityAuditModel.serializeBinaryToWriter
    );
  }
};


/**
 * optional bool status = 1;
 * @return {boolean}
 */
proto.security_audit.UpdateSecurityAuditResponse.prototype.getStatus = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 1, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.UpdateSecurityAuditResponse} returns this
 */
proto.security_audit.UpdateSecurityAuditResponse.prototype.setStatus = function(value) {
  return jspb.Message.setProto3BooleanField(this, 1, value);
};


/**
 * optional SecurityAuditModel data = 2;
 * @return {?proto.security_audit.SecurityAuditModel}
 */
proto.security_audit.UpdateSecurityAuditResponse.prototype.getData = function() {
  return /** @type{?proto.security_audit.SecurityAuditModel} */ (
    jspb.Message.getWrapperField(this, proto.security_audit.SecurityAuditModel, 2));
};


/**
 * @param {?proto.security_audit.SecurityAuditModel|undefined} value
 * @return {!proto.security_audit.UpdateSecurityAuditResponse} returns this
*/
proto.security_audit.UpdateSecurityAuditResponse.prototype.setData = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.UpdateSecurityAuditResponse} returns this
 */
proto.security_audit.UpdateSecurityAuditResponse.prototype.clearData = function() {
  return this.setData(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.UpdateSecurityAuditResponse.prototype.hasData = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.DeleteSecurityAuditRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.DeleteSecurityAuditRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.DeleteSecurityAuditRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.DeleteSecurityAuditRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
id: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.DeleteSecurityAuditRequest}
 */
proto.security_audit.DeleteSecurityAuditRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.DeleteSecurityAuditRequest;
  return proto.security_audit.DeleteSecurityAuditRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.DeleteSecurityAuditRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.DeleteSecurityAuditRequest}
 */
proto.security_audit.DeleteSecurityAuditRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.DeleteSecurityAuditRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.DeleteSecurityAuditRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.DeleteSecurityAuditRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.DeleteSecurityAuditRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string id = 1;
 * @return {string}
 */
proto.security_audit.DeleteSecurityAuditRequest.prototype.getId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.DeleteSecurityAuditRequest} returns this
 */
proto.security_audit.DeleteSecurityAuditRequest.prototype.setId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.DeleteSecurityAuditResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.DeleteSecurityAuditResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.DeleteSecurityAuditResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.DeleteSecurityAuditResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
status: jspb.Message.getBooleanFieldWithDefault(msg, 1, false)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.DeleteSecurityAuditResponse}
 */
proto.security_audit.DeleteSecurityAuditResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.DeleteSecurityAuditResponse;
  return proto.security_audit.DeleteSecurityAuditResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.DeleteSecurityAuditResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.DeleteSecurityAuditResponse}
 */
proto.security_audit.DeleteSecurityAuditResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setStatus(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.DeleteSecurityAuditResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.DeleteSecurityAuditResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.DeleteSecurityAuditResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.DeleteSecurityAuditResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStatus();
  if (f) {
    writer.writeBool(
      1,
      f
    );
  }
};


/**
 * optional bool status = 1;
 * @return {boolean}
 */
proto.security_audit.DeleteSecurityAuditResponse.prototype.getStatus = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 1, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.DeleteSecurityAuditResponse} returns this
 */
proto.security_audit.DeleteSecurityAuditResponse.prototype.setStatus = function(value) {
  return jspb.Message.setProto3BooleanField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetIpSecuritySummaryRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetIpSecuritySummaryRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetIpSecuritySummaryRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetIpSecuritySummaryRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
ipAddress: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetIpSecuritySummaryRequest}
 */
proto.security_audit.GetIpSecuritySummaryRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetIpSecuritySummaryRequest;
  return proto.security_audit.GetIpSecuritySummaryRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetIpSecuritySummaryRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetIpSecuritySummaryRequest}
 */
proto.security_audit.GetIpSecuritySummaryRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setIpAddress(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetIpSecuritySummaryRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetIpSecuritySummaryRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetIpSecuritySummaryRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetIpSecuritySummaryRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getIpAddress();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string ip_address = 1;
 * @return {string}
 */
proto.security_audit.GetIpSecuritySummaryRequest.prototype.getIpAddress = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.GetIpSecuritySummaryRequest} returns this
 */
proto.security_audit.GetIpSecuritySummaryRequest.prototype.setIpAddress = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetIpSecuritySummaryResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetIpSecuritySummaryResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetIpSecuritySummaryResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetIpSecuritySummaryResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
status: jspb.Message.getBooleanFieldWithDefault(msg, 1, false),
data: (f = msg.getData()) && proto.security_audit.SecuritySummary.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetIpSecuritySummaryResponse}
 */
proto.security_audit.GetIpSecuritySummaryResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetIpSecuritySummaryResponse;
  return proto.security_audit.GetIpSecuritySummaryResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetIpSecuritySummaryResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetIpSecuritySummaryResponse}
 */
proto.security_audit.GetIpSecuritySummaryResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setStatus(value);
      break;
    case 2:
      var value = new proto.security_audit.SecuritySummary;
      reader.readMessage(value,proto.security_audit.SecuritySummary.deserializeBinaryFromReader);
      msg.setData(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetIpSecuritySummaryResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetIpSecuritySummaryResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetIpSecuritySummaryResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetIpSecuritySummaryResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStatus();
  if (f) {
    writer.writeBool(
      1,
      f
    );
  }
  f = message.getData();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.security_audit.SecuritySummary.serializeBinaryToWriter
    );
  }
};


/**
 * optional bool status = 1;
 * @return {boolean}
 */
proto.security_audit.GetIpSecuritySummaryResponse.prototype.getStatus = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 1, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.GetIpSecuritySummaryResponse} returns this
 */
proto.security_audit.GetIpSecuritySummaryResponse.prototype.setStatus = function(value) {
  return jspb.Message.setProto3BooleanField(this, 1, value);
};


/**
 * optional SecuritySummary data = 2;
 * @return {?proto.security_audit.SecuritySummary}
 */
proto.security_audit.GetIpSecuritySummaryResponse.prototype.getData = function() {
  return /** @type{?proto.security_audit.SecuritySummary} */ (
    jspb.Message.getWrapperField(this, proto.security_audit.SecuritySummary, 2));
};


/**
 * @param {?proto.security_audit.SecuritySummary|undefined} value
 * @return {!proto.security_audit.GetIpSecuritySummaryResponse} returns this
*/
proto.security_audit.GetIpSecuritySummaryResponse.prototype.setData = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.GetIpSecuritySummaryResponse} returns this
 */
proto.security_audit.GetIpSecuritySummaryResponse.prototype.clearData = function() {
  return this.setData(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.GetIpSecuritySummaryResponse.prototype.hasData = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.CreateSecurityAlertRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.CreateSecurityAlertRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.CreateSecurityAlertRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.CreateSecurityAlertRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
alert: (f = msg.getAlert()) && proto.security_audit.CreateSecurityAlertModel.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.CreateSecurityAlertRequest}
 */
proto.security_audit.CreateSecurityAlertRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.CreateSecurityAlertRequest;
  return proto.security_audit.CreateSecurityAlertRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.CreateSecurityAlertRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.CreateSecurityAlertRequest}
 */
proto.security_audit.CreateSecurityAlertRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = new proto.security_audit.CreateSecurityAlertModel;
      reader.readMessage(value,proto.security_audit.CreateSecurityAlertModel.deserializeBinaryFromReader);
      msg.setAlert(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.CreateSecurityAlertRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.CreateSecurityAlertRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.CreateSecurityAlertRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.CreateSecurityAlertRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getAlert();
  if (f != null) {
    writer.writeMessage(
      1,
      f,
      proto.security_audit.CreateSecurityAlertModel.serializeBinaryToWriter
    );
  }
};


/**
 * optional CreateSecurityAlertModel alert = 1;
 * @return {?proto.security_audit.CreateSecurityAlertModel}
 */
proto.security_audit.CreateSecurityAlertRequest.prototype.getAlert = function() {
  return /** @type{?proto.security_audit.CreateSecurityAlertModel} */ (
    jspb.Message.getWrapperField(this, proto.security_audit.CreateSecurityAlertModel, 1));
};


/**
 * @param {?proto.security_audit.CreateSecurityAlertModel|undefined} value
 * @return {!proto.security_audit.CreateSecurityAlertRequest} returns this
*/
proto.security_audit.CreateSecurityAlertRequest.prototype.setAlert = function(value) {
  return jspb.Message.setWrapperField(this, 1, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAlertRequest} returns this
 */
proto.security_audit.CreateSecurityAlertRequest.prototype.clearAlert = function() {
  return this.setAlert(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAlertRequest.prototype.hasAlert = function() {
  return jspb.Message.getField(this, 1) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.CreateSecurityAlertResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.CreateSecurityAlertResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.CreateSecurityAlertResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.CreateSecurityAlertResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
status: jspb.Message.getBooleanFieldWithDefault(msg, 1, false),
data: (f = msg.getData()) && proto.security_audit.SecurityAlertModel.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.CreateSecurityAlertResponse}
 */
proto.security_audit.CreateSecurityAlertResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.CreateSecurityAlertResponse;
  return proto.security_audit.CreateSecurityAlertResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.CreateSecurityAlertResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.CreateSecurityAlertResponse}
 */
proto.security_audit.CreateSecurityAlertResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setStatus(value);
      break;
    case 2:
      var value = new proto.security_audit.SecurityAlertModel;
      reader.readMessage(value,proto.security_audit.SecurityAlertModel.deserializeBinaryFromReader);
      msg.setData(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.CreateSecurityAlertResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.CreateSecurityAlertResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.CreateSecurityAlertResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.CreateSecurityAlertResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStatus();
  if (f) {
    writer.writeBool(
      1,
      f
    );
  }
  f = message.getData();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.security_audit.SecurityAlertModel.serializeBinaryToWriter
    );
  }
};


/**
 * optional bool status = 1;
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAlertResponse.prototype.getStatus = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 1, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.CreateSecurityAlertResponse} returns this
 */
proto.security_audit.CreateSecurityAlertResponse.prototype.setStatus = function(value) {
  return jspb.Message.setProto3BooleanField(this, 1, value);
};


/**
 * optional SecurityAlertModel data = 2;
 * @return {?proto.security_audit.SecurityAlertModel}
 */
proto.security_audit.CreateSecurityAlertResponse.prototype.getData = function() {
  return /** @type{?proto.security_audit.SecurityAlertModel} */ (
    jspb.Message.getWrapperField(this, proto.security_audit.SecurityAlertModel, 2));
};


/**
 * @param {?proto.security_audit.SecurityAlertModel|undefined} value
 * @return {!proto.security_audit.CreateSecurityAlertResponse} returns this
*/
proto.security_audit.CreateSecurityAlertResponse.prototype.setData = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAlertResponse} returns this
 */
proto.security_audit.CreateSecurityAlertResponse.prototype.clearData = function() {
  return this.setData(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAlertResponse.prototype.hasData = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.CreateSecurityAlertAutoIdRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.CreateSecurityAlertAutoIdRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
alertType: jspb.Message.getFieldWithDefault(msg, 1, 0),
severity: jspb.Message.getFieldWithDefault(msg, 2, 0),
message: jspb.Message.getFieldWithDefault(msg, 3, ""),
source: jspb.Message.getFieldWithDefault(msg, 4, ""),
affectedResource: (f = jspb.Message.getField(msg, 5)) == null ? undefined : f,
metadataJson: (f = jspb.Message.getField(msg, 6)) == null ? undefined : f
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.CreateSecurityAlertAutoIdRequest}
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.CreateSecurityAlertAutoIdRequest;
  return proto.security_audit.CreateSecurityAlertAutoIdRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.CreateSecurityAlertAutoIdRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.CreateSecurityAlertAutoIdRequest}
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {!proto.security_audit.AlertType} */ (reader.readEnum());
      msg.setAlertType(value);
      break;
    case 2:
      var value = /** @type {!proto.security_audit.AlertSeverity} */ (reader.readEnum());
      msg.setSeverity(value);
      break;
    case 3:
      var value = /** @type {string} */ (reader.readString());
      msg.setMessage(value);
      break;
    case 4:
      var value = /** @type {string} */ (reader.readString());
      msg.setSource(value);
      break;
    case 5:
      var value = /** @type {string} */ (reader.readString());
      msg.setAffectedResource(value);
      break;
    case 6:
      var value = /** @type {string} */ (reader.readString());
      msg.setMetadataJson(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.CreateSecurityAlertAutoIdRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.CreateSecurityAlertAutoIdRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getAlertType();
  if (f !== 0.0) {
    writer.writeEnum(
      1,
      f
    );
  }
  f = message.getSeverity();
  if (f !== 0.0) {
    writer.writeEnum(
      2,
      f
    );
  }
  f = message.getMessage();
  if (f.length > 0) {
    writer.writeString(
      3,
      f
    );
  }
  f = message.getSource();
  if (f.length > 0) {
    writer.writeString(
      4,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 5));
  if (f != null) {
    writer.writeString(
      5,
      f
    );
  }
  f = /** @type {string} */ (jspb.Message.getField(message, 6));
  if (f != null) {
    writer.writeString(
      6,
      f
    );
  }
};


/**
 * optional AlertType alert_type = 1;
 * @return {!proto.security_audit.AlertType}
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest.prototype.getAlertType = function() {
  return /** @type {!proto.security_audit.AlertType} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {!proto.security_audit.AlertType} value
 * @return {!proto.security_audit.CreateSecurityAlertAutoIdRequest} returns this
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest.prototype.setAlertType = function(value) {
  return jspb.Message.setProto3EnumField(this, 1, value);
};


/**
 * optional AlertSeverity severity = 2;
 * @return {!proto.security_audit.AlertSeverity}
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest.prototype.getSeverity = function() {
  return /** @type {!proto.security_audit.AlertSeverity} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {!proto.security_audit.AlertSeverity} value
 * @return {!proto.security_audit.CreateSecurityAlertAutoIdRequest} returns this
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest.prototype.setSeverity = function(value) {
  return jspb.Message.setProto3EnumField(this, 2, value);
};


/**
 * optional string message = 3;
 * @return {string}
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest.prototype.getMessage = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 3, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.CreateSecurityAlertAutoIdRequest} returns this
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest.prototype.setMessage = function(value) {
  return jspb.Message.setProto3StringField(this, 3, value);
};


/**
 * optional string source = 4;
 * @return {string}
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest.prototype.getSource = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 4, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.CreateSecurityAlertAutoIdRequest} returns this
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest.prototype.setSource = function(value) {
  return jspb.Message.setProto3StringField(this, 4, value);
};


/**
 * optional string affected_resource = 5;
 * @return {string}
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest.prototype.getAffectedResource = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 5, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.CreateSecurityAlertAutoIdRequest} returns this
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest.prototype.setAffectedResource = function(value) {
  return jspb.Message.setField(this, 5, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAlertAutoIdRequest} returns this
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest.prototype.clearAffectedResource = function() {
  return jspb.Message.setField(this, 5, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest.prototype.hasAffectedResource = function() {
  return jspb.Message.getField(this, 5) != null;
};


/**
 * optional string metadata_json = 6;
 * @return {string}
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest.prototype.getMetadataJson = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 6, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.CreateSecurityAlertAutoIdRequest} returns this
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest.prototype.setMetadataJson = function(value) {
  return jspb.Message.setField(this, 6, value);
};


/**
 * Clears the field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAlertAutoIdRequest} returns this
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest.prototype.clearMetadataJson = function() {
  return jspb.Message.setField(this, 6, undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAlertAutoIdRequest.prototype.hasMetadataJson = function() {
  return jspb.Message.getField(this, 6) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.CreateSecurityAlertAutoIdResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.CreateSecurityAlertAutoIdResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.CreateSecurityAlertAutoIdResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.CreateSecurityAlertAutoIdResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
status: jspb.Message.getBooleanFieldWithDefault(msg, 1, false),
data: (f = msg.getData()) && proto.security_audit.SecurityAlertModel.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.CreateSecurityAlertAutoIdResponse}
 */
proto.security_audit.CreateSecurityAlertAutoIdResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.CreateSecurityAlertAutoIdResponse;
  return proto.security_audit.CreateSecurityAlertAutoIdResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.CreateSecurityAlertAutoIdResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.CreateSecurityAlertAutoIdResponse}
 */
proto.security_audit.CreateSecurityAlertAutoIdResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setStatus(value);
      break;
    case 2:
      var value = new proto.security_audit.SecurityAlertModel;
      reader.readMessage(value,proto.security_audit.SecurityAlertModel.deserializeBinaryFromReader);
      msg.setData(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.CreateSecurityAlertAutoIdResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.CreateSecurityAlertAutoIdResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.CreateSecurityAlertAutoIdResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.CreateSecurityAlertAutoIdResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStatus();
  if (f) {
    writer.writeBool(
      1,
      f
    );
  }
  f = message.getData();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.security_audit.SecurityAlertModel.serializeBinaryToWriter
    );
  }
};


/**
 * optional bool status = 1;
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAlertAutoIdResponse.prototype.getStatus = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 1, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.CreateSecurityAlertAutoIdResponse} returns this
 */
proto.security_audit.CreateSecurityAlertAutoIdResponse.prototype.setStatus = function(value) {
  return jspb.Message.setProto3BooleanField(this, 1, value);
};


/**
 * optional SecurityAlertModel data = 2;
 * @return {?proto.security_audit.SecurityAlertModel}
 */
proto.security_audit.CreateSecurityAlertAutoIdResponse.prototype.getData = function() {
  return /** @type{?proto.security_audit.SecurityAlertModel} */ (
    jspb.Message.getWrapperField(this, proto.security_audit.SecurityAlertModel, 2));
};


/**
 * @param {?proto.security_audit.SecurityAlertModel|undefined} value
 * @return {!proto.security_audit.CreateSecurityAlertAutoIdResponse} returns this
*/
proto.security_audit.CreateSecurityAlertAutoIdResponse.prototype.setData = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.CreateSecurityAlertAutoIdResponse} returns this
 */
proto.security_audit.CreateSecurityAlertAutoIdResponse.prototype.clearData = function() {
  return this.setData(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.CreateSecurityAlertAutoIdResponse.prototype.hasData = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetSecurityAlertRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetSecurityAlertRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetSecurityAlertRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAlertRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
id: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetSecurityAlertRequest}
 */
proto.security_audit.GetSecurityAlertRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetSecurityAlertRequest;
  return proto.security_audit.GetSecurityAlertRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetSecurityAlertRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetSecurityAlertRequest}
 */
proto.security_audit.GetSecurityAlertRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetSecurityAlertRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetSecurityAlertRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetSecurityAlertRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAlertRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string id = 1;
 * @return {string}
 */
proto.security_audit.GetSecurityAlertRequest.prototype.getId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.GetSecurityAlertRequest} returns this
 */
proto.security_audit.GetSecurityAlertRequest.prototype.setId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetSecurityAlertResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetSecurityAlertResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetSecurityAlertResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAlertResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
status: jspb.Message.getBooleanFieldWithDefault(msg, 1, false),
data: (f = msg.getData()) && proto.security_audit.SecurityAlertModel.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetSecurityAlertResponse}
 */
proto.security_audit.GetSecurityAlertResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetSecurityAlertResponse;
  return proto.security_audit.GetSecurityAlertResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetSecurityAlertResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetSecurityAlertResponse}
 */
proto.security_audit.GetSecurityAlertResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setStatus(value);
      break;
    case 2:
      var value = new proto.security_audit.SecurityAlertModel;
      reader.readMessage(value,proto.security_audit.SecurityAlertModel.deserializeBinaryFromReader);
      msg.setData(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetSecurityAlertResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetSecurityAlertResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetSecurityAlertResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAlertResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStatus();
  if (f) {
    writer.writeBool(
      1,
      f
    );
  }
  f = message.getData();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.security_audit.SecurityAlertModel.serializeBinaryToWriter
    );
  }
};


/**
 * optional bool status = 1;
 * @return {boolean}
 */
proto.security_audit.GetSecurityAlertResponse.prototype.getStatus = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 1, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.GetSecurityAlertResponse} returns this
 */
proto.security_audit.GetSecurityAlertResponse.prototype.setStatus = function(value) {
  return jspb.Message.setProto3BooleanField(this, 1, value);
};


/**
 * optional SecurityAlertModel data = 2;
 * @return {?proto.security_audit.SecurityAlertModel}
 */
proto.security_audit.GetSecurityAlertResponse.prototype.getData = function() {
  return /** @type{?proto.security_audit.SecurityAlertModel} */ (
    jspb.Message.getWrapperField(this, proto.security_audit.SecurityAlertModel, 2));
};


/**
 * @param {?proto.security_audit.SecurityAlertModel|undefined} value
 * @return {!proto.security_audit.GetSecurityAlertResponse} returns this
*/
proto.security_audit.GetSecurityAlertResponse.prototype.setData = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.GetSecurityAlertResponse} returns this
 */
proto.security_audit.GetSecurityAlertResponse.prototype.clearData = function() {
  return this.setData(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.GetSecurityAlertResponse.prototype.hasData = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetUnresolvedAlertsBySeverityRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetUnresolvedAlertsBySeverityRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetUnresolvedAlertsBySeverityRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetUnresolvedAlertsBySeverityRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
severity: jspb.Message.getFieldWithDefault(msg, 1, 0),
page: jspb.Message.getFieldWithDefault(msg, 2, 0),
perPage: jspb.Message.getFieldWithDefault(msg, 3, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetUnresolvedAlertsBySeverityRequest}
 */
proto.security_audit.GetUnresolvedAlertsBySeverityRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetUnresolvedAlertsBySeverityRequest;
  return proto.security_audit.GetUnresolvedAlertsBySeverityRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetUnresolvedAlertsBySeverityRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetUnresolvedAlertsBySeverityRequest}
 */
proto.security_audit.GetUnresolvedAlertsBySeverityRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {!proto.security_audit.AlertSeverity} */ (reader.readEnum());
      msg.setSeverity(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setPage(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setPerPage(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetUnresolvedAlertsBySeverityRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetUnresolvedAlertsBySeverityRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetUnresolvedAlertsBySeverityRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetUnresolvedAlertsBySeverityRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getSeverity();
  if (f !== 0.0) {
    writer.writeEnum(
      1,
      f
    );
  }
  f = message.getPage();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
  f = message.getPerPage();
  if (f !== 0) {
    writer.writeInt64(
      3,
      f
    );
  }
};


/**
 * optional AlertSeverity severity = 1;
 * @return {!proto.security_audit.AlertSeverity}
 */
proto.security_audit.GetUnresolvedAlertsBySeverityRequest.prototype.getSeverity = function() {
  return /** @type {!proto.security_audit.AlertSeverity} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {!proto.security_audit.AlertSeverity} value
 * @return {!proto.security_audit.GetUnresolvedAlertsBySeverityRequest} returns this
 */
proto.security_audit.GetUnresolvedAlertsBySeverityRequest.prototype.setSeverity = function(value) {
  return jspb.Message.setProto3EnumField(this, 1, value);
};


/**
 * optional int64 page = 2;
 * @return {number}
 */
proto.security_audit.GetUnresolvedAlertsBySeverityRequest.prototype.getPage = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.GetUnresolvedAlertsBySeverityRequest} returns this
 */
proto.security_audit.GetUnresolvedAlertsBySeverityRequest.prototype.setPage = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


/**
 * optional int64 per_page = 3;
 * @return {number}
 */
proto.security_audit.GetUnresolvedAlertsBySeverityRequest.prototype.getPerPage = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.GetUnresolvedAlertsBySeverityRequest} returns this
 */
proto.security_audit.GetUnresolvedAlertsBySeverityRequest.prototype.setPerPage = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetUnresolvedAlertsBySeverityResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetUnresolvedAlertsBySeverityResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetUnresolvedAlertsBySeverityResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetUnresolvedAlertsBySeverityResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
status: jspb.Message.getBooleanFieldWithDefault(msg, 1, false),
data: (f = msg.getData()) && proto.security_audit.SecurityAlertPaginationModel.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetUnresolvedAlertsBySeverityResponse}
 */
proto.security_audit.GetUnresolvedAlertsBySeverityResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetUnresolvedAlertsBySeverityResponse;
  return proto.security_audit.GetUnresolvedAlertsBySeverityResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetUnresolvedAlertsBySeverityResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetUnresolvedAlertsBySeverityResponse}
 */
proto.security_audit.GetUnresolvedAlertsBySeverityResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setStatus(value);
      break;
    case 2:
      var value = new proto.security_audit.SecurityAlertPaginationModel;
      reader.readMessage(value,proto.security_audit.SecurityAlertPaginationModel.deserializeBinaryFromReader);
      msg.setData(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetUnresolvedAlertsBySeverityResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetUnresolvedAlertsBySeverityResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetUnresolvedAlertsBySeverityResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetUnresolvedAlertsBySeverityResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStatus();
  if (f) {
    writer.writeBool(
      1,
      f
    );
  }
  f = message.getData();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.security_audit.SecurityAlertPaginationModel.serializeBinaryToWriter
    );
  }
};


/**
 * optional bool status = 1;
 * @return {boolean}
 */
proto.security_audit.GetUnresolvedAlertsBySeverityResponse.prototype.getStatus = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 1, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.GetUnresolvedAlertsBySeverityResponse} returns this
 */
proto.security_audit.GetUnresolvedAlertsBySeverityResponse.prototype.setStatus = function(value) {
  return jspb.Message.setProto3BooleanField(this, 1, value);
};


/**
 * optional SecurityAlertPaginationModel data = 2;
 * @return {?proto.security_audit.SecurityAlertPaginationModel}
 */
proto.security_audit.GetUnresolvedAlertsBySeverityResponse.prototype.getData = function() {
  return /** @type{?proto.security_audit.SecurityAlertPaginationModel} */ (
    jspb.Message.getWrapperField(this, proto.security_audit.SecurityAlertPaginationModel, 2));
};


/**
 * @param {?proto.security_audit.SecurityAlertPaginationModel|undefined} value
 * @return {!proto.security_audit.GetUnresolvedAlertsBySeverityResponse} returns this
*/
proto.security_audit.GetUnresolvedAlertsBySeverityResponse.prototype.setData = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.GetUnresolvedAlertsBySeverityResponse} returns this
 */
proto.security_audit.GetUnresolvedAlertsBySeverityResponse.prototype.clearData = function() {
  return this.setData(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.GetUnresolvedAlertsBySeverityResponse.prototype.hasData = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetAlertsByTypeRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetAlertsByTypeRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetAlertsByTypeRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetAlertsByTypeRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
alertType: jspb.Message.getFieldWithDefault(msg, 1, 0),
page: jspb.Message.getFieldWithDefault(msg, 2, 0),
perPage: jspb.Message.getFieldWithDefault(msg, 3, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetAlertsByTypeRequest}
 */
proto.security_audit.GetAlertsByTypeRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetAlertsByTypeRequest;
  return proto.security_audit.GetAlertsByTypeRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetAlertsByTypeRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetAlertsByTypeRequest}
 */
proto.security_audit.GetAlertsByTypeRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {!proto.security_audit.AlertType} */ (reader.readEnum());
      msg.setAlertType(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setPage(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setPerPage(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetAlertsByTypeRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetAlertsByTypeRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetAlertsByTypeRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetAlertsByTypeRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getAlertType();
  if (f !== 0.0) {
    writer.writeEnum(
      1,
      f
    );
  }
  f = message.getPage();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
  f = message.getPerPage();
  if (f !== 0) {
    writer.writeInt64(
      3,
      f
    );
  }
};


/**
 * optional AlertType alert_type = 1;
 * @return {!proto.security_audit.AlertType}
 */
proto.security_audit.GetAlertsByTypeRequest.prototype.getAlertType = function() {
  return /** @type {!proto.security_audit.AlertType} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {!proto.security_audit.AlertType} value
 * @return {!proto.security_audit.GetAlertsByTypeRequest} returns this
 */
proto.security_audit.GetAlertsByTypeRequest.prototype.setAlertType = function(value) {
  return jspb.Message.setProto3EnumField(this, 1, value);
};


/**
 * optional int64 page = 2;
 * @return {number}
 */
proto.security_audit.GetAlertsByTypeRequest.prototype.getPage = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.GetAlertsByTypeRequest} returns this
 */
proto.security_audit.GetAlertsByTypeRequest.prototype.setPage = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


/**
 * optional int64 per_page = 3;
 * @return {number}
 */
proto.security_audit.GetAlertsByTypeRequest.prototype.getPerPage = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.GetAlertsByTypeRequest} returns this
 */
proto.security_audit.GetAlertsByTypeRequest.prototype.setPerPage = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetAlertsByTypeResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetAlertsByTypeResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetAlertsByTypeResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetAlertsByTypeResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
status: jspb.Message.getBooleanFieldWithDefault(msg, 1, false),
data: (f = msg.getData()) && proto.security_audit.SecurityAlertPaginationModel.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetAlertsByTypeResponse}
 */
proto.security_audit.GetAlertsByTypeResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetAlertsByTypeResponse;
  return proto.security_audit.GetAlertsByTypeResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetAlertsByTypeResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetAlertsByTypeResponse}
 */
proto.security_audit.GetAlertsByTypeResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setStatus(value);
      break;
    case 2:
      var value = new proto.security_audit.SecurityAlertPaginationModel;
      reader.readMessage(value,proto.security_audit.SecurityAlertPaginationModel.deserializeBinaryFromReader);
      msg.setData(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetAlertsByTypeResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetAlertsByTypeResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetAlertsByTypeResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetAlertsByTypeResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStatus();
  if (f) {
    writer.writeBool(
      1,
      f
    );
  }
  f = message.getData();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.security_audit.SecurityAlertPaginationModel.serializeBinaryToWriter
    );
  }
};


/**
 * optional bool status = 1;
 * @return {boolean}
 */
proto.security_audit.GetAlertsByTypeResponse.prototype.getStatus = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 1, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.GetAlertsByTypeResponse} returns this
 */
proto.security_audit.GetAlertsByTypeResponse.prototype.setStatus = function(value) {
  return jspb.Message.setProto3BooleanField(this, 1, value);
};


/**
 * optional SecurityAlertPaginationModel data = 2;
 * @return {?proto.security_audit.SecurityAlertPaginationModel}
 */
proto.security_audit.GetAlertsByTypeResponse.prototype.getData = function() {
  return /** @type{?proto.security_audit.SecurityAlertPaginationModel} */ (
    jspb.Message.getWrapperField(this, proto.security_audit.SecurityAlertPaginationModel, 2));
};


/**
 * @param {?proto.security_audit.SecurityAlertPaginationModel|undefined} value
 * @return {!proto.security_audit.GetAlertsByTypeResponse} returns this
*/
proto.security_audit.GetAlertsByTypeResponse.prototype.setData = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.GetAlertsByTypeResponse} returns this
 */
proto.security_audit.GetAlertsByTypeResponse.prototype.clearData = function() {
  return this.setData(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.GetAlertsByTypeResponse.prototype.hasData = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetAlertsBySourceRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetAlertsBySourceRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetAlertsBySourceRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetAlertsBySourceRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
source: jspb.Message.getFieldWithDefault(msg, 1, ""),
page: jspb.Message.getFieldWithDefault(msg, 2, 0),
perPage: jspb.Message.getFieldWithDefault(msg, 3, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetAlertsBySourceRequest}
 */
proto.security_audit.GetAlertsBySourceRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetAlertsBySourceRequest;
  return proto.security_audit.GetAlertsBySourceRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetAlertsBySourceRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetAlertsBySourceRequest}
 */
proto.security_audit.GetAlertsBySourceRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setSource(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setPage(value);
      break;
    case 3:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setPerPage(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetAlertsBySourceRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetAlertsBySourceRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetAlertsBySourceRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetAlertsBySourceRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getSource();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getPage();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
  f = message.getPerPage();
  if (f !== 0) {
    writer.writeInt64(
      3,
      f
    );
  }
};


/**
 * optional string source = 1;
 * @return {string}
 */
proto.security_audit.GetAlertsBySourceRequest.prototype.getSource = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.GetAlertsBySourceRequest} returns this
 */
proto.security_audit.GetAlertsBySourceRequest.prototype.setSource = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional int64 page = 2;
 * @return {number}
 */
proto.security_audit.GetAlertsBySourceRequest.prototype.getPage = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.GetAlertsBySourceRequest} returns this
 */
proto.security_audit.GetAlertsBySourceRequest.prototype.setPage = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};


/**
 * optional int64 per_page = 3;
 * @return {number}
 */
proto.security_audit.GetAlertsBySourceRequest.prototype.getPerPage = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 3, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.GetAlertsBySourceRequest} returns this
 */
proto.security_audit.GetAlertsBySourceRequest.prototype.setPerPage = function(value) {
  return jspb.Message.setProto3IntField(this, 3, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetAlertsBySourceResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetAlertsBySourceResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetAlertsBySourceResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetAlertsBySourceResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
status: jspb.Message.getBooleanFieldWithDefault(msg, 1, false),
data: (f = msg.getData()) && proto.security_audit.SecurityAlertPaginationModel.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetAlertsBySourceResponse}
 */
proto.security_audit.GetAlertsBySourceResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetAlertsBySourceResponse;
  return proto.security_audit.GetAlertsBySourceResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetAlertsBySourceResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetAlertsBySourceResponse}
 */
proto.security_audit.GetAlertsBySourceResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setStatus(value);
      break;
    case 2:
      var value = new proto.security_audit.SecurityAlertPaginationModel;
      reader.readMessage(value,proto.security_audit.SecurityAlertPaginationModel.deserializeBinaryFromReader);
      msg.setData(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetAlertsBySourceResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetAlertsBySourceResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetAlertsBySourceResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetAlertsBySourceResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStatus();
  if (f) {
    writer.writeBool(
      1,
      f
    );
  }
  f = message.getData();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.security_audit.SecurityAlertPaginationModel.serializeBinaryToWriter
    );
  }
};


/**
 * optional bool status = 1;
 * @return {boolean}
 */
proto.security_audit.GetAlertsBySourceResponse.prototype.getStatus = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 1, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.GetAlertsBySourceResponse} returns this
 */
proto.security_audit.GetAlertsBySourceResponse.prototype.setStatus = function(value) {
  return jspb.Message.setProto3BooleanField(this, 1, value);
};


/**
 * optional SecurityAlertPaginationModel data = 2;
 * @return {?proto.security_audit.SecurityAlertPaginationModel}
 */
proto.security_audit.GetAlertsBySourceResponse.prototype.getData = function() {
  return /** @type{?proto.security_audit.SecurityAlertPaginationModel} */ (
    jspb.Message.getWrapperField(this, proto.security_audit.SecurityAlertPaginationModel, 2));
};


/**
 * @param {?proto.security_audit.SecurityAlertPaginationModel|undefined} value
 * @return {!proto.security_audit.GetAlertsBySourceResponse} returns this
*/
proto.security_audit.GetAlertsBySourceResponse.prototype.setData = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.GetAlertsBySourceResponse} returns this
 */
proto.security_audit.GetAlertsBySourceResponse.prototype.clearData = function() {
  return this.setData(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.GetAlertsBySourceResponse.prototype.hasData = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.ResolveSecurityAlertRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.ResolveSecurityAlertRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.ResolveSecurityAlertRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.ResolveSecurityAlertRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
id: jspb.Message.getFieldWithDefault(msg, 1, ""),
resolvedBy: jspb.Message.getFieldWithDefault(msg, 2, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.ResolveSecurityAlertRequest}
 */
proto.security_audit.ResolveSecurityAlertRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.ResolveSecurityAlertRequest;
  return proto.security_audit.ResolveSecurityAlertRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.ResolveSecurityAlertRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.ResolveSecurityAlertRequest}
 */
proto.security_audit.ResolveSecurityAlertRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setId(value);
      break;
    case 2:
      var value = /** @type {string} */ (reader.readString());
      msg.setResolvedBy(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.ResolveSecurityAlertRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.ResolveSecurityAlertRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.ResolveSecurityAlertRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.ResolveSecurityAlertRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
  f = message.getResolvedBy();
  if (f.length > 0) {
    writer.writeString(
      2,
      f
    );
  }
};


/**
 * optional string id = 1;
 * @return {string}
 */
proto.security_audit.ResolveSecurityAlertRequest.prototype.getId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.ResolveSecurityAlertRequest} returns this
 */
proto.security_audit.ResolveSecurityAlertRequest.prototype.setId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};


/**
 * optional string resolved_by = 2;
 * @return {string}
 */
proto.security_audit.ResolveSecurityAlertRequest.prototype.getResolvedBy = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 2, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.ResolveSecurityAlertRequest} returns this
 */
proto.security_audit.ResolveSecurityAlertRequest.prototype.setResolvedBy = function(value) {
  return jspb.Message.setProto3StringField(this, 2, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.ResolveSecurityAlertResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.ResolveSecurityAlertResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.ResolveSecurityAlertResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.ResolveSecurityAlertResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
status: jspb.Message.getBooleanFieldWithDefault(msg, 1, false),
data: (f = msg.getData()) && proto.security_audit.SecurityAlertModel.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.ResolveSecurityAlertResponse}
 */
proto.security_audit.ResolveSecurityAlertResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.ResolveSecurityAlertResponse;
  return proto.security_audit.ResolveSecurityAlertResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.ResolveSecurityAlertResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.ResolveSecurityAlertResponse}
 */
proto.security_audit.ResolveSecurityAlertResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setStatus(value);
      break;
    case 2:
      var value = new proto.security_audit.SecurityAlertModel;
      reader.readMessage(value,proto.security_audit.SecurityAlertModel.deserializeBinaryFromReader);
      msg.setData(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.ResolveSecurityAlertResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.ResolveSecurityAlertResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.ResolveSecurityAlertResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.ResolveSecurityAlertResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStatus();
  if (f) {
    writer.writeBool(
      1,
      f
    );
  }
  f = message.getData();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.security_audit.SecurityAlertModel.serializeBinaryToWriter
    );
  }
};


/**
 * optional bool status = 1;
 * @return {boolean}
 */
proto.security_audit.ResolveSecurityAlertResponse.prototype.getStatus = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 1, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.ResolveSecurityAlertResponse} returns this
 */
proto.security_audit.ResolveSecurityAlertResponse.prototype.setStatus = function(value) {
  return jspb.Message.setProto3BooleanField(this, 1, value);
};


/**
 * optional SecurityAlertModel data = 2;
 * @return {?proto.security_audit.SecurityAlertModel}
 */
proto.security_audit.ResolveSecurityAlertResponse.prototype.getData = function() {
  return /** @type{?proto.security_audit.SecurityAlertModel} */ (
    jspb.Message.getWrapperField(this, proto.security_audit.SecurityAlertModel, 2));
};


/**
 * @param {?proto.security_audit.SecurityAlertModel|undefined} value
 * @return {!proto.security_audit.ResolveSecurityAlertResponse} returns this
*/
proto.security_audit.ResolveSecurityAlertResponse.prototype.setData = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.ResolveSecurityAlertResponse} returns this
 */
proto.security_audit.ResolveSecurityAlertResponse.prototype.clearData = function() {
  return this.setData(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.ResolveSecurityAlertResponse.prototype.hasData = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetSecurityAlertsPaginatedRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetSecurityAlertsPaginatedRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetSecurityAlertsPaginatedRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAlertsPaginatedRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
page: jspb.Message.getFieldWithDefault(msg, 1, 0),
perPage: jspb.Message.getFieldWithDefault(msg, 2, 0)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetSecurityAlertsPaginatedRequest}
 */
proto.security_audit.GetSecurityAlertsPaginatedRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetSecurityAlertsPaginatedRequest;
  return proto.security_audit.GetSecurityAlertsPaginatedRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetSecurityAlertsPaginatedRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetSecurityAlertsPaginatedRequest}
 */
proto.security_audit.GetSecurityAlertsPaginatedRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setPage(value);
      break;
    case 2:
      var value = /** @type {number} */ (reader.readInt64());
      msg.setPerPage(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetSecurityAlertsPaginatedRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetSecurityAlertsPaginatedRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetSecurityAlertsPaginatedRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAlertsPaginatedRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getPage();
  if (f !== 0) {
    writer.writeInt64(
      1,
      f
    );
  }
  f = message.getPerPage();
  if (f !== 0) {
    writer.writeInt64(
      2,
      f
    );
  }
};


/**
 * optional int64 page = 1;
 * @return {number}
 */
proto.security_audit.GetSecurityAlertsPaginatedRequest.prototype.getPage = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 1, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.GetSecurityAlertsPaginatedRequest} returns this
 */
proto.security_audit.GetSecurityAlertsPaginatedRequest.prototype.setPage = function(value) {
  return jspb.Message.setProto3IntField(this, 1, value);
};


/**
 * optional int64 per_page = 2;
 * @return {number}
 */
proto.security_audit.GetSecurityAlertsPaginatedRequest.prototype.getPerPage = function() {
  return /** @type {number} */ (jspb.Message.getFieldWithDefault(this, 2, 0));
};


/**
 * @param {number} value
 * @return {!proto.security_audit.GetSecurityAlertsPaginatedRequest} returns this
 */
proto.security_audit.GetSecurityAlertsPaginatedRequest.prototype.setPerPage = function(value) {
  return jspb.Message.setProto3IntField(this, 2, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetSecurityAlertsPaginatedResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetSecurityAlertsPaginatedResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetSecurityAlertsPaginatedResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAlertsPaginatedResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
status: jspb.Message.getBooleanFieldWithDefault(msg, 1, false),
data: (f = msg.getData()) && proto.security_audit.SecurityAlertPaginationModel.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetSecurityAlertsPaginatedResponse}
 */
proto.security_audit.GetSecurityAlertsPaginatedResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetSecurityAlertsPaginatedResponse;
  return proto.security_audit.GetSecurityAlertsPaginatedResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetSecurityAlertsPaginatedResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetSecurityAlertsPaginatedResponse}
 */
proto.security_audit.GetSecurityAlertsPaginatedResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setStatus(value);
      break;
    case 2:
      var value = new proto.security_audit.SecurityAlertPaginationModel;
      reader.readMessage(value,proto.security_audit.SecurityAlertPaginationModel.deserializeBinaryFromReader);
      msg.setData(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetSecurityAlertsPaginatedResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetSecurityAlertsPaginatedResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetSecurityAlertsPaginatedResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetSecurityAlertsPaginatedResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStatus();
  if (f) {
    writer.writeBool(
      1,
      f
    );
  }
  f = message.getData();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.security_audit.SecurityAlertPaginationModel.serializeBinaryToWriter
    );
  }
};


/**
 * optional bool status = 1;
 * @return {boolean}
 */
proto.security_audit.GetSecurityAlertsPaginatedResponse.prototype.getStatus = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 1, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.GetSecurityAlertsPaginatedResponse} returns this
 */
proto.security_audit.GetSecurityAlertsPaginatedResponse.prototype.setStatus = function(value) {
  return jspb.Message.setProto3BooleanField(this, 1, value);
};


/**
 * optional SecurityAlertPaginationModel data = 2;
 * @return {?proto.security_audit.SecurityAlertPaginationModel}
 */
proto.security_audit.GetSecurityAlertsPaginatedResponse.prototype.getData = function() {
  return /** @type{?proto.security_audit.SecurityAlertPaginationModel} */ (
    jspb.Message.getWrapperField(this, proto.security_audit.SecurityAlertPaginationModel, 2));
};


/**
 * @param {?proto.security_audit.SecurityAlertPaginationModel|undefined} value
 * @return {!proto.security_audit.GetSecurityAlertsPaginatedResponse} returns this
*/
proto.security_audit.GetSecurityAlertsPaginatedResponse.prototype.setData = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.GetSecurityAlertsPaginatedResponse} returns this
 */
proto.security_audit.GetSecurityAlertsPaginatedResponse.prototype.clearData = function() {
  return this.setData(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.GetSecurityAlertsPaginatedResponse.prototype.hasData = function() {
  return jspb.Message.getField(this, 2) != null;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetCriticalUnresolvedAlertsRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetCriticalUnresolvedAlertsRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetCriticalUnresolvedAlertsRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetCriticalUnresolvedAlertsRequest.toObject = function(includeInstance, msg) {
  var f, obj = {

  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetCriticalUnresolvedAlertsRequest}
 */
proto.security_audit.GetCriticalUnresolvedAlertsRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetCriticalUnresolvedAlertsRequest;
  return proto.security_audit.GetCriticalUnresolvedAlertsRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetCriticalUnresolvedAlertsRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetCriticalUnresolvedAlertsRequest}
 */
proto.security_audit.GetCriticalUnresolvedAlertsRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetCriticalUnresolvedAlertsRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetCriticalUnresolvedAlertsRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetCriticalUnresolvedAlertsRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetCriticalUnresolvedAlertsRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
};



/**
 * List of repeated fields within this message type.
 * @private {!Array<number>}
 * @const
 */
proto.security_audit.GetCriticalUnresolvedAlertsResponse.repeatedFields_ = [2];



if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetCriticalUnresolvedAlertsResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetCriticalUnresolvedAlertsResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetCriticalUnresolvedAlertsResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetCriticalUnresolvedAlertsResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
status: jspb.Message.getBooleanFieldWithDefault(msg, 1, false),
dataList: jspb.Message.toObjectList(msg.getDataList(),
    proto.security_audit.SecurityAlertModel.toObject, includeInstance)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetCriticalUnresolvedAlertsResponse}
 */
proto.security_audit.GetCriticalUnresolvedAlertsResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetCriticalUnresolvedAlertsResponse;
  return proto.security_audit.GetCriticalUnresolvedAlertsResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetCriticalUnresolvedAlertsResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetCriticalUnresolvedAlertsResponse}
 */
proto.security_audit.GetCriticalUnresolvedAlertsResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setStatus(value);
      break;
    case 2:
      var value = new proto.security_audit.SecurityAlertModel;
      reader.readMessage(value,proto.security_audit.SecurityAlertModel.deserializeBinaryFromReader);
      msg.addData(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetCriticalUnresolvedAlertsResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetCriticalUnresolvedAlertsResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetCriticalUnresolvedAlertsResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetCriticalUnresolvedAlertsResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStatus();
  if (f) {
    writer.writeBool(
      1,
      f
    );
  }
  f = message.getDataList();
  if (f.length > 0) {
    writer.writeRepeatedMessage(
      2,
      f,
      proto.security_audit.SecurityAlertModel.serializeBinaryToWriter
    );
  }
};


/**
 * optional bool status = 1;
 * @return {boolean}
 */
proto.security_audit.GetCriticalUnresolvedAlertsResponse.prototype.getStatus = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 1, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.GetCriticalUnresolvedAlertsResponse} returns this
 */
proto.security_audit.GetCriticalUnresolvedAlertsResponse.prototype.setStatus = function(value) {
  return jspb.Message.setProto3BooleanField(this, 1, value);
};


/**
 * repeated SecurityAlertModel data = 2;
 * @return {!Array<!proto.security_audit.SecurityAlertModel>}
 */
proto.security_audit.GetCriticalUnresolvedAlertsResponse.prototype.getDataList = function() {
  return /** @type{!Array<!proto.security_audit.SecurityAlertModel>} */ (
    jspb.Message.getRepeatedWrapperField(this, proto.security_audit.SecurityAlertModel, 2));
};


/**
 * @param {!Array<!proto.security_audit.SecurityAlertModel>} value
 * @return {!proto.security_audit.GetCriticalUnresolvedAlertsResponse} returns this
*/
proto.security_audit.GetCriticalUnresolvedAlertsResponse.prototype.setDataList = function(value) {
  return jspb.Message.setRepeatedWrapperField(this, 2, value);
};


/**
 * @param {!proto.security_audit.SecurityAlertModel=} opt_value
 * @param {number=} opt_index
 * @return {!proto.security_audit.SecurityAlertModel}
 */
proto.security_audit.GetCriticalUnresolvedAlertsResponse.prototype.addData = function(opt_value, opt_index) {
  return jspb.Message.addToRepeatedWrapperField(this, 2, opt_value, proto.security_audit.SecurityAlertModel, opt_index);
};


/**
 * Clears the list making it empty but non-null.
 * @return {!proto.security_audit.GetCriticalUnresolvedAlertsResponse} returns this
 */
proto.security_audit.GetCriticalUnresolvedAlertsResponse.prototype.clearDataList = function() {
  return this.setDataList([]);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.DeleteSecurityAlertRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.DeleteSecurityAlertRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.DeleteSecurityAlertRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.DeleteSecurityAlertRequest.toObject = function(includeInstance, msg) {
  var f, obj = {
id: jspb.Message.getFieldWithDefault(msg, 1, "")
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.DeleteSecurityAlertRequest}
 */
proto.security_audit.DeleteSecurityAlertRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.DeleteSecurityAlertRequest;
  return proto.security_audit.DeleteSecurityAlertRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.DeleteSecurityAlertRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.DeleteSecurityAlertRequest}
 */
proto.security_audit.DeleteSecurityAlertRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {string} */ (reader.readString());
      msg.setId(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.DeleteSecurityAlertRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.DeleteSecurityAlertRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.DeleteSecurityAlertRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.DeleteSecurityAlertRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getId();
  if (f.length > 0) {
    writer.writeString(
      1,
      f
    );
  }
};


/**
 * optional string id = 1;
 * @return {string}
 */
proto.security_audit.DeleteSecurityAlertRequest.prototype.getId = function() {
  return /** @type {string} */ (jspb.Message.getFieldWithDefault(this, 1, ""));
};


/**
 * @param {string} value
 * @return {!proto.security_audit.DeleteSecurityAlertRequest} returns this
 */
proto.security_audit.DeleteSecurityAlertRequest.prototype.setId = function(value) {
  return jspb.Message.setProto3StringField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.DeleteSecurityAlertResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.DeleteSecurityAlertResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.DeleteSecurityAlertResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.DeleteSecurityAlertResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
status: jspb.Message.getBooleanFieldWithDefault(msg, 1, false)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.DeleteSecurityAlertResponse}
 */
proto.security_audit.DeleteSecurityAlertResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.DeleteSecurityAlertResponse;
  return proto.security_audit.DeleteSecurityAlertResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.DeleteSecurityAlertResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.DeleteSecurityAlertResponse}
 */
proto.security_audit.DeleteSecurityAlertResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setStatus(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.DeleteSecurityAlertResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.DeleteSecurityAlertResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.DeleteSecurityAlertResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.DeleteSecurityAlertResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStatus();
  if (f) {
    writer.writeBool(
      1,
      f
    );
  }
};


/**
 * optional bool status = 1;
 * @return {boolean}
 */
proto.security_audit.DeleteSecurityAlertResponse.prototype.getStatus = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 1, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.DeleteSecurityAlertResponse} returns this
 */
proto.security_audit.DeleteSecurityAlertResponse.prototype.setStatus = function(value) {
  return jspb.Message.setProto3BooleanField(this, 1, value);
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetAlertStatisticsRequest.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetAlertStatisticsRequest.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetAlertStatisticsRequest} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetAlertStatisticsRequest.toObject = function(includeInstance, msg) {
  var f, obj = {

  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetAlertStatisticsRequest}
 */
proto.security_audit.GetAlertStatisticsRequest.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetAlertStatisticsRequest;
  return proto.security_audit.GetAlertStatisticsRequest.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetAlertStatisticsRequest} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetAlertStatisticsRequest}
 */
proto.security_audit.GetAlertStatisticsRequest.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetAlertStatisticsRequest.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetAlertStatisticsRequest.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetAlertStatisticsRequest} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetAlertStatisticsRequest.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
};





if (jspb.Message.GENERATE_TO_OBJECT) {
/**
 * Creates an object representation of this proto.
 * Field names that are reserved in JavaScript and will be renamed to pb_name.
 * Optional fields that are not set will be set to undefined.
 * To access a reserved field use, foo.pb_<name>, eg, foo.pb_default.
 * For the list of reserved names please see:
 *     net/proto2/compiler/js/internal/generator.cc#kKeyword.
 * @param {boolean=} opt_includeInstance Deprecated. whether to include the
 *     JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @return {!Object}
 */
proto.security_audit.GetAlertStatisticsResponse.prototype.toObject = function(opt_includeInstance) {
  return proto.security_audit.GetAlertStatisticsResponse.toObject(opt_includeInstance, this);
};


/**
 * Static version of the {@see toObject} method.
 * @param {boolean|undefined} includeInstance Deprecated. Whether to include
 *     the JSPB instance for transitional soy proto support:
 *     http://goto/soy-param-migration
 * @param {!proto.security_audit.GetAlertStatisticsResponse} msg The msg instance to transform.
 * @return {!Object}
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetAlertStatisticsResponse.toObject = function(includeInstance, msg) {
  var f, obj = {
status: jspb.Message.getBooleanFieldWithDefault(msg, 1, false),
data: (f = msg.getData()) && proto.security_audit.AlertStatistics.toObject(includeInstance, f)
  };

  if (includeInstance) {
    obj.$jspbMessageInstance = msg;
  }
  return obj;
};
}


/**
 * Deserializes binary data (in protobuf wire format).
 * @param {jspb.ByteSource} bytes The bytes to deserialize.
 * @return {!proto.security_audit.GetAlertStatisticsResponse}
 */
proto.security_audit.GetAlertStatisticsResponse.deserializeBinary = function(bytes) {
  var reader = new jspb.BinaryReader(bytes);
  var msg = new proto.security_audit.GetAlertStatisticsResponse;
  return proto.security_audit.GetAlertStatisticsResponse.deserializeBinaryFromReader(msg, reader);
};


/**
 * Deserializes binary data (in protobuf wire format) from the
 * given reader into the given message object.
 * @param {!proto.security_audit.GetAlertStatisticsResponse} msg The message object to deserialize into.
 * @param {!jspb.BinaryReader} reader The BinaryReader to use.
 * @return {!proto.security_audit.GetAlertStatisticsResponse}
 */
proto.security_audit.GetAlertStatisticsResponse.deserializeBinaryFromReader = function(msg, reader) {
  while (reader.nextField()) {
    if (reader.isEndGroup()) {
      break;
    }
    var field = reader.getFieldNumber();
    switch (field) {
    case 1:
      var value = /** @type {boolean} */ (reader.readBool());
      msg.setStatus(value);
      break;
    case 2:
      var value = new proto.security_audit.AlertStatistics;
      reader.readMessage(value,proto.security_audit.AlertStatistics.deserializeBinaryFromReader);
      msg.setData(value);
      break;
    default:
      reader.skipField();
      break;
    }
  }
  return msg;
};


/**
 * Serializes the message to binary data (in protobuf wire format).
 * @return {!Uint8Array}
 */
proto.security_audit.GetAlertStatisticsResponse.prototype.serializeBinary = function() {
  var writer = new jspb.BinaryWriter();
  proto.security_audit.GetAlertStatisticsResponse.serializeBinaryToWriter(this, writer);
  return writer.getResultBuffer();
};


/**
 * Serializes the given message to binary data (in protobuf wire
 * format), writing to the given BinaryWriter.
 * @param {!proto.security_audit.GetAlertStatisticsResponse} message
 * @param {!jspb.BinaryWriter} writer
 * @suppress {unusedLocalVariables} f is only used for nested messages
 */
proto.security_audit.GetAlertStatisticsResponse.serializeBinaryToWriter = function(message, writer) {
  var f = undefined;
  f = message.getStatus();
  if (f) {
    writer.writeBool(
      1,
      f
    );
  }
  f = message.getData();
  if (f != null) {
    writer.writeMessage(
      2,
      f,
      proto.security_audit.AlertStatistics.serializeBinaryToWriter
    );
  }
};


/**
 * optional bool status = 1;
 * @return {boolean}
 */
proto.security_audit.GetAlertStatisticsResponse.prototype.getStatus = function() {
  return /** @type {boolean} */ (jspb.Message.getBooleanFieldWithDefault(this, 1, false));
};


/**
 * @param {boolean} value
 * @return {!proto.security_audit.GetAlertStatisticsResponse} returns this
 */
proto.security_audit.GetAlertStatisticsResponse.prototype.setStatus = function(value) {
  return jspb.Message.setProto3BooleanField(this, 1, value);
};


/**
 * optional AlertStatistics data = 2;
 * @return {?proto.security_audit.AlertStatistics}
 */
proto.security_audit.GetAlertStatisticsResponse.prototype.getData = function() {
  return /** @type{?proto.security_audit.AlertStatistics} */ (
    jspb.Message.getWrapperField(this, proto.security_audit.AlertStatistics, 2));
};


/**
 * @param {?proto.security_audit.AlertStatistics|undefined} value
 * @return {!proto.security_audit.GetAlertStatisticsResponse} returns this
*/
proto.security_audit.GetAlertStatisticsResponse.prototype.setData = function(value) {
  return jspb.Message.setWrapperField(this, 2, value);
};


/**
 * Clears the message field making it undefined.
 * @return {!proto.security_audit.GetAlertStatisticsResponse} returns this
 */
proto.security_audit.GetAlertStatisticsResponse.prototype.clearData = function() {
  return this.setData(undefined);
};


/**
 * Returns whether this field is set.
 * @return {boolean}
 */
proto.security_audit.GetAlertStatisticsResponse.prototype.hasData = function() {
  return jspb.Message.getField(this, 2) != null;
};


/**
 * @enum {number}
 */
proto.security_audit.AlertType = {
  ALERT_TYPE_UNSPECIFIED: 0,
  AUTHENTICATION_FAILURE: 1,
  INJECTION_ATTEMPT: 2,
  RATE_LIMIT_EXCEEDED: 3,
  SUSPICIOUS_ACTIVITY: 4,
  PRIVILEGE_ESCALATION: 5,
  DATA_BREACH_ATTEMPT: 6,
  UNAUTHORIZED_ACCESS: 7,
  MALFORMED_REQUEST: 8,
  BRUTE_FORCE_ATTACK: 9,
  SESSION_HIJACKING: 10
};

/**
 * @enum {number}
 */
proto.security_audit.AlertSeverity = {
  ALERT_SEVERITY_UNSPECIFIED: 0,
  LOW: 1,
  MEDIUM: 2,
  HIGH: 3,
  CRITICAL: 4
};

/**
 * @enum {number}
 */
proto.security_audit.SecurityEventType = {
  SECURITY_EVENT_TYPE_UNSPECIFIED: 0,
  AUTHENTICATION_SUCCESS: 1,
  AUTHENTICATION_FAILURE_EVENT: 2,
  INJECTION_ATTEMPT_EVENT: 3,
  RATE_LIMIT_EXCEEDED_EVENT: 4,
  SUSPICIOUS_ACTIVITY_EVENT: 5,
  SECURITY_VIOLATION_EVENT: 6
};

goog.object.extend(exports, proto.security_audit);

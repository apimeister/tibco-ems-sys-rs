#![deny(missing_docs)]

//! Bindings for the EMS C library.

use std::ffi::c_void;
use std::os::raw::c_char;
use std::os::raw::c_uchar;

#[allow(dead_code)]
extern "C" {
  /// Create a new error context object.
  pub fn tibemsErrorContext_Create(
    errorContext: *mut c_void) -> tibems_status;
  /// Retrieve any available detailed error string associated with the last EMS call.
  pub fn tibemsErrorContext_GetLastErrorString(
    errorContext: *mut c_void,
    str: *mut c_char) -> tibems_status;
  /// Create a new SSL parameter object.
  pub fn tibemsSSLParams_Create() -> *mut c_void;
  /// Destroy an SSL parameter object.
  pub fn tibemsSSLParams_Destroy(sslParams: *mut c_void);
  /// Get the text string corresponding to a status code.
  pub fn tibemsStatus_GetText(status: tibems_status) -> *const c_char;
  /// Get the total number of queues in the server.
  pub fn tibemsServerInfo_GetQueueCount(
    serverInfo: usize,
    count: *mut usize) -> tibems_status;
  /// Create a tibemsDestinationInfo object.
  pub fn tibemsDestinationInfo_Create(
    destInfo: *mut usize,
    destName: *const c_char,
    destType: usize) -> tibems_status;
  /// Destroy a tibemsDestinationInfo object.
  pub fn tibemsDestinationInfo_Destroy(
    destInfo: usize) -> tibems_status;
  /// Get the total number of pending messages for this destination.
  pub fn tibemsDestinationInfo_GetPendingMessageCount(
    destInfo: usize,
    count: *mut i64 ) -> tibems_status;
  /// Get the number of active consumers on this destination.
  pub fn tibemsDestinationInfo_GetConsumerCount(
    destInfo: usize,
    count: &mut usize) -> tibems_status;
  /// Get the overflow policy for this destination.
  pub fn tibemsDestinationInfo_GetOverflowPolicy(
    destInfo: usize,
    overflowPolicy: *mut usize) -> tibems_status;
  /// Get the name of this destination.
  pub fn tibemsDestinationInfo_GetName(
    destInfo: usize,
    name: *const c_char,
    name_len: usize) -> tibems_status;
  /// Get the first object in a collection.
  pub fn tibemsCollection_GetFirst(
    collection: usize,
    collection_ptr: *mut usize) -> tibems_status;
  /// Get the next object in a collection.
  pub fn tibemsCollection_GetNext(
    collection: usize,
    collection_ptr: *mut usize) -> tibems_status;
  /// Destroy a collection.
  pub fn tibemsCollection_Destroy(
    collection: usize) -> tibems_status;
  /// Get the current number of subscriptions for this topic.
  pub fn tibemsTopicInfo_GetSubscriptionCount(
    topicInfo: usize,
    count: *mut i64) -> tibems_status;
  /// Get the current number of durable subscriptions for this topic.
  pub fn tibemsTopicInfo_GetDurableSubscriptionCount(
    topicInfo: usize,
    count: *mut i64) -> tibems_status;
  /// Create a connection factory.
  pub fn tibemsConnectionFactory_Create() -> *mut tibemsConnectionFactory;
  /// Destroy a connection factory object.
  pub fn tibemsConnectionFactory_Destroy(factory: *mut tibemsConnectionFactory) -> tibems_status;
  /// Set the server URL.
  pub fn tibemsConnectionFactory_SetServerURL(
    factory: *mut tibemsConnectionFactory,
    url: *const c_char) -> tibems_status;
  /// Set a connection factory’s username.
  pub fn tibemsConnectionFactory_SetUserName(
    factory: *mut tibemsConnectionFactory,
    username: *const c_char) -> tibems_status;
  /// Set the password used by the connection factory to authenticate itself with the EMS Server.
  pub fn tibemsConnectionFactory_SetUserPassword(
    factory: *mut tibemsConnectionFactory,
    password: *const c_char) -> tibems_status;
  /// Create a connection object.
  pub fn tibemsConnectionFactory_CreateConnection(
    factory: *mut tibemsConnectionFactory,
    connection: *mut usize,
    username: *const c_char,
    password: *const c_char) -> tibems_status;
  /// Start delivering inbound messages
  pub fn tibemsConnection_Start(
    connection: usize) -> tibems_status;
  /// Create a session object.
  pub fn tibemsConnection_CreateSession(
    connection: usize,
    session: *mut usize,
    transacted: tibems_bool,
    acknowledgeMode: tibemsAcknowledgeMode) -> tibems_status;
  /// Create a destination object.
  pub fn tibemsDestination_Create(
    destination: *mut usize,
    destType: tibemsDestinationType,
    name: *const c_char) -> tibems_status;
  /// Get the active URL of a connection.
  pub fn tibemsConnection_GetActiveURL(
    connection: usize,
    serverURL: *const *const c_char) -> tibems_status;
  /// Destroy a destination object.
  pub fn tibemsDestination_Destroy(
    destination: usize) -> tibems_status;
  /// Get the type of a destination object.
  pub fn tibemsDestination_GetType(
    destination: usize,
    destination_type: *mut tibemsDestinationType) -> tibems_status;
  /// Get the name of a destination object.
  pub fn tibemsDestination_GetName(
    destination: usize,
    name: *const c_char,
    name_len: usize) -> tibems_status;
  /// Create an independent copy of a destination object.
  pub fn tibemsDestination_Copy(
    destination: usize,
    copy: *mut usize) -> tibems_status;
  }

//
// session
//
#[allow(dead_code)]
extern "C" {
  /// Close a session; reclaim resources.
  pub fn tibemsSession_Close(session: usize) -> tibems_status;
  /// Commit the open transaction.
  pub fn tibemsSession_Commit(
    session: usize) -> tibems_status;
  /// Create a message consumer
  pub fn tibemsSession_CreateConsumer(
    session: usize,
    consumer: *mut usize,
    destination: usize,
    messageSelector: *const c_char,
    noLocal: tibems_bool) -> tibems_status;
  /// Create a message.
  pub fn tibemsSession_CreateMessage(
    session: usize,
    textMsg: *mut usize) -> tibems_status;

  /// Create a message producer.
  pub fn tibemsSession_CreateProducer(
    session: usize,
    producer: *mut usize,
    destination: usize ) -> tibems_status;
  /// create a shared topic consumer
  pub fn tibemsSession_CreateSharedConsumer(
    session: usize,
    consumer: *mut usize,
    topic: usize,
    sharedSubscriptionName: *const c_char,
    messageSelector: *const c_char) -> tibems_status;
  /// create a shared durable topic consumer
  pub fn tibemsSession_CreateSharedDurableConsumer(
    session: usize,
    consumer: *mut usize,
    topic: usize,
    durableName: *const c_char,
    messageSelector: *const c_char) -> tibems_status;
  /// Create a temporary queue.
  pub fn tibemsSession_CreateTemporaryQueue(
    session: usize,
    tmpQueue: *mut usize) -> tibems_status;
  /// Create a temporary topic.
  pub fn tibemsSession_CreateTemporaryTopic(
    session: usize,
    tmpTopic: *mut usize) -> tibems_status;
  /// Delete a temporary queue.
  pub fn tibemsSession_DeleteTemporaryQueue(
    session: usize,
    tmpQueue: usize) -> tibems_status;
  /// Delete a temporary topic.
  pub fn tibemsSession_DeleteTemporaryTopic(
    session: usize,
    tmpTopic: usize) -> tibems_status;

  /// Create a text message.
  pub fn tibemsSession_CreateTextMessage(
    session: usize,
    textMsg: *mut tibemsMsg) -> tibems_status;
}

//
// producer
//
#[allow(dead_code)]
extern "C" {
  /// Send a message.
  pub fn tibemsMsgProducer_Send(
    msgProducer: usize,
    message: usize) -> tibems_status;
  /// Send a message.
  pub fn tibemsMsgProducer_SendEx(
    msgProducer: usize,
    message: usize,
    deliveryMode: tibemsDeliveryMode,
    priority: i32,
    timeToLive: i64) -> tibems_status;
  /// Send a message.
  pub fn tibemsMsgProducer_SendToDestination(
    msgProducer: usize,
    destination: usize,
    message: usize) -> tibems_status;
  /// Send a message.
  pub fn tibemsMsgProducer_SendToDestinationEx(
    msgProducer: usize,
    destination: usize,
    message: usize,
    deliveryMode: tibemsDeliveryMode,
    priority: i32,
    timeToLive: i64) -> tibems_status;
  /// Destroy the producer object; reclaim resources.
  pub fn tibemsMsgProducer_Close(
    msgProducer: usize) -> tibems_status;
  /// Set the time-to-live property of a producer object.
  pub fn tibemsMsgProducer_SetTimeToLive(msgProducer: usize, timeToLive: i64) -> tibems_status;
  /// Create a queue object.
  pub fn tibemsQueue_Create(
    queue: *mut usize,
    queueName: *const c_char) -> tibems_status;
  /// Create a new EMS lookup context object.
  pub fn tibemsLookupContext_Create(
    context: *mut tibemsLookupContext,
    brokerURL: *const c_char,
    username: *const c_char,
    password: *const c_char) -> tibems_status;
  /// Look up an object in the naming server.
  pub fn tibemsLookupContext_LookupDestination(
    context: tibemsLookupContext,
    name: *const c_char,
    destination: *mut usize) -> tibems_status;
}

//
// consumer
//
#[allow(dead_code)]
extern "C" {
  /// Receive a message (synchronous).
  pub fn tibemsMsgConsumer_Receive(
    msgConsumer: usize,
    message: *mut usize) -> tibems_status;
  /// Receive a message (synchronous, blocks up to a time limit).
  pub fn tibemsMsgConsumer_ReceiveTimeout(
    msgConsumer: usize,
    message: *mut usize,
    timeout: i64) -> tibems_status;
  /// Close a message consumer and release associated storage.
  pub fn tibemsMsgConsumer_Close(msgConsumer: usize) -> tibems_status;
}

//
// messages
//
#[allow(dead_code)]
extern "C" {
  /// Create a bytes message.
  pub fn tibemsBytesMsg_Create(
    message: *mut usize) -> tibems_status;
  /// Get the body length (in bytes) of a bytes message.
  pub fn tibemsBytesMsg_GetBodyLength(
    message: usize,
    return_length: *mut i32) -> tibems_status;
  /// Get the body data of a bytes message.
  pub fn tibemsBytesMsg_GetBytes(
    message: usize,
    bytes: *const *const c_uchar,
    byteSize: *mut u32) -> tibems_status;
  /// Set the body data of a bytes message from a byte sequence.
  pub fn tibemsBytesMsg_SetBytes(
    message: usize,
    bytes: *const c_void,
    byteSize: u32) -> tibems_status;
  
  /// Create a map message.
  pub fn tibemsMapMsg_Create(
    message: *mut usize) -> tibems_status;
  /// Get an enumeration of the field names in a map message.
  pub fn tibemsMapMsg_GetMapNames(
    message: usize,
    enumeration: *mut usize) -> tibems_status;
  /// Get data values from a map message.
  pub fn tibemsMapMsg_GetBoolean(
    message: usize,
    name: *const c_char,
    value: *mut tibems_bool) -> tibems_status;
  /// Get data values from a map message.
  pub fn tibemsMapMsg_GetByte(
    message: usize,
    name: *const c_char,
    value: *mut c_void) -> tibems_status;
  /// Get data values from a map message.
  pub fn tibemsMapMsg_GetBytes(
    message: usize,
    name: *const c_char,
    bytes: *const *const c_char,
    bytesSize: *mut u32) -> tibems_status;
  /// Get data values from a map message.
  pub fn tibemsMapMsg_GetChar(
    message: usize,
    name: *const c_char,
    value: *mut c_void) -> tibems_status;
  /// Get data values from a map message.
  pub fn tibemsMapMsg_GetDouble(
    message: usize,
    name: *const c_char,
    value: *mut f64) -> tibems_status;
  /// Get data values from a map message.
  pub fn tibemsMapMsg_GetField(
    message: usize,
    name: *const c_char,
    value: *mut tibemsMsgField) -> tibems_status;
  /// Get data values from a map message.
  pub fn tibemsMapMsg_GetFloat(
    message: usize,
    name: *const c_char,
    value: *mut f32) -> tibems_status;
  /// Get data values from a map message.
  pub fn tibemsMapMsg_GetInt(
    message: usize,
    name: *const c_char,
    value: *mut i32) -> tibems_status;
  /// Get data values from a map message.
  pub fn tibemsMapMsg_GetLong(
    message: usize,
    name: *const c_char,
    value: *mut i64) -> tibems_status;
  /// Get data values from a map message.
  pub fn tibemsMapMsg_GetMapMsg(
    message: usize,
    name: *const c_char,
    value: *mut usize) -> tibems_status;
  /// Get data values from a map message.
  pub fn tibemsMapMsg_GetShort(
    message: usize,
    name: *const c_char,
    value: *mut i16) -> tibems_status;
  /// Get data values from a map message.
  pub fn tibemsMapMsg_GetString(
    message: usize,
    name: *const c_char,
    value: *mut *mut c_char) -> tibems_status;
  /// Test if a named pair exists.
  pub fn tibemsMapMsg_ItemExists(
    message: usize,
    name: *const c_char,
    exists: *mut tibems_bool) -> tibems_status;
  /// Set a byte array as a named value in a map message.
  pub fn tibemsMapMsg_SetBytes(
    message: usize,
    name: *const c_char,
    bytes: *mut c_void,
    bytesSize: u64) -> tibems_status;
  /// Set a byte array as a named value in a map message.
  pub fn tibemsMapMsg_SetReferencedBytes(
    message: usize,
    name: *const c_char,
    bytes: *mut c_void,
    bytesSize: u64) -> tibems_status;
  /// Set a name-value pair in a map message.
  pub fn tibemsMapMsg_SetBoolean(
      message: usize,
      name: *const c_char,
      value: tibems_bool) -> tibems_status;
  /// Set a name-value pair in a map message.
  pub fn tibemsMapMsg_SetByte(
      message: usize,
      name: *const c_char,
      value: *const c_void) -> tibems_status;
  /// Set a name-value pair in a map message.
  pub fn tibemsMapMsg_SetChar(
      message: usize,
      name: *const c_char,
      value: *const c_void) -> tibems_status;
  /// Set a name-value pair in a map message.
  pub fn tibemsMapMsg_SetDouble(
      message: usize,
      name: *const c_char,
      value: f64) -> tibems_status;
  /// Set a name-value pair in a map message.
  pub fn tibemsMapMsg_SetFloat(
      message: usize,
      name: *const c_char,
      value: f32) -> tibems_status;
  /// Set a name-value pair in a map message.
  pub fn tibemsMapMsg_SetInt(
      message: usize,
      name: *const c_char,
      value: i32) -> tibems_status;
  /// Set a name-value pair in a map message.
  pub fn tibemsMapMsg_SetLong(
      message: usize,
      name: *const c_char,
      value: i64) -> tibems_status;
  /// Set a name-value pair in a map message.
  pub fn tibemsMapMsg_SetShort(
      message: usize,
      name: *const c_char,
      value: i16) -> tibems_status;
  /// Set a name-value pair in a map message.
  pub fn tibemsMapMsg_SetString(
      message: usize,
      name: *const c_char,
      value: *const c_char) -> tibems_status;

  /// Acknowledge messages.
  pub fn tibemsMsg_Acknowledge(
    message: usize) -> tibems_status;
  /// Create a message object.
  pub fn tibemsMsg_Create(
    message: *mut usize) -> tibems_status;
  /// Destroy a message.
  pub fn tibemsMsg_Destroy(
    message: usize) -> tibems_status;
  /// Get the body type of a message.
  pub fn tibemsMsg_GetBodyType(
    message: usize,
    bodyType: *mut tibemsMsgType) -> tibems_status;
  /// Get the value of a message property.
  pub fn tibemsMsg_GetBooleanProperty(
    message: usize,
    name: *const c_char,
    value: *mut tibems_bool) -> tibems_status;
  /// Get the value of a message property.
  pub fn tibemsMsg_GetByteProperty(
    message: usize, 
    name: *const c_char,
    value: *mut c_char) -> tibems_status;
  /// Get the correlation ID header of a message.
  pub fn tibemsMsg_GetCorrelationID(
    message: usize,
    value: *const *const c_char) -> tibems_status;
  /// Get the destination header from a message.
  pub fn tibemsMsg_GetDestination(
    message: usize,
    value: *mut usize) -> tibems_status;
  /// Get the message ID header from a message.
  pub fn tibemsMsg_GetMessageID(
    message: usize,
    value: *const *const c_char ) -> tibems_status;
  /// Get a list of property names from a message.
  pub fn tibemsMsg_GetPropertyNames(
    message: usize,
    enumeration: *mut usize) -> tibems_status;
  /// Get the reply-to header from a message.
  pub fn tibemsMsg_GetReplyTo(
    message: usize,
    value: *mut usize) -> tibems_status;
  /// Get the value of a message property.
  pub fn tibemsMsg_GetStringProperty(
    message: usize,
    name: *const c_char,
    value: *const *const c_char) -> tibems_status;
  /// Get the timestamp header from a message.
  pub fn tibemsMsg_GetTimestamp(
    message: usize,
    value: *mut i64) -> tibems_status;
  /// Get the type header of a message.
  pub fn tibemsMsg_GetType(
    message: usize,
    value: *const *const c_char) -> tibems_status;
  /// Test whether a named property has been set on a message.
  pub fn tibemsMsg_PropertyExists(
    message: usize,
    name: *const c_char,
    result: *mut tibems_bool) -> tibems_status;
  /// Recover a single message.
  pub fn tibemsMsg_Recover(
    message: usize) -> tibems_status;
  /// Set the value of a message property.
  pub fn tibemsMsg_SetBooleanProperty(
    message: usize,
    name: *const c_char,
    value: tibems_bool) -> tibems_status;
  /// Set the value of a message property.
  pub fn tibemsMsg_SetByteProperty(
    message: usize,
    name: *const c_char,
    value: c_char) -> tibems_status;  
  /// Set the correlation ID header of a message.
  pub fn tibemsMsg_SetCorrelationID(
    message: usize,
    value: *const c_char) -> tibems_status;
  /// Set the destination header of a message. 
  pub fn tibemsMsg_SetDestination(
    message: usize,
    value: usize) -> tibems_status;
  /// Set the value of a message property.
  pub fn tibemsMsg_SetDoubleProperty(
    message: usize,
    name: *const c_char,
    value: f64) -> tibems_status; 
  /// Set the expiration header of a message.
  pub fn tibemsMsg_SetExpiration(
    message: usize,
    value: i64) -> tibems_status;
  /// Set the value of a message property.
  pub fn tibemsMsg_SetFloatProperty(
    message: usize,
    name: *const c_char,
    value: f32) -> tibems_status;
  /// Set the value of a message property.
  pub fn tibemsMsg_SetIntProperty(
    message: usize,
    name: *const c_char,
    value: i32) -> tibems_status;
  /// Set the value of a message property.
  pub fn tibemsMsg_SetLongProperty(
    message: usize,
    name: *const c_char,
    value: i64) -> tibems_status;
  /// Set the reply-to header of a message.
  pub fn tibemsMsg_SetReplyTo(
    message: usize,
    value: usize) -> tibems_status;
  /// Set the value of a message property.
  pub fn tibemsMsg_SetShortProperty(
    message: usize,
    name: *const c_char,
    value: i16) -> tibems_status; 
  /// Set the value of a message property.
  pub fn tibemsMsg_SetStringProperty(
    message: usize, 
    name: *const c_char,
    value: *const c_char) -> tibems_status;
  /// Set the type header of a message.
  pub fn tibemsMsg_SetType(
    message: usize,
    value: *const c_char) -> tibems_status;
  /// Destroy a message enumerator.
  pub fn tibemsMsgEnum_Destroy(
    enumeration: usize) -> tibems_status;
  /// Get the next item from a message enumerator.
  pub fn tibemsMsgEnum_GetNextName(
    enumeration: usize,
    name: *const *const c_char) -> tibems_status;

  /// Create an object message.
  pub fn tibemsObjectMsg_Create(
    message: *mut usize) -> tibems_status;
  /// Get the byte sequence representing a serialized object from a message. 
  pub fn tibemsObjectMsg_GetObjectBytes(
    message: usize,
    bytes: *const *const c_uchar,
    byteSize: *mut u32) -> tibems_status;
  /// Set the byte sequence of an object message. 
  pub fn tibemsObjectMsg_SetObjectBytes(
    message: usize,
    bytes: *const c_void,
    byteSize: u32) -> tibems_status;

  /// Create a text message.
  pub fn tibemsTextMsg_Create(
    message: *mut usize) -> tibems_status;
  /// Get the string data from a text message.
  pub fn tibemsTextMsg_GetText(
    message: usize,
    text: *const *const c_char) -> tibems_status;
  /// Set the data string of a text message.
  pub fn tibemsTextMsg_SetText(
    message: usize,
    text: *const c_char) -> tibems_status;
}

/// struct to hold the error context
#[allow(dead_code)]
#[repr(C)]
pub struct tibemsErrorContext{
  /// internal value
  pub _val: [u8; 0]
}

/// struct to hold the connection factory
#[allow(dead_code)]
#[repr(C)]
pub struct tibemsConnectionFactory{
  /// internal value
  pub _val: [u8; 0]
}

/// struct to hold the message
#[allow(dead_code)]
#[repr(C)]
pub struct tibemsMsg{
  /// internal value
  pub _val: usize
}

/// struct to hold the lookup context
#[allow(dead_code)]
#[repr(C)]
#[derive(Copy,Clone,Debug)]
pub struct tibemsLookupContext{
  /// internal value
  pub _val: [u8; 0]
}

/// body types of a message
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum tibemsMsgType{
  /// unknown body
  TIBEMS_MESSAGE_UNKNOWN                      = 0,
  /// generic message
  TIBEMS_MESSAGE                              = 1,
  /// binary message
  TIBEMS_BYTES_MESSAGE                        = 2,
  /// map message
  TIBEMS_MAP_MESSAGE                          = 3,
  /// serialized object message
  TIBEMS_OBJECT_MESSAGE                       = 4,
  /// streaming message
  TIBEMS_STREAM_MESSAGE                       = 5,
  /// text message
  TIBEMS_TEXT_MESSAGE                         = 6,
  /// undefined
  TIBEMS_MESSAGE_UNDEFINED                    = 256
}

/// destination types
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum tibemsDestinationType{
  /// unkown
  TIBEMS_UNKNOWN                              = 0,
  /// queue
  TIBEMS_QUEUE                                = 1,
  /// topic
  TIBEMS_TOPIC                                = 2,
  /// undefined
  TIBEMS_DEST_UNDEFINED                       = 256
}

/// tibco boolean type
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum tibems_bool{
  /// false
  TIBEMS_FALSE  = 0,
  /// true
  TIBEMS_TRUE   = 1
}

/// Define delivery mode constants.
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum tibemsDeliveryMode{
  /// Non-persistent delivery.
  TIBEMS_NON_PERSISTENT  = 1,
  /// Persistent delivery.
  TIBEMS_PERSISTENT      = 2,
  /// Reliable delivery mode is a TIBCO proprietary extension that offers increased performance of the message producers. See also RELIABLE_DELIVERY in TIBCO Enterprise Message Service User’s Guide.
  TIBEMS_RELIABLE        = 22,
}

/// persistence type of a destination
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum tibems_permType{
  /// static
  TIBEMS_DEST_GET_STATIC = 1,
  /// dynamic
  TIBEMS_DEST_GET_DYNAMIC = 2,
  /// all but temporary destinations
  TIBEMS_DEST_GET_NOTEMP = 3,
  /// all
  TIBEMS_DEST_GET_ALL = 4,
}

/// acknowledgement types
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum tibemsAcknowledgeMode{
  /// transacted
  TIBEMS_SESSION_TRANSACTED                   = 0,
  /// In this mode, the session automatically acknowledges a message when message processing is finished—that is, when either of these calls returns successfully:
  /// synchronous receive calls (such as tibemsMsgConsumer_Receive)
  /// asynchronous listener callback (namely, tibemsMsgCallback)
  TIBEMS_AUTO_ACKNOWLEDGE                     = 1,
  /// In this mode, the client program acknowledges receipt by calling tibemsMsg_Acknowledge. Each call acknowledges all messages received so far.
  TIBEMS_CLIENT_ACKNOWLEDGE                   = 2,
  /// As with TIBEMS_AUTO_ACKNOWLEDGE, the session automatically acknowledges messages. However, it may do so lazily.
  /// Lazy means that the provider client library can delay transferring the acknowledgement to the server until a convenient time; meanwhile the server might redeliver the message. Lazy acknowledgement can reduce session overhead.
  TIBEMS_DUPS_OK_ACKNOWLEDGE                  = 3,
  /// In TIBEMS_NO_ACKNOWLEDGE mode, messages do not require acknowledgement (which reduces message overhead). The server never redelivers messages.
  /// This mode and behavior are proprietary extensions, specific to TIBCO EMS.
  TIBEMS_NO_ACKNOWLEDGE                       = 22,
  /// As with TIBEMS_CLIENT_ACKNOWLEDGE, the client program acknowledges receipt by calling tibemsMsg_Acknowledge. However, each call acknowledges only the individual message. The client may acknowledge messages in any order.
  /// This mode and behavior are proprietary extensions, specific to TIBCO EMS.
  TIBEMS_EXPLICIT_CLIENT_ACKNOWLEDGE          = 23,
  /// In this mode, the client program lazily acknowledges only the individual message, by calling tibemsMsg_Acknowledge. The client may acknowledge messages in any order.
  /// Lazy means that the provider client library can delay transferring the acknowledgement to the server until a convenient time; meanwhile the server might redeliver the message.
  /// This mode and behavior are proprietary extensions, specific to TIBCO EMS.
  TIBEMS_EXPLICIT_CLIENT_DUPS_OK_ACKNOWLEDGE  = 24
}

/// tibco ems status
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
#[repr(C)]
pub enum tibems_status{
  /// The call completed normally.
  TIBEMS_OK                          = 0,
  /// A function call or server request occurred in an inappropriate context. For example, tibemsSession_Commit indicates this status when the session is non-transactional.
  TIBEMS_ILLEGAL_STATE               = 1,
  /// The provider rejects the connection’s client ID. Setting a connection’s client ID to an invalid or duplicate value results in this exception. (A duplicate value is one that is already in use by another connection.)
  TIBEMS_INVALID_CLIENT_ID           = 2,
  /// tibemsd cannot locate the destination.
  TIBEMS_INVALID_DESTINATION         = 3,
  /// The client passed a message selector with invalid syntax; see Message Selectors.
  TIBEMS_INVALID_SELECTOR            = 4,
  /// Non-specific error code.
  TIBEMS_EXCEPTION                   = 5,
  /// The function cannot complete because of a security restriction.
  /// For example, the provider rejects a user or the user’s authentication.
  TIBEMS_SECURITY_EXCEPTION          = 6,
  /// The data stream within a message ended unexpectedly. tibemsBytesMsg contains a stream of bytes. tibemsStreamMsg contains a stream of characters. If any of their read functions detects the end of a stream unexpectedly, it indicates this status.
  TIBEMS_MSG_EOF                     = 7,
  /// Attempt to read from a message in write-only mode.
  TIBEMS_MSG_NOT_READABLE            = 9,
  /// Attempt to write to a message in read-only mode. See also, tibemsMsg_MakeWriteable.
  TIBEMS_MSG_NOT_WRITEABLE           = 10,
  /// An attempt to connect to the server has failed.
  /// The operation requires a server connection, but the program is not connected.
  TIBEMS_SERVER_NOT_CONNECTED        = 11,
  /// TIBEMS_VERSION_MISMATCH
  TIBEMS_VERSION_MISMATCH            = 12,
  /// The server cannot create a topic or durable because the name is already in use. (Also applies to collisions with external subjects, such as Rendezvous.)
  TIBEMS_SUBJECT_COLLISION           = 13,
  /// Cannot create a connection or transaction because the specified protocol does not exist.
  TIBEMS_INVALID_PROTOCOL            = 15,
  /// The connection URL includes an invalid hostname, or an attempt to lookup the host address failed.
  /// Host names must be less than 128 characters.
  TIBEMS_INVALID_HOSTNAME            = 17,
  /// The connection URL includes an invalid port number.
  TIBEMS_INVALID_PORT                = 18,
  /// The program exceeded available memory during the call.
  TIBEMS_NO_MEMORY                   = 19,
  /// The function received an illegal value as an argument.
  TIBEMS_INVALID_ARG                 = 20,
  /// The server has exceeded the maximum number of licensed connections or hosts that it can service.
  TIBEMS_SERVER_LIMIT                = 21,
  /// TIBEMS_MSG_DUPLICATE
  TIBEMS_MSG_DUPLICATE               = 22,
  /// TIBEMS_SERVER_DISCONNECTED
  TIBEMS_SERVER_DISCONNECTED         = 23,
  /// TIBEMS_SERVER_RECONNECTING
  TIBEMS_SERVER_RECONNECTING         = 24,
  /// The function call is not permitted (for example, closing a connection within a callback).
  TIBEMS_NOT_PERMITTED               = 27,
  /// Exception callback handler functions receive this code to indicate that the server has reconnected.
  /// See tibemsExceptionCallback
  TIBEMS_SERVER_RECONNECTED          = 28,
  /// In a lookup request, the name has incorrect syntax.
  /// The most common syntax error is a prefix other than tibjmsnaming:// (or a misspelling).
  /// See also, tibemsLookupContext.
  TIBEMS_INVALID_NAME                = 30,
  /// TIBEMS_INVALID_TYPE
  TIBEMS_INVALID_TYPE                = 31,
  /// An argument is outside the range of valid values.
  TIBEMS_INVALID_SIZE                = 32,
  /// TIBEMS_INVALID_COUNT
  TIBEMS_INVALID_COUNT               = 33,
  /// 1. The name lookup repository cannot find a name; the name is not bound. See also, tibemsLookupContext
  /// 2. A function that gets a message field or property value cannot find the specified item because the name is not bound in the message.
  TIBEMS_NOT_FOUND                   = 35,
  /// TIBEMS_ID_IN_USE
  TIBEMS_ID_IN_USE                   = 36,
  /// TIBEMS_ID_CONFLICT
  TIBEMS_ID_CONFLICT                 = 37,
  /// A datatype conversion failed while parsing a message (converting UTF-8 data to native datatypes).
  TIBEMS_CONVERSION_FAILED           = 38,
  /// The message is uninitialized or corrupt.
  TIBEMS_INVALID_MSG                 = 42,
  /// The message contains an invalid field. The message might be corrupt.
  TIBEMS_INVALID_FIELD               = 43,
  /// TIBEMS_INVALID_INSTANCE
  TIBEMS_INVALID_INSTANCE            = 44,
  /// The message is corrupt.
  TIBEMS_CORRUPT_MSG                 = 45,
  /// TIBEMS_PRODUCER_FAILED
  TIBEMS_PRODUCER_FAILED             = 47,
  /// The timeout has expired while waiting for a message. See tibemsMsgConsumer_ReceiveTimeout.
  TIBEMS_TIMEOUT                     = 50,
  /// A blocking operation has been interrupted. See tibemsMsgConsumer_Receive.
  TIBEMS_INTR                        = 51,
  /// A server queue or topic has exceeded its size limit, and cannot add a new message.
  TIBEMS_DESTINATION_LIMIT_EXCEEDED  = 52,
  /// The server has exceeded its memory limit.
  TIBEMS_MEM_LIMIT_EXCEEDED          = 53,
  /// IBM z/OS only. A blocking operation has been interrupted. See tibx_MVSConsole_SetConsumer().
  TIBEMS_USER_INTR                   = 54,
  /// TIBEMS_INVALID_QUEUE_GROUP
  TIBEMS_INVALID_QUEUE_GROUP         = 63,
  /// TIBEMS_INVALID_TIME_INTERVAL
  TIBEMS_INVALID_TIME_INTERVAL       = 64,
  /// The function detected an invalid I/O source (such as a socket or file).
  TIBEMS_INVALID_IO_SOURCE           = 65,
  /// TIBEMS_INVALID_IO_CONDITION
  TIBEMS_INVALID_IO_CONDITION        = 66,
  /// TIBEMS_SOCKET_LIMIT
  TIBEMS_SOCKET_LIMIT                = 67,
  /// An operating system error occurred during the call.
  TIBEMS_OS_ERROR                    = 68,
  /// TIBEMS_WOULD_BLOCK
  TIBEMS_WOULD_BLOCK                 = 69,
  /// The result of the call overflowed the buffer supplied by the program.
  TIBEMS_INSUFFICIENT_BUFFER         = 70,
  /// The call detected an unexpected end-of-file.
  TIBEMS_EOF                         = 71,
  /// The function detected an invalid file.
  TIBEMS_INVALID_FILE                = 72,
  /// The specified file does not exist.
  TIBEMS_FILE_NOT_FOUND              = 73,
  /// An operating system I/O call failed.
  TIBEMS_IO_FAILED                   = 74,
  /// TIBEMS_NOT_FILE_OWNER
  TIBEMS_NOT_FILE_OWNER              = 80,
  /// Cannot create an item that already exists.
  TIBEMS_ALREADY_EXISTS              = 91,
  /// The connection is invalid.
  TIBEMS_INVALID_CONNECTION          = 100,
  /// The session is invalid.
  TIBEMS_INVALID_SESSION             = 101,
  /// The consumer is invalid.
  TIBEMS_INVALID_CONSUMER            = 102,
  /// The producer is invalid.
  TIBEMS_INVALID_PRODUCER            = 103,
  /// The server could not authenticate the user.
  TIBEMS_INVALID_USER                = 104,
  /// TIBEMS_INVALID_GROUP
  TIBEMS_INVALID_GROUP               = 105,
  /// A transaction failed at the server during a commit call.
  TIBEMS_TRANSACTION_FAILED          = 106,
  /// Failure during prepare or commit caused automatic rollback of a transaction. This type of rollback can occur during fault tolerance failover.
  TIBEMS_TRANSACTION_ROLLBACK        = 107,
  /// A transaction failed during two-phase commit; the program may attempt to commit it again.
  TIBEMS_TRANSACTION_RETRY           = 108,
  /// When a session uses an XA transaction manager, the XA resource is the correct locus for all commit and rollback requests. Local commit or rollback calls are not permitted, and indicate this status.
  TIBEMS_INVALID_XARESOURCE          = 109,
  /// The producer attempted to send a message immediately after a fault tolerance failover to another server. The new server has no record of the transaction.
  TIBEMS_FT_SERVER_LACKS_TRANSACTION = 110,
  /// TIBEMS_LDAP_ERROR
  TIBEMS_LDAP_ERROR                  = 120,
  /// TIBEMS_INVALID_PROXY_USER
  TIBEMS_INVALID_PROXY_USER          = 121,
  /// SSL detected an invalid X.509 certificate.
  TIBEMS_INVALID_CERT                = 150,
  /// SSL detected an X.509 certificate that is not yet valid; that is, the current date is before the first date for which the certificate becomes valid.
  TIBEMS_INVALID_CERT_NOT_YET        = 151,
  /// SSL detected an X.509 certificate that is no longer valid; that is, the current date is after the expiration date.
  TIBEMS_INVALID_CERT_EXPIRED        = 152,
  /// SSL detected an X.509 certificate containing corrupt data.
  TIBEMS_INVALID_CERT_DATA           = 153,
  /// Error loading a cipher suite algorithm.
  TIBEMS_ALGORITHM_ERROR             = 154,
  /// Generic SSL error code.
  TIBEMS_SSL_ERROR                   = 155,
  /// SSL detected a private key that does not match its public key.   
  TIBEMS_INVALID_PRIVATE_KEY         = 156,
  /// SSL detected a certificate encoding that it cannot read.
  TIBEMS_INVALID_ENCODING            = 157,
  /// SSL lacks sufficient random data to complete an operation securely.
  TIBEMS_NOT_ENOUGH_RANDOM           = 158,
  /// TIBEMS_INVALID_CRL_DATA
  TIBEMS_INVALID_CRL_DATA            = 159,
  /// TIBEMS_CRL_OFF
  TIBEMS_CRL_OFF                     = 160,
  /// TIBEMS_EMPTY_CRL
  TIBEMS_EMPTY_CRL                   = 161,
  /// Initialization of the tibems library failed. For example, this code could be generated if the library failed to allocate memory while building its basic structures.
  TIBEMS_NOT_INITIALIZED             = 200,
  /// TIBEMS_INIT_FAILURE
  TIBEMS_INIT_FAILURE                = 201,
  /// TIBEMS_ARG_CONFLICT
  TIBEMS_ARG_CONFLICT                = 202,
  /// TIBEMS_SERVICE_NOT_FOUND
  TIBEMS_SERVICE_NOT_FOUND           = 210,
  /// TIBEMS_INVALID_CALLBACK
  TIBEMS_INVALID_CALLBACK            = 211,
  /// TIBEMS_INVALID_QUEUE
  TIBEMS_INVALID_QUEUE               = 212,
  /// TIBEMS_INVALID_EVENT
  TIBEMS_INVALID_EVENT               = 213,
  /// TIBEMS_INVALID_SUBJECT
  TIBEMS_INVALID_SUBJECT             = 214,
  /// TIBEMS_INVALID_DISPATCHER
  TIBEMS_INVALID_DISPATCHER          = 215,
    
  /* JVM related errors */
  /// TIBEMS_JNI_EXCEPTION
  TIBEMS_JNI_EXCEPTION               = 230,
  /// TIBEMS_JNI_ERR
  TIBEMS_JNI_ERR                     = 231,
  /// TIBEMS_JNI_EDETACHED
  TIBEMS_JNI_EDETACHED               = 232,
  /// TIBEMS_JNI_EVERSION
  TIBEMS_JNI_EVERSION                = 233,
  /// TIBEMS_JNI_EEXIST
  TIBEMS_JNI_EEXIST                  = 235,
  /// TIBEMS_JNI_EINVAL
  TIBEMS_JNI_EINVAL                  = 236,
  /// TIBEMS_NO_MEMORY_FOR_OBJECT
  TIBEMS_NO_MEMORY_FOR_OBJECT        = 237,
  /// TIBEMS_UFO_CONNECTION_FAILURE
  TIBEMS_UFO_CONNECTION_FAILURE      = 240,
  /// The function is not implemented.
  TIBEMS_NOT_IMPLEMENTED             = 255
}

/// Represents a message field or property.
#[allow(dead_code)]
#[repr(C)]
#[derive(Copy,Clone,Debug)]
pub struct tibemsMsgField{
  /// One-byte indicator of the field’s datatype; for values, see the following table.
  r#type: i8,
  /// Size of the data (in bytes). Zero is a special value, indicating that the size is unknown.
  size: i32,
  /// Number of elements in the array.
  count: i32,
  /// Actual data in the field, or property value.
  data: *const c_void
}
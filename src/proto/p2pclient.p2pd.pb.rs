// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(enumeration = "request::Type", required, tag = "1")]
    pub r#type: i32,
    #[prost(message, optional, tag = "2")]
    pub connect: ::core::option::Option<ConnectRequest>,
    #[prost(message, optional, tag = "3")]
    pub stream_open: ::core::option::Option<StreamOpenRequest>,
    #[prost(message, optional, tag = "4")]
    pub stream_handler: ::core::option::Option<StreamHandlerRequest>,
    #[prost(message, optional, tag = "9")]
    pub remove_stream_handler: ::core::option::Option<RemoveStreamHandlerRequest>,
    #[prost(message, optional, tag = "5")]
    pub dht: ::core::option::Option<DhtRequest>,
    #[prost(message, optional, tag = "6")]
    pub conn_manager: ::core::option::Option<ConnManagerRequest>,
    #[prost(message, optional, tag = "7")]
    pub disconnect: ::core::option::Option<DisconnectRequest>,
    #[prost(message, optional, tag = "8")]
    pub pubsub: ::core::option::Option<PsRequest>,
}
/// Nested message and enum types in `Request`.
pub mod request {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        Identify = 0,
        Connect = 1,
        StreamOpen = 2,
        StreamHandler = 3,
        RemoveStreamHandler = 10,
        Dht = 4,
        ListPeers = 5,
        Connmanager = 6,
        Disconnect = 7,
        Pubsub = 8,
        PersistentConnUpgrade = 9,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Identify => "IDENTIFY",
                Type::Connect => "CONNECT",
                Type::StreamOpen => "STREAM_OPEN",
                Type::StreamHandler => "STREAM_HANDLER",
                Type::RemoveStreamHandler => "REMOVE_STREAM_HANDLER",
                Type::Dht => "DHT",
                Type::ListPeers => "LIST_PEERS",
                Type::Connmanager => "CONNMANAGER",
                Type::Disconnect => "DISCONNECT",
                Type::Pubsub => "PUBSUB",
                Type::PersistentConnUpgrade => "PERSISTENT_CONN_UPGRADE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "IDENTIFY" => Some(Self::Identify),
                "CONNECT" => Some(Self::Connect),
                "STREAM_OPEN" => Some(Self::StreamOpen),
                "STREAM_HANDLER" => Some(Self::StreamHandler),
                "REMOVE_STREAM_HANDLER" => Some(Self::RemoveStreamHandler),
                "DHT" => Some(Self::Dht),
                "LIST_PEERS" => Some(Self::ListPeers),
                "CONNMANAGER" => Some(Self::Connmanager),
                "DISCONNECT" => Some(Self::Disconnect),
                "PUBSUB" => Some(Self::Pubsub),
                "PERSISTENT_CONN_UPGRADE" => Some(Self::PersistentConnUpgrade),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(enumeration = "response::Type", required, tag = "1")]
    pub r#type: i32,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<ErrorResponse>,
    #[prost(message, optional, tag = "3")]
    pub stream_info: ::core::option::Option<StreamInfo>,
    #[prost(message, optional, tag = "4")]
    pub identify: ::core::option::Option<IdentifyResponse>,
    #[prost(message, optional, tag = "5")]
    pub dht: ::core::option::Option<DhtResponse>,
    #[prost(message, repeated, tag = "6")]
    pub peers: ::prost::alloc::vec::Vec<PeerInfo>,
    #[prost(message, optional, tag = "7")]
    pub pubsub: ::core::option::Option<PsResponse>,
}
/// Nested message and enum types in `Response`.
pub mod response {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        Ok = 0,
        Error = 1,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Ok => "OK",
                Type::Error => "ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OK" => Some(Self::Ok),
                "ERROR" => Some(Self::Error),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistentConnectionRequest {
    #[prost(bytes = "vec", required, tag = "1")]
    pub call_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(oneof = "persistent_connection_request::Message", tags = "2, 6, 3, 4, 5")]
    pub message: ::core::option::Option<persistent_connection_request::Message>,
}
/// Nested message and enum types in `PersistentConnectionRequest`.
pub mod persistent_connection_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "2")]
        AddUnaryHandler(super::AddUnaryHandlerRequest),
        #[prost(message, tag = "6")]
        RemoveUnaryHandler(super::RemoveUnaryHandlerRequest),
        #[prost(message, tag = "3")]
        CallUnary(super::CallUnaryRequest),
        #[prost(message, tag = "4")]
        UnaryResponse(super::CallUnaryResponse),
        #[prost(message, tag = "5")]
        Cancel(super::Cancel),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersistentConnectionResponse {
    #[prost(bytes = "vec", required, tag = "1")]
    pub call_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(oneof = "persistent_connection_response::Message", tags = "2, 3, 4, 5")]
    pub message: ::core::option::Option<persistent_connection_response::Message>,
}
/// Nested message and enum types in `PersistentConnectionResponse`.
pub mod persistent_connection_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        #[prost(message, tag = "2")]
        CallUnaryResponse(super::CallUnaryResponse),
        #[prost(message, tag = "3")]
        RequestHandling(super::CallUnaryRequest),
        #[prost(message, tag = "4")]
        DaemonError(super::DaemonError),
        #[prost(message, tag = "5")]
        Cancel(super::Cancel),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentifyResponse {
    #[prost(bytes = "vec", required, tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub addrs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectRequest {
    #[prost(bytes = "vec", required, tag = "1")]
    pub peer: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub addrs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(int64, optional, tag = "3")]
    pub timeout: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamOpenRequest {
    #[prost(bytes = "vec", required, tag = "1")]
    pub peer: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub proto: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "3")]
    pub timeout: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamHandlerRequest {
    #[prost(bytes = "vec", required, tag = "1")]
    pub addr: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub proto: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, required, tag = "3")]
    pub balanced: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveStreamHandlerRequest {
    #[prost(bytes = "vec", required, tag = "1")]
    pub addr: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub proto: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorResponse {
    #[prost(string, required, tag = "1")]
    pub msg: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamInfo {
    #[prost(bytes = "vec", required, tag = "1")]
    pub peer: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", required, tag = "2")]
    pub addr: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, required, tag = "3")]
    pub proto: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DhtRequest {
    #[prost(enumeration = "dht_request::Type", required, tag = "1")]
    pub r#type: i32,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub peer: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub cid: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "4")]
    pub key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(int32, optional, tag = "6")]
    pub count: ::core::option::Option<i32>,
    #[prost(int64, optional, tag = "7")]
    pub timeout: ::core::option::Option<i64>,
}
/// Nested message and enum types in `DHTRequest`.
pub mod dht_request {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        FindPeer = 0,
        FindPeersConnectedToPeer = 1,
        FindProviders = 2,
        GetClosestPeers = 3,
        GetPublicKey = 4,
        GetValue = 5,
        SearchValue = 6,
        PutValue = 7,
        Provide = 8,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::FindPeer => "FIND_PEER",
                Type::FindPeersConnectedToPeer => "FIND_PEERS_CONNECTED_TO_PEER",
                Type::FindProviders => "FIND_PROVIDERS",
                Type::GetClosestPeers => "GET_CLOSEST_PEERS",
                Type::GetPublicKey => "GET_PUBLIC_KEY",
                Type::GetValue => "GET_VALUE",
                Type::SearchValue => "SEARCH_VALUE",
                Type::PutValue => "PUT_VALUE",
                Type::Provide => "PROVIDE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FIND_PEER" => Some(Self::FindPeer),
                "FIND_PEERS_CONNECTED_TO_PEER" => Some(Self::FindPeersConnectedToPeer),
                "FIND_PROVIDERS" => Some(Self::FindProviders),
                "GET_CLOSEST_PEERS" => Some(Self::GetClosestPeers),
                "GET_PUBLIC_KEY" => Some(Self::GetPublicKey),
                "GET_VALUE" => Some(Self::GetValue),
                "SEARCH_VALUE" => Some(Self::SearchValue),
                "PUT_VALUE" => Some(Self::PutValue),
                "PROVIDE" => Some(Self::Provide),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DhtResponse {
    #[prost(enumeration = "dht_response::Type", required, tag = "1")]
    pub r#type: i32,
    #[prost(message, optional, tag = "2")]
    pub peer: ::core::option::Option<PeerInfo>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `DHTResponse`.
pub mod dht_response {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        Begin = 0,
        Value = 1,
        End = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Begin => "BEGIN",
                Type::Value => "VALUE",
                Type::End => "END",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BEGIN" => Some(Self::Begin),
                "VALUE" => Some(Self::Value),
                "END" => Some(Self::End),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerInfo {
    #[prost(bytes = "vec", required, tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub addrs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnManagerRequest {
    #[prost(enumeration = "conn_manager_request::Type", required, tag = "1")]
    pub r#type: i32,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub peer: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "3")]
    pub tag: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "4")]
    pub weight: ::core::option::Option<i64>,
}
/// Nested message and enum types in `ConnManagerRequest`.
pub mod conn_manager_request {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        TagPeer = 0,
        UntagPeer = 1,
        Trim = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::TagPeer => "TAG_PEER",
                Type::UntagPeer => "UNTAG_PEER",
                Type::Trim => "TRIM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TAG_PEER" => Some(Self::TagPeer),
                "UNTAG_PEER" => Some(Self::UntagPeer),
                "TRIM" => Some(Self::Trim),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisconnectRequest {
    #[prost(bytes = "vec", required, tag = "1")]
    pub peer: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PsRequest {
    #[prost(enumeration = "ps_request::Type", required, tag = "1")]
    pub r#type: i32,
    #[prost(string, optional, tag = "2")]
    pub topic: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `PSRequest`.
pub mod ps_request {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        GetTopics = 0,
        ListPeers = 1,
        Publish = 2,
        Subscribe = 3,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::GetTopics => "GET_TOPICS",
                Type::ListPeers => "LIST_PEERS",
                Type::Publish => "PUBLISH",
                Type::Subscribe => "SUBSCRIBE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "GET_TOPICS" => Some(Self::GetTopics),
                "LIST_PEERS" => Some(Self::ListPeers),
                "PUBLISH" => Some(Self::Publish),
                "SUBSCRIBE" => Some(Self::Subscribe),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PsMessage {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub from: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub seqno: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, repeated, tag = "4")]
    pub topic_i_ds: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub signature: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PsResponse {
    #[prost(string, repeated, tag = "1")]
    pub topics: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub peer_i_ds: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallUnaryRequest {
    #[prost(bytes = "vec", required, tag = "1")]
    pub peer: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, required, tag = "2")]
    pub proto: ::prost::alloc::string::String,
    #[prost(bytes = "vec", required, tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallUnaryResponse {
    #[prost(oneof = "call_unary_response::Result", tags = "1, 2")]
    pub result: ::core::option::Option<call_unary_response::Result>,
}
/// Nested message and enum types in `CallUnaryResponse`.
pub mod call_unary_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(bytes, tag = "1")]
        Response(::prost::alloc::vec::Vec<u8>),
        #[prost(bytes, tag = "2")]
        Error(::prost::alloc::vec::Vec<u8>),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddUnaryHandlerRequest {
    #[prost(string, required, tag = "1")]
    pub proto: ::prost::alloc::string::String,
    #[prost(bool, required, tag = "2")]
    pub balanced: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveUnaryHandlerRequest {
    #[prost(string, required, tag = "1")]
    pub proto: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DaemonError {
    #[prost(string, optional, tag = "1")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cancel {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpcError {
    #[prost(string, optional, tag = "1")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}

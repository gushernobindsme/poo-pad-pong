#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncKeys {
    #[prost(enumeration = "MessageType", tag = "1")]
    pub message_type: i32,
    #[prost(oneof = "sync_keys::Payload", tags = "2, 3, 4")]
    pub payload: ::core::option::Option<sync_keys::Payload>,
}
/// Nested message and enum types in `SyncKeys`.
pub mod sync_keys {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "2")]
        CreateKeysRequest(super::CreateKeysRequest),
        #[prost(message, tag = "3")]
        UpdateKeysRequest(super::UpdateKeysRequest),
        #[prost(message, tag = "4")]
        DeleteKeysRequest(super::DeleteKeysRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateKeysRequest {
    #[prost(string, tag = "1")]
    pub rule_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateKeysRequest {
    #[prost(string, tag = "1")]
    pub rule_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteKeysRequest {
    #[prost(string, tag = "1")]
    pub rule_id: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MessageType {
    Invalid = 0,
    CreateKeysRequest = 1,
    UpdateKeysRequest = 2,
    DeleteKeysRequest = 3,
}
impl MessageType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MessageType::Invalid => "INVALID",
            MessageType::CreateKeysRequest => "CREATE_KEYS_REQUEST",
            MessageType::UpdateKeysRequest => "UPDATE_KEYS_REQUEST",
            MessageType::DeleteKeysRequest => "DELETE_KEYS_REQUEST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVALID" => Some(Self::Invalid),
            "CREATE_KEYS_REQUEST" => Some(Self::CreateKeysRequest),
            "UPDATE_KEYS_REQUEST" => Some(Self::UpdateKeysRequest),
            "DELETE_KEYS_REQUEST" => Some(Self::DeleteKeysRequest),
            _ => None,
        }
    }
}

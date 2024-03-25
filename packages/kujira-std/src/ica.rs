use cosmwasm_schema::{cw_serde, serde::Serializer};
use cosmwasm_std::Binary;

#[cw_serde]
pub enum IcaSudoMsg {
    IcaRegisterCallback(IcaRegisterCallbackData),
    IcaTxCallback(IcaTxCallbackData),
}

#[cw_serde]
pub struct IcaRegisterCallbackData {
    pub connection_id: String,
    pub account_id: String,
    pub callback: Option<Binary>,
    pub result: IcaRegisterResult,
}

#[cw_serde]
pub struct IcaTxCallbackData {
    pub connection_id: String,
    pub account_id: String,
    pub sequence: u64,
    pub callback: Option<Binary>,
    pub result: IcaTxResult,
}

#[cw_serde]
pub enum IcaRegisterResult {
    Success { data: IcaOpenVersion },
    Error { error: String },
    Timeout {},
}

#[cw_serde]
pub enum IcaTxResult {
    Success { data: Binary },
    Error { error: String },
    Timeout {},
}

#[cw_serde]
pub enum IcaMsg {
    Register {
        connection_id: String,
        account_id: String,
        version: IcaRegisterVersion,
        callback: Option<Binary>,
    },
    Submit {
        connection_id: String,
        account_id: String,
        msgs: Vec<ProtobufAny>,
        memo: String,
        timeout: u64,
        callback: Option<Binary>,
    },
}

#[cw_serde]
#[serde(untagged)]
pub enum IcaRegisterVersion {
    #[serde(serialize_with = "empty_string")]
    Default,
    Ics27(Ics27MetadataInit),
    Ics29(Ics29MetadataInit),
}

impl Default for IcaRegisterVersion {
    fn default() -> Self {
        Self::Default
    }
}

fn empty_string<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str("")
}

#[cw_serde]
#[serde(untagged)]
pub enum IcaOpenVersion {
    Ics27(Ics27MetadataOpen),
    Ics29(Ics29MetadataOpen),
}

#[cw_serde]
pub struct Ics27MetadataInit {
    version: String,
    controller_connection_id: String,
    host_connection_id: String,
    encoding: String,
    tx_type: String,
}

impl Ics27MetadataInit {
    pub fn new(controller_connection_id: String, host_connection_id: String) -> Self {
        Self {
            version: "ics27-1".to_string(),
            controller_connection_id,
            host_connection_id,
            encoding: "proto3".to_string(),
            tx_type: "sdk_multi_msg".to_string(),
        }
    }
}

#[cw_serde]
pub struct Ics27MetadataOpen {
    version: String,
    controller_connection_id: String,
    host_connection_id: String,
    address: String,
    encoding: String,
    tx_type: String,
}

#[cw_serde]
pub struct Ics29MetadataInit {
    fee_version: String,
    app_version: Ics27MetadataInit,
}

impl From<Ics27MetadataInit> for Ics29MetadataInit {
    fn from(app_version: Ics27MetadataInit) -> Self {
        Self {
            fee_version: "ics29-1".to_string(),
            app_version,
        }
    }
}

impl Ics29MetadataInit {
    pub fn new(controller_connection_id: String, host_connection_id: String) -> Self {
        Ics27MetadataInit::new(controller_connection_id, host_connection_id).into()
    }
}

#[cw_serde]
pub struct Ics29MetadataOpen {
    fee_version: String,
    app_version: Ics27MetadataOpen,
}

#[cw_serde]
/// Type for wrapping any protobuf message
pub struct ProtobufAny {
    /// **type_url** describes the type of the serialized message
    type_url: String,

    ///  **value** must be a valid serialized protocol buffer of the above specified type
    value: Binary,
}

impl ProtobufAny {
    /// Helper to create new ProtobufAny type:
    /// * **type_url** describes the type of the serialized message
    /// * **value** must be a valid serialized protocol buffer of the above specified type
    pub fn new(type_url: impl Into<String>, value: impl Into<Binary>) -> Self {
        ProtobufAny {
            type_url: type_url.into(),
            value: value.into(),
        }
    }
}

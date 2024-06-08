use cosmwasm_schema::{
    cw_serde,
    serde::{Deserialize, Deserializer, Serializer},
};
use cosmwasm_std::{from_json, to_json_string, Binary};

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
    #[serde(serialize_with = "serialize_empty_string")]
    Default,
    #[serde(serialize_with = "serialize_ics27")]
    Ics27(Ics27MetadataInit),
    #[serde(serialize_with = "serialize_ics29")]
    Ics29(Ics29MetadataInit),
}

impl Default for IcaRegisterVersion {
    fn default() -> Self {
        Self::Default
    }
}

fn serialize_empty_string<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str("")
}

fn serialize_ics27<S>(metadata: &Ics27MetadataInit, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&to_json_string(metadata).unwrap())
}

fn serialize_ics29<S>(metadata: &Ics29MetadataInit, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&to_json_string(metadata).unwrap())
}
use cosmwasm_schema::{
    cw_serde,
    serde::{Deserialize, Deserializer, Serializer},
};
use cosmwasm_std::{from_json, to_json_string, Binary, Coin, IbcTimeout};

#[cw_serde]
pub enum IcaSudoMsg {
    IcaRegisterCallback(IcaRegisterCallbackData),
    IcaTxCallback(IcaTxCallbackData),
    TransferCallback(TransferCallbackData),
    TransferReceipt(TransferReceiptData),
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
    Transfer {
        channel_id: String,
        to_address: String,
        amount: Coin,
        timeout: IbcTimeout,
        callback: Binary,
    },
}

#[cw_serde]
#[serde(untagged)]
pub enum IcaRegisterVersion {
    #[serde(serialize_with = "serialize_empty_string")]
    Default,
    #[serde(serialize_with = "serialize_ics27")]
    Ics27(Ics27MetadataInit),
    #[serde(serialize_with = "serialize_ics29")]
    Ics29(Ics29MetadataInit),
}

impl Default for IcaRegisterVersion {
    fn default() -> Self {
        Self::Default
    }
}

fn serialize_empty_string<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str("")
}

fn serialize_ics27<S>(metadata: &Ics27MetadataInit, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&to_json_string(metadata).unwrap())
}

fn serialize_ics29<S>(metadata: &Ics29MetadataInit, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&to_json_string(metadata).unwrap())
}

#[cw_serde]
pub struct Ics27MetadataInit {
    pub version: String,
    pub controller_connection_id: String,
    pub host_connection_id: String,
    pub encoding: String,
    pub tx_type: String,
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
pub struct Ics29MetadataInit {
    pub fee_version: String,
    #[serde(serialize_with = "serialize_ics27")]
    pub app_version: Ics27MetadataInit,
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

#[derive(
    ::cosmwasm_schema::serde::Serialize,
    // ::cosmwasm_schema::serde::Deserialize,
    ::std::clone::Clone,
    ::std::fmt::Debug,
    ::std::cmp::PartialEq,
    ::cosmwasm_schema::schemars::JsonSchema,
)]
#[allow(clippy::derive_partial_eq_without_eq)] // Allow users of `#[cw_serde]` to not implement Eq without clippy complaining
#[serde(
    untagged,
    deny_unknown_fields,
    rename_all = "snake_case",
    crate = "::cosmwasm_schema::serde"
)]
#[schemars(crate = "::cosmwasm_schema::schemars")]
pub enum IcaOpenVersion {
    Ics27(Ics27MetadataOpen),
    Ics29(Ics29MetadataOpen),
}

impl<'de> Deserialize<'de> for IcaOpenVersion {
    fn deserialize<D>(deserializer: D) -> Result<IcaOpenVersion, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IcaOpenVersionVisitor;
        impl<'de> cosmwasm_schema::serde::de::Visitor<'de> for IcaOpenVersionVisitor {
            type Value = IcaOpenVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string containing json data")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: cosmwasm_schema::serde::de::Error,
            {
                #[derive(cosmwasm_schema::serde::Deserialize)]
                #[serde(
                    untagged,
                    rename_all = "snake_case",
                    crate = "::cosmwasm_schema::serde"
                )]
                enum DeserOpenVersion {
                    Ics27 {
                        #[allow(dead_code)]
                        version: String,
                    },
                    Ics29 {
                        fee_version: String,
                        app_version: String,
                    },
                }
                let data = Binary::from_base64(v).map_err(E::custom)?;
                let r#type: DeserOpenVersion = from_json(data).map_err(E::custom)?;
                match r#type {
                    DeserOpenVersion::Ics27 { .. } => {
                        let data = Binary::from_base64(v).map_err(E::custom)?;
                        Ok(IcaOpenVersion::Ics27(
                            from_json::<Ics27MetadataOpen>(data).map_err(E::custom)?,
                        ))
                    }
                    DeserOpenVersion::Ics29 {
                        fee_version,
                        app_version,
                    } => {
                        // app version is json-escaped Ics27MetadataOpen
                        serde_json_wasm::from_str(&app_version)
                            .map_err(E::custom)
                            .map(|app_version| {
                                IcaOpenVersion::Ics29(Ics29MetadataOpen {
                                    fee_version,
                                    app_version,
                                })
                            })
                    }
                }
            }
        }

        deserializer.deserialize_any(IcaOpenVersionVisitor)
    }
}

#[cw_serde]
pub struct Ics27MetadataOpen {
    pub version: String,
    pub controller_connection_id: String,
    pub host_connection_id: String,
    pub address: String,
    pub encoding: String,
    pub tx_type: String,
}

#[cw_serde]
pub struct Ics29MetadataOpen {
    pub fee_version: String,
    pub app_version: Ics27MetadataOpen,
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

#[cw_serde]
pub struct TransferCallbackData {
    pub port: String,
    pub channel: String,
    pub sequence: u64,
    pub receiver: String,
    pub denom: String,
    pub amount: String,
    pub memo: String,
    pub result: IcaTxResult,
    pub callback: Binary,
}

#[cw_serde]
pub struct TransferReceiptData {
    pub port: String,
    pub channel: String,
    pub sequence: u64,
    pub sender: String,
    pub denom: String,
    pub amount: String,
    pub memo: String,
}

#[cfg(test)]
mod tests {
    use crate::IcaOpenVersion;

    #[test]
    fn test_deserialize_callback_data() {
        let data = 
            "{\"data\":\"eyJ2ZXJzaW9uIjoiaWNzMjctMSIsImNvbnRyb2xsZXJfY29ubmVjdGlvbl9pZCI6ImNvbm5lY3Rpb24tMCIsImhvc3RfY29ubmVjdGlvbl9pZCI6ImNvbm5lY3Rpb24tMCIsImFkZHJlc3MiOiJjb3Ntb3MxbDhkY2xubmxjc2twanBuMmtwczBjYWw2ZWpuZzdyNGR5cGV0NnIyeGZkdHJjNjBjeWg2cXh2eXdjYyIsImVuY29kaW5nIjoicHJvdG8zIiwidHhfdHlwZSI6InNka19tdWx0aV9tc2cifQ==\"}";

        let as_json: serde_json::Value = serde_json::from_slice(data.as_bytes()).unwrap();

        let data: Result<IcaOpenVersion,_> = serde_json::from_value(as_json.get("data").unwrap().clone());
        assert!(data.is_ok());
        assert!(matches!(data.unwrap(), IcaOpenVersion::Ics27(_)));

        let data = "{\"data\":\"eyJmZWVfdmVyc2lvbiI6ImljczI5LTEiLCAiYXBwX3ZlcnNpb24iOiJ7XCJ2ZXJzaW9uXCI6XCJpY3MyNy0xXCIsXCJjb250cm9sbGVyX2Nvbm5lY3Rpb25faWRcIjpcImNvbm5lY3Rpb24tMFwiLFwiaG9zdF9jb25uZWN0aW9uX2lkXCI6XCJjb25uZWN0aW9uLTBcIixcImFkZHJlc3NcIjpcImNvc21vczFsOGRjbG5ubGNza3BqcG4ya3BzMGNhbDZlam5nN3I0ZHlwZXQ2cjJ4ZmR0cmM2MGN5aDZxeHZ5d2NjXCIsXCJlbmNvZGluZ1wiOlwicHJvdG8zXCIsXCJ0eF90eXBlXCI6XCJzZGtfbXVsdGlfbXNnXCJ9In0=\"}";

        let as_json: serde_json::Value = serde_json::from_slice(data.as_bytes()).unwrap();

        let data: Result<IcaOpenVersion,_> = serde_json::from_value(as_json.get("data").unwrap().clone());
        assert!(data.is_ok());
        assert!(matches!(data.unwrap(), IcaOpenVersion::Ics29(_)));
    }
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
pub struct Ics29MetadataInit {
    fee_version: String,
    #[serde(serialize_with = "serialize_ics27")]
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

#[derive(
    ::cosmwasm_schema::serde::Serialize,
    // ::cosmwasm_schema::serde::Deserialize,
    ::std::clone::Clone,
    ::std::fmt::Debug,
    ::std::cmp::PartialEq,
    ::cosmwasm_schema::schemars::JsonSchema,
)]
#[allow(clippy::derive_partial_eq_without_eq)] // Allow users of `#[cw_serde]` to not implement Eq without clippy complaining
#[serde(
    untagged,
    deny_unknown_fields,
    rename_all = "snake_case",
    crate = "::cosmwasm_schema::serde"
)]
#[schemars(crate = "::cosmwasm_schema::schemars")]
pub enum IcaOpenVersion {
    Ics27(Ics27MetadataOpen),
    Ics29(Ics29MetadataOpen),
}

impl<'de> Deserialize<'de> for IcaOpenVersion {
    fn deserialize<D>(deserializer: D) -> Result<IcaOpenVersion, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IcaOpenVersionVisitor;
        impl<'de> cosmwasm_schema::serde::de::Visitor<'de> for IcaOpenVersionVisitor {
            type Value = IcaOpenVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string containing json data")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: cosmwasm_schema::serde::de::Error,
            {
                #[derive(cosmwasm_schema::serde::Deserialize)]
                #[serde(
                    untagged,
                    rename_all = "snake_case",
                    crate = "::cosmwasm_schema::serde"
                )]
                enum DeserOpenVersion {
                    Ics27 {
                        #[allow(dead_code)]
                        version: String,
                    },
                    Ics29 {
                        fee_version: String,
                        app_version: String,
                    },
                }
                let data = Binary::from_base64(v).map_err(E::custom)?;
                let r#type: DeserOpenVersion = from_json(data).map_err(E::custom)?;
                match r#type {
                    DeserOpenVersion::Ics27 { .. } => {
                        let data = Binary::from_base64(v).map_err(E::custom)?;
                        Ok(IcaOpenVersion::Ics27(
                            from_json::<Ics27MetadataOpen>(data).map_err(E::custom)?,
                        ))
                    }
                    DeserOpenVersion::Ics29 {
                        fee_version,
                        app_version,
                    } => {
                        // app version is json-escaped Ics27MetadataOpen
                        serde_json_wasm::from_str(&app_version)
                            .map_err(E::custom)
                            .map(|app_version| {
                                IcaOpenVersion::Ics29(Ics29MetadataOpen {
                                    fee_version,
                                    app_version,
                                })
                            })
                    }
                }
            }
        }

        deserializer.deserialize_any(IcaOpenVersionVisitor)
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

#[cfg(test)]
mod tests {
    use crate::IcaOpenVersion;

    #[test]
    fn test_deserialize_callback_data() {
        let data = 
            "{\"data\":\"eyJ2ZXJzaW9uIjoiaWNzMjctMSIsImNvbnRyb2xsZXJfY29ubmVjdGlvbl9pZCI6ImNvbm5lY3Rpb24tMCIsImhvc3RfY29ubmVjdGlvbl9pZCI6ImNvbm5lY3Rpb24tMCIsImFkZHJlc3MiOiJjb3Ntb3MxbDhkY2xubmxjc2twanBuMmtwczBjYWw2ZWpuZzdyNGR5cGV0NnIyeGZkdHJjNjBjeWg2cXh2eXdjYyIsImVuY29kaW5nIjoicHJvdG8zIiwidHhfdHlwZSI6InNka19tdWx0aV9tc2cifQ==\"}";

        let as_json: serde_json::Value = serde_json::from_slice(data.as_bytes()).unwrap();

        let data: Result<IcaOpenVersion,_> = serde_json::from_value(as_json.get("data").unwrap().clone());
        assert!(data.is_ok());
        assert!(matches!(data.unwrap(), IcaOpenVersion::Ics27(_)));

        let data = "{\"data\":\"eyJmZWVfdmVyc2lvbiI6ImljczI5LTEiLCAiYXBwX3ZlcnNpb24iOiJ7XCJ2ZXJzaW9uXCI6XCJpY3MyNy0xXCIsXCJjb250cm9sbGVyX2Nvbm5lY3Rpb25faWRcIjpcImNvbm5lY3Rpb24tMFwiLFwiaG9zdF9jb25uZWN0aW9uX2lkXCI6XCJjb25uZWN0aW9uLTBcIixcImFkZHJlc3NcIjpcImNvc21vczFsOGRjbG5ubGNza3BqcG4ya3BzMGNhbDZlam5nN3I0ZHlwZXQ2cjJ4ZmR0cmM2MGN5aDZxeHZ5d2NjXCIsXCJlbmNvZGluZ1wiOlwicHJvdG8zXCIsXCJ0eF90eXBlXCI6XCJzZGtfbXVsdGlfbXNnXCJ9In0=\"}";

        let as_json: serde_json::Value = serde_json::from_slice(data.as_bytes()).unwrap();

        let data: Result<IcaOpenVersion,_> = serde_json::from_value(as_json.get("data").unwrap().clone());
        assert!(data.is_ok());
        assert!(matches!(data.unwrap(), IcaOpenVersion::Ics29(_)));
    }
}
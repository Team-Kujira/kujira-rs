//!    Bindings for message execution on Kujira Core

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, Coin, CosmosMsg, CustomMsg, Timestamp, Uint128};

use crate::denom::Denom;

#[cw_serde]
pub enum KujiraMsg {
    Auth(AuthMsg),
    Denom(DenomMsg),
    Intertx(InterTxMsg),
}

impl CustomMsg for KujiraMsg {}

impl From<KujiraMsg> for CosmosMsg<KujiraMsg> {
    fn from(msg: KujiraMsg) -> Self {
        CosmosMsg::Custom(msg)
    }
}

#[cw_serde]
pub enum AuthMsg {
    CreateVestingAccount {
        to_address: Addr,
        amount: Vec<Coin>,
        end_time: Option<Timestamp>,
        delayed: Option<bool>,
    },
}

impl From<AuthMsg> for CosmosMsg<KujiraMsg> {
    fn from(msg: AuthMsg) -> Self {
        KujiraMsg::Auth(msg).into()
    }
}

#[cw_serde]
pub enum DenomMsg {
    Create {
        subdenom: Denom,
    },
    ChangeAdmin {
        denom: Denom,
        address: Addr,
    },
    Mint {
        denom: Denom,
        amount: Uint128,
        recipient: Addr,
    },
    Burn {
        denom: Denom,
        amount: Uint128,
    },
}

impl From<DenomMsg> for CosmosMsg<KujiraMsg> {
    fn from(msg: DenomMsg) -> Self {
        KujiraMsg::Denom(msg).into()
    }
}

#[cw_serde]
/// Type for wrapping any protobuf message
pub struct ProtobufAny {
    /// **type_url** describes the type of the serialized message
    pub type_url: String,

    ///  **value** must be a valid serialized protocol buffer of the above specified type
    pub value: Binary,
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
pub enum InterTxMsg {
    Register {
        connection_id: String,
        account_id: String,
        version: String,
    },
    Submit {
        connection_id: String,
        account_id: String,
        msgs: Vec<ProtobufAny>,
        memo: String,
        timeout: u64,
    },
}

impl From<InterTxMsg> for CosmosMsg<KujiraMsg> {
    fn from(msg: InterTxMsg) -> Self {
        KujiraMsg::Intertx(msg).into()
    }
}
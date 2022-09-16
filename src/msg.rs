//!    Bindings for message execution on Kujira Core

use cosmwasm_std::{Addr, Coin, CosmosMsg, CustomMsg, Timestamp, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::denom::Denom;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum KujiraMsg {
    Auth(AuthMsg),
    Denom(DenomMsg),
}

impl CustomMsg for KujiraMsg {}

impl From<KujiraMsg> for CosmosMsg<KujiraMsg> {
    fn from(msg: KujiraMsg) -> Self {
        CosmosMsg::Custom(msg)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
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

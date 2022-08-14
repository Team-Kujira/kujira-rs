//!    Bindings for message execution on Kujira Core

use cosmwasm_std::{Addr, CosmosMsg, CustomMsg, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum KujiraMsg {
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
pub enum DenomMsg {
    Create {
        subdenom: String,
    },
    ChangeAdmin {
        denom: String,
        address: Addr,
    },
    Mint {
        denom: String,
        amount: Uint128,
        recipient: Addr,
    },
    Burn {
        denom: String,
        amount: Uint128,
    },
}

impl From<DenomMsg> for CosmosMsg<KujiraMsg> {
    fn from(msg: DenomMsg) -> Self {
        KujiraMsg::Denom(msg).into()
    }
}

//!    Bindings for message execution on Kujira Core

use cosmwasm_std::{Addr, CustomMsg, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum KujiraMsg {
    Denom(DenomMsg),
}

impl CustomMsg for KujiraMsg {}

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

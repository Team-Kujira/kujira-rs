//! Interfaces for the Mint contract for Kujira's USK Stablecoin.
//! Only a single instance of this contract will be deployed, acting
//! simply as an authorized gateway for minting and burning of the stable denom, for all deployed Markets

use cosmwasm_std::{to_binary, Addr, Coin, CosmosMsg, Uint128, WasmMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{denom::Denom, msg::KujiraMsg};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    /// The address permitted to call [ExecuteMsg::Permit]
    pub owner: Addr,

    /// The native denom that this contract is authorized to mint and burn
    pub denom: Denom,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Mint { amount: Uint128, recipient: Addr },
    Burn {},
    Permit { address: Addr },
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConfigResponse {
    pub owner: Addr,
    pub denom: Denom,
    pub permitted: Vec<Addr>,
}

pub fn burn_msg(admin: Addr, coin: Coin) -> CosmosMsg<KujiraMsg> {
    CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: admin.to_string(),
        funds: vec![coin],
        msg: to_binary(&ExecuteMsg::Burn {}).unwrap(),
    })
}

pub fn mint_msg(admin: Addr, amount: Uint128, recipient: Addr) -> CosmosMsg<KujiraMsg> {
    CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: admin.to_string(),
        funds: vec![],
        msg: to_binary(&ExecuteMsg::Mint { amount, recipient }).unwrap(),
    })
}

use cosmwasm_std::{to_binary, Addr, Coin, CosmosMsg, Uint128, WasmMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub denom: String,
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
    pub denom: String,
    pub permitted: Vec<Addr>,
}

pub fn burn_msg(admin: Addr, coin: Coin) -> CosmosMsg {
    CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: admin.to_string(),
        funds: vec![coin],
        msg: to_binary(&ExecuteMsg::Burn {}).unwrap(),
    })
}

pub fn mint_msg(admin: Addr, amount: Uint128, recipient: Addr) -> CosmosMsg {
    CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: admin.to_string(),
        funds: vec![],
        msg: to_binary(&ExecuteMsg::Mint { amount, recipient }).unwrap(),
    })
}

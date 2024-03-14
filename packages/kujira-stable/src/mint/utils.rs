use cosmwasm_std::{to_binary, Addr, Coin, CosmosMsg, Uint128, WasmMsg};
use kujira_std::KujiraMsg;

use super::execute::ExecuteMsg;

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

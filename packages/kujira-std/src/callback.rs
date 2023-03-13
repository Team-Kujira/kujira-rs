use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, Coin, CosmosMsg, WasmMsg};

#[cw_serde]
pub struct Callback(pub Binary);

impl Callback {
    pub fn to_message<T>(&self, cb_addr: &Addr, funds: &[Coin]) -> CosmosMsg<T> {
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: cb_addr.to_string(),
            msg: self.0.clone(),
            funds: funds.to_owned(),
        })
    }

    pub fn into_binary(self) -> Binary {
        self.0
    }
}

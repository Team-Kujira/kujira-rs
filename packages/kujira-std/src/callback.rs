use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Coin, CosmosMsg, Env, WasmMsg};

#[cw_serde]
pub struct Callback(pub Binary);

impl Callback {
    pub fn to_message<T>(&self, env: &Env, funds: &[Coin]) -> CosmosMsg<T> {
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: self.0.clone(),
            funds: funds.to_owned(),
        })
    }
}

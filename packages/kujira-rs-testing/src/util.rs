use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Coin, CosmosMsg, WasmMsg};
use kujira::{CallbackData, CallbackMsg, KujiraMsg};
use serde::Serialize;

#[cw_serde]
enum CallbackExecute {
    Callback(CallbackMsg),
}

pub fn assert_callback(
    data: impl Serialize,
    callback: CallbackData,
    callback_addr: String,
    funds: impl Into<Vec<Coin>>,
    msg: CosmosMsg<KujiraMsg>,
) {
    let callback = CallbackExecute::Callback(CallbackMsg {
        data: to_binary(&data).unwrap(),
        callback,
    });
    assert_eq!(
        msg,
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: callback_addr,
            msg: to_binary(&callback).unwrap(),
            funds: funds.into(),
        })
    );
}

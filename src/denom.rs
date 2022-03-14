//! Convenience functions for generically handling cw20::Denom

use crate::tax::deduct_tax;
use cosmwasm_std::{to_binary, Addr, BankMsg, Coin, CosmosMsg, Deps, StdResult, Uint128, WasmMsg};
use cw20::{Cw20ExecuteMsg, Denom};

/// Convenience function to generate relevant CosmosMsg for sending either Native or CW20 tokens
pub fn send_denom(
    deps: Deps,
    recipient: &Addr,
    denom: &Denom,
    amount: &Uint128,
) -> StdResult<CosmosMsg> {
    match denom {
        Denom::Cw20(contract_addr) => Ok(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_addr.to_string(),
            funds: vec![],
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: recipient.to_string(),
                amount: *amount,
            })?,
        })),
        Denom::Native(native) => Ok(CosmosMsg::Bank(BankMsg::Send {
            to_address: recipient.to_string(),
            amount: vec![deduct_tax(
                deps,
                Coin {
                    denom: native.clone(),
                    amount: *amount,
                },
            )?],
        })),
    }
}

use cosmwasm_std::{Addr, Coin, StdError, StdResult, Uint128};

use crate::denom::Denom;

pub fn amount(denom: &Denom, funds: Vec<Coin>) -> StdResult<Uint128> {
    let coin = funds
        .iter()
        .find(|d| &Denom::from(d.denom.clone()) == denom);
    match coin {
        None => Err(StdError::not_found(denom.to_string())),
        Some(Coin { amount, .. }) => match funds
            .iter()
            .find(|d| &Denom::from(d.denom.clone()) != denom)
        {
            Some(x) => Err(StdError::invalid_utf8(x.denom.clone())),
            None => Ok(*amount),
        },
    }
}

pub fn fee_address() -> Addr {
    Addr::unchecked("kujira17xpfvakm2amg962yls6f84z3kell8c5lp3pcxh")
}

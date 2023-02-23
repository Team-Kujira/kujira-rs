use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub enum ExecuteMsg {
    Mint { amount: Uint128, recipient: Addr },
    Burn {},
    Permit { address: Addr },
}

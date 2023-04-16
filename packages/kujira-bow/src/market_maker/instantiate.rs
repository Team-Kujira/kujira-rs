use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub fin_contract: Addr,
    pub intervals: Vec<Decimal>,
    pub fee: Decimal,
    pub amp: Decimal,
}

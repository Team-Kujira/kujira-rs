use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use kujira_std::Denom;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub vault_addr: Addr,
    pub orca_addr: Addr,
    pub collateral_denom: Denom,
    pub collateral_oracle_denom: String,
    pub collateral_decimals: u8,
    pub max_ltv: Decimal,
    pub full_liquidation_threshold: Uint128,
    pub partial_liquidation_target: Decimal,
    pub borrow_fee: Decimal,
}

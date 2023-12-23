use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};

use super::DenomInfo;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub bow_contract: Addr,
    pub denoms: [DenomInfo; 2],
    pub vaults: [Option<Addr>; 2],
    pub orcas: [Option<Addr>; 2],
    pub max_ltv: Decimal,
    pub full_liquidation_threshold: Uint128,
    pub partial_liq_fraction: Decimal,
    pub borrow_fee: Decimal,
}

#[cw_serde]
pub struct MigrateMsg {}

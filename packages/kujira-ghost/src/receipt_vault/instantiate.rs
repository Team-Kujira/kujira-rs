use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use kujira_std::Denom;

use crate::common::OracleType;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub denom: Denom,
    pub oracle: OracleType,
    pub decimals: u8,
    pub receipt_denom: String,
    pub debt_token_denom: String,
    pub denom_creation_fee: Uint128,

    /// Interest rate per utilization point
    pub utilization_to_rate: Vec<(Decimal, Decimal)>,
}

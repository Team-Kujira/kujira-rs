use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use kujira_std::Denom;

use crate::common::OracleType;

use crate::interest::InterestCurveType;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub denom: Denom,
    pub oracle: OracleType,
    pub decimals: u8,
    pub denom_creation_fee: Uint128,

    /// Min-utilization to curve mapping.
    pub utilization_to_curve: Vec<(Decimal, InterestCurveType)>,
}

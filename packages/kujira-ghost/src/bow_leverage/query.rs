use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use kujira_std::Denom;

use super::DenomInfo;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},

    #[returns(StatusResponse)]
    Status {},

    #[returns(PositionResponse)]
    Position { idx: Uint128 },

    #[returns(PositionsResponse)]
    PositionsByHolder {
        holder: Addr,
        start_after: Option<Uint128>,
    },
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub bow_contract: Addr,
    pub denoms: [DenomInfo; 2],
    pub vaults: [Option<Addr>; 2],
    pub orcas: [Option<Addr>; 2],
    pub max_ltv: Decimal,
    pub full_liquidation_threshold: Uint128,
    pub partial_liquidation_target: Decimal,
    pub borrow_fee: Decimal,
}

#[cw_serde]
pub struct DebtAmount {
    pub denom: Denom,
    pub amount: Uint128,
    pub ratio: Decimal,
}

#[cw_serde]
pub struct PositionResponse {
    pub idx: Uint128,
    pub holder: Addr,
    pub lp_amount: Uint128,
    pub debt_shares: Vec<DebtAmount>,
    pub ltv: Decimal,
}

#[cw_serde]
pub struct PositionsResponse {
    pub positions: Vec<PositionResponse>,
}

#[cw_serde]
pub struct StatusResponse {
    pub borrowed: Vec<Coin>,
    pub total_lp_amount: Uint128,
}

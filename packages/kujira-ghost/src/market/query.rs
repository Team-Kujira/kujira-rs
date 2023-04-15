use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal, Uint128};
use kujira_std::Denom;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},

    #[returns(StatusResponse)]
    Status {},

    #[returns(PositionResponse)]
    Position { holder: Addr },
}

#[cw_serde]
pub struct ConfigResponse {
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

#[cw_serde]
pub struct StatusResponse {
    pub deposited: Uint128,
    pub borrowed: Uint128,
}

#[cw_serde]
pub struct PositionResponse {
    pub holder: Addr,
    pub collateral_amount: Uint128,
    pub debt_shares: Uint128,
}

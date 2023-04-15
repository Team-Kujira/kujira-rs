use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal, Uint128};
use kujira_std::Denom;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Get the current contract config.
    #[returns(ConfigResponse)]
    Config {},

    /// Get the current contract config.
    #[returns(StatusResponse)]
    Status {},

    /// Get the whitelist parameters for all markets
    #[returns(MarketsResponse)]
    Markets {},

    /// Get the whitelist parameters for a given market.
    #[returns(MarketParamsResponse)]
    MarketParams { market: Addr },
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub denom: Denom,
    pub oracle_denom: String,
    pub decimals: u8,
}

#[cw_serde]
pub struct StatusResponse {
    pub deposited: Uint128,
    pub lent: Uint128,
    pub interest_rate: Decimal,
}

#[cw_serde]
pub struct MarketsResponse {
    pub markets: Vec<MarketParamsResponse>,
}

#[cw_serde]
pub struct MarketParamsResponse {
    pub addr: Addr,
    pub borrow_limit: Option<Uint128>,
    pub current_borrows: Uint128,
}

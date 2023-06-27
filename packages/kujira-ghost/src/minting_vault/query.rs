use crate::common::OracleType;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal, Uint128};
use kujira_std::Denom;

pub use crate::basic_vault::query::{MarketParamsResponse, MarketsResponse};
pub use crate::receipt_vault::query::InterestParamsResponse;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Get the current contract config.
    #[returns(ConfigResponse)]
    Config {},
    /// Get current interest, redemption, and debt token info, and deposit/lent amounts.
    #[returns(StatusResponse)]
    Status {},
    /// Get the whitelist parameters for a given market.
    #[returns(MarketParamsResponse)]
    MarketParams { market: Addr },
    /// Get the whitelist parameters for all markets, paginated.
    #[returns(MarketsResponse)]
    Markets {
        start_after: Option<Addr>,
        limit: Option<u32>,
    },
    /// Get the current interest parameters.
    #[returns(InterestParamsResponse)]
    InterestParams {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub denom: Denom,
    pub denom_admin: Addr,
    pub oracle: OracleType,
    pub decimals: u8,
    pub debt_token_denom: String,
}

#[cw_serde]
pub struct StatusResponse {
    pub minted: Uint128,
    pub borrowed: Uint128,
    pub rate: Decimal,
    /// How much "denom" each debt share is valued at
    /// (e.g. at 1.5, each share requires 1.5 denom to be repaid)
    pub debt_share_ratio: Decimal,
}

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal, Uint128};
use kujira_std::Denom;

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
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub denom: Denom,
    pub oracle: OracleType,
    pub decimals: u8,
    pub receipt_denom: String,
    pub debt_token_denom: String,
}

#[cw_serde]
pub struct StatusResponse {
    pub deposited: Uint128,
    pub borrowed: Uint128,
    pub rate: Decimal,
    pub deposit_redemption_ratio: Decimal,
    /// How much "denom" each debt share is valued at
    /// (e.g. at 1.5, each share requires 1.5 denom to be repaid)
    pub debt_share_ratio: Decimal,
}

pub use crate::basic_vault::query::{MarketParamsResponse, MarketsResponse};
use crate::common::OracleType;

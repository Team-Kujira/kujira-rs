use crate::market::ConfigResponse as MarketConfigResponse;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Decimal, Uint128};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},

    #[returns(StatusResponse)]
    Status {},

    #[returns(PositionResponse)]
    Position { idx: Uint128 },
}

#[cw_serde]
pub struct ConfigResponse {
    pub market: MarketConfigResponse,
    pub fin_addr: Addr,
    pub bid_denom: String,
    pub ask_denom: String,
    pub max_orders_per_position: u8,
}

#[cw_serde]
pub struct PositionResponse {
    pub holder: Addr,
    pub debt_shares: Uint128,
    pub ltv: Decimal,
    pub orders: Vec<(Uint128, kujira_fin::OrderResponse)>,
    pub custodied: Vec<Coin>,
}

#[cw_serde]
pub struct StatusResponse {
    pub borrowed: Uint128,
    pub ask_balance: Uint128,
    pub bid_balance: Uint128,
}

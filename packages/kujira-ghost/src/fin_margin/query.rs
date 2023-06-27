use crate::market::{ConfigResponse as MarketConfigResponse, StatusResponse};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

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
    pub market: MarketConfigResponse,
    pub fin_addr: Addr,
}

#[cw_serde]
pub struct PositionResponse {
    pub holder: Addr,
    pub collateral_amount: Uint128,
    pub debt_shares: Uint128,
}

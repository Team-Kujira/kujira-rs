use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Decimal256, Uint128};

use crate::market;

#[cw_serde]
pub enum QueryMsg {
    Config {},
    Status {},
    Position {
        idx: Uint128,
    },
    Positions {
        start_after: Option<Uint128>,
        limit: Option<u32>,
        owner: Option<Addr>,
    },
}
#[cw_serde]
pub struct ConfigResponse {
    pub market: market::ConfigResponse,
    pub fin_address: Addr,
}

#[cw_serde]
pub struct PositionsResponse {
    pub positions: Vec<PositionResponse>,
}

#[cw_serde]
pub struct PositionResponse {
    pub idx: Uint128,

    /// The address managing this position
    pub owner: Addr,

    /// The limit order collateralising this position (None if spot)
    pub order: Option<kujira_fin::OrderResponse>,

    /// The amount of margin provided when opening the position
    pub margin_amount: Uint128,

    /// The principal debt on this position, ie the total amount of stable minted
    pub mint_amount: Uint128,

    /// The amount of interest accrued on the position, based on the current interest_rate,
    /// since the previous withdrawal or liquidation (as these actions both collect interest payments)
    pub interest_amount: Uint128,

    /// The price at which the LTV of this loan will exceed [InstantiateMsg::max_ratio], and must be liquidated.
    pub liquidation_price: Option<Decimal>,

    /// Whether this position is currently safe
    pub is_safe: bool,
}

//! Standardized interface to support custom Swappers in [orca](crate::orca)

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Uint128};

#[cw_serde]
pub enum SwapperQueryMsg {
    SwapRate {
        /// The amount of the returned asset required for this swap.
        ask_amount: Uint128,
    },
}

#[cw_serde]
pub enum SwapperExecuteMsg {
    Swap {},
}

#[cw_serde]
pub struct SwapRateResponse {
    /// The exchange rate that the Swapper can achieve for the amount in [SwapperQueryMsg::SwapRate::ask_amount]
    pub exchange_rate: Decimal,
}

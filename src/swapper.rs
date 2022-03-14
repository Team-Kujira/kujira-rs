//! Standardized interface to support custom Swappers in [orca](crate::orca)

use cosmwasm_std::{Decimal256, Uint256};
use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SwapperQueryMsg {
    SwapRate {
        /// The amount of the returned asset required for this swap.
        ask_amount: Uint256,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SwapperExecuteMsg {
    Receive(Cw20ReceiveMsg),
    /// Execute a Native swap
    Swap {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SwapperReceiveMsg {
    /// Execute a CW20 Swap
    Swap {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct SwapRateResponse {
    /// The exchange rate that the Swapper can achieve for the amount in [SwapperQueryMsg::SwapRate::ask_amount]
    pub ask_amount: Decimal256,
}

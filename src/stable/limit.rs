//! Interfaces for the Limit contract for Kujira's USK Stablecoin. This contract allows
//! users to open leveraged limit orders on FIN by minting USK and borrowing it against
//! the filled value of the limit order
//!
use cosmwasm_std::{Addr, Decimal, Decimal256, Uint128};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    /// Standard Market intantiation paramters
    pub market: crate::stable::market::InstantiateMsg,

    /// The address of the FIN Market that is used to place the limit orders
    pub fin_address: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum ExecuteMsg {
    /// Open a leveraged limit order. This will mint the mint_amount, and add it to the stable provided in the tx,
    /// placing a limit order at the `price` provided. This will then be checked for solvency in a `reply` to ensure
    /// it's not been placed the wrong side of the limit and filled at a price that would instantly render it underwater.
    Open {
        mint_amount: Uint128,
        price: Decimal256,
    },

    /// Close a position
    /// The limit order will be withdrawn, and filled amounts claimed
    /// Filled amounts will be swapped for stable, based on `belief_price` and `max_spread`
    /// The loan principal will be burned
    /// Any interest due will be burned
    /// The remaining stable will be returned to the user
    Close {
        idx: Uint128,
        belief_price: Option<Decimal256>,
        max_spread: Option<Decimal256>,
    },

    /// Executes multiple liquidations.
    Liquidates { indices: Vec<Uint128> },

    /// Updates the config of the contract
    UpdateConfig(ConfigUpdate),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
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

    Liquidatable {
        limit: Option<u32>,
        offset: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConfigUpdate {
    pub market: crate::stable::market::ConfigUpdate,
    pub fin_address: Option<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConfigResponse {
    pub market: crate::stable::market::ConfigResponse,
    pub fin_address: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PositionsResponse {
    pub positions: Vec<PositionResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PositionResponse {
    pub idx: Uint128,

    /// The address managing this position
    pub owner: Addr,

    /// The limit order collateralising this position (None if spot)
    pub order: Option<crate::fin::OrderResponse>,

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

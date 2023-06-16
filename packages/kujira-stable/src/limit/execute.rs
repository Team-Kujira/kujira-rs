use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal256, Uint128};
use kujira_std::CallbackMsg;

use crate::market;

#[cw_serde]
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

    /// Callbacks, for internal use. Cannot (and should not) be called directly.
    Callback(CallbackMsg),
}

#[cw_serde]
pub enum CallbackType {
    /// Simply recalculates the amount of collateral on a position after withdrawing it from FIN.
    /// Used for both Close and Liquidate flows.
    WithdrawOrderCallback { position_idx: Uint128 },
    /// Callback after retracting an order on FIN, for when a position is in the middle of being closed.
    ContinueCloseCallback {
        position_idx: Uint128,
        belief_price: Option<Decimal256>,
        max_spread: Option<Decimal256>,
    },
    /// Final callback of the close process.
    FinishCloseCallback { position_idx: Uint128 },
    /// Callback after retracting an order on FIN, for when a position is in the middle of being liquidated.
    FinishLiquidationCallback { position_idx: Uint128 },
}

#[cw_serde]
pub struct ConfigUpdate {
    pub market: market::ConfigUpdate,
    pub fin_address: Option<Addr>,
}

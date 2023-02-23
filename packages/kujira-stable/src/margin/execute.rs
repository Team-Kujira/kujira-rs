use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal256, Uint128};

use crate::market;

#[cw_serde]
pub enum ExecuteMsg {
    /// Open a margin position. This will mint an amount of stable, and swap it on FIN for
    /// the collateral asset.
    /// That collateral is then posted on the Position as debt, less the funds sent
    /// with the transaction
    /// This requires the sender to provide _at least_ `1 - rate::stable::market::InstantiateMsg::max_liquidation_ratio`
    /// of the total debt amount, in order for the position to open below its liquidation threshold
    Open {
        mint_amount: Uint128,
        swap_amount: Uint128,
        belief_price: Option<Decimal256>,
        max_spread: Option<Decimal256>,
    },

    /// Close a position
    /// Colalteral swapped on FIN, and then the margin + profit is returned
    Close {
        /// If provided, this will close the specific amount of collateral
        /// on the position, so that eg a trader can close out a winning
        /// trade enough to pay off the debt, and just let the remaining
        /// collateral run
        amount: Option<Uint128>,

        belief_price: Option<Decimal256>,
        max_spread: Option<Decimal256>,
    },

    /// Deposit [InstantiateMsg::stable_denom] to maintain the LTV of the loan,
    /// This will call the `burn` function on the underlying position to reduce
    /// the liquidation price
    Burn { address: Option<Addr> },

    /// This allows a trader to extract profit from a trade, minting USK and
    /// increasing the LTV
    Mint { amount: Uint128 },

    /// Liquidate and close the sender's position.
    ///
    /// Close the position.
    /// Pay off interest
    /// Sell the remaining collateral on the open market (potentially on Orca if there's a better
    /// price??).
    /// Pay off the debt on the position
    /// Return remaining funds to the trader    Liquidate {},
    Liquidate { amount: Option<Uint128> },

    /// Executes multiple liquidations.
    Liquidates(market::Liquidates),

    /// Updates the config of the contract
    UpdateConfig(ConfigUpdate),
}

#[cw_serde]
pub struct ConfigUpdate {
    pub market: market::ConfigUpdate,
    pub fin_address: Option<Addr>,
}

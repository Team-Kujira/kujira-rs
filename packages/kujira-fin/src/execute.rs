use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, Coin, Decimal256, Uint128, Uint256};
use kujira_std::{Callback, Precision};

/// Callable interfaces
#[cw_serde]
pub enum ExecuteMsg {
    /// Admin-only. Enables trading.
    Launch {},

    /// Update queue configuration
    UpdateConfig {
        /// Change the owner
        owner: Option<Addr>,

        /// Update the decimal precision
        price_precision: Option<Precision>,

        fee_taker: Option<Decimal256>,

        fee_maker: Option<Decimal256>,

        fee_maker_negative: Option<bool>,
    },

    /// Called by an end-user to place a order
    SubmitOrder {
        /// The price of the order in terms of the quote denom. See [InstantiateMsg::denoms]
        price: Decimal256,
        callback: Option<Callback>,
    },

    /// Executes a market trade based on current order book.
    /// Matches Terraswap, Astroport etc interfaces to be compatible with
    /// existing UIs
    Swap {
        /// Field provided for backward compatibility but ignored. Only a single
        /// asset may be provided for a swap
        offer_asset: Option<Coin>,
        belief_price: Option<Decimal256>,
        max_spread: Option<Decimal256>,
        to: Option<Addr>,

        /// An optional callback that FIN will execute with the funds from the swap.
        /// The callback is executed on the sender's address.
        /// NB: This is currently pre-release, and not yet available on production contracts
        callback: Option<Callback>,
    },

    /// Retract the order and withdraw funds
    RetractOrder {
        /// The order idx to be retracted
        order_idx: Uint128,

        /// The amount of order to retract. IF omitted, the whole order is retracted
        amount: Option<Uint256>,

        /// An optional callback that FIN will execute with the funds from the retraction.
        /// The callback is executed on the sender's address.
        /// NB: This is currently pre-release, and not yet available on production contracts
        callback: Option<Callback>,
    },

    /// Fully retract orders and withdraw funds
    RetractOrders {
        /// The order idxs to be retracted
        order_idxs: Vec<Uint128>,

        /// An optional callback that FIN will execute with the funds from the retractions.
        /// The callback is executed on the sender's address.
        /// NB: This is currently pre-release, and not yet available on production contracts
        callback: Option<Callback>,
    },

    /// Claim filled orders
    WithdrawOrders {
        /// If provided, only the selected orders will be withdrawn.
        /// If omitted, the first 30 orders for the sending address
        /// will be withdrawn       
        order_idxs: Option<Vec<Uint128>>,

        /// An optional callback that FIN will execute with the funds from the withdrawals.
        /// The callback is executed on the sender's address.
        /// NB: This is currently pre-release, and not yet available on production contracts
        callback: Option<Callback>,
    },
}

#[cw_serde]
pub struct NewOrderCallback {
    pub t: Binary,
    pub idx: Uint128,
}

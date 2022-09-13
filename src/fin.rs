//! [Kujira's](https://fin.kujira.app/) 100% on-chain, order-book style decentralised exchange
//! for all CosmWASM compatible Blockchains.

use crate::precision::Precision;
use crate::{asset::Asset, denom::Denom};
use cosmwasm_std::{Addr, Coin, Decimal256, Timestamp, Uint128, Uint256};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// Contract owner. Allowed to update parameters
    pub owner: Addr,

    /// The denoms of the pair. The second denom is the quote denom:__rust_force_expr!
    /// Price of pools[0].denom in terms of pools[1].denom
    /// eg if Denoms == [Kuji, UST] then this pool quotes the UST price of Kuji
    pub denoms: [Denom; 2],

    /// The difference in the decimals counts of the quote base base denom
    /// ie base.decimals - quote.decimals
    /// This is used when validating the quote price of an order against the
    /// price_precision setting
    pub decimal_delta: Option<i8>,

    /// In order to prevent basically a DoS attack with hundreds of pools being created at
    /// insignificant price points, we require a limit to the precision of the pricing
    pub price_precision: Precision,
}

/// Callable interfaces
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Admin-only. Enables trading.
    Launch {},

    /// Update queue configuration
    UpdateConfig {
        /// Change the owner
        owner: Option<Addr>,

        /// Update the decimal precision
        price_precision: Option<Precision>,
    },

    /// Called by an end-user to place a order
    SubmitOrder {
        /// The price of the order in terms of the quote denom. See [InstantiateMsg::denoms]
        price: Decimal256,
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
    },

    /// Retract the order and withdraw funds
    RetractOrder {
        /// The order idx to be retracted
        order_idx: Uint128,

        /// The amount of order to retract. IF omitted, the whole order is retracted
        amount: Option<Uint256>,
    },

    /// Fully retract orders and withdraw funds
    RetractOrders {
        /// The order idxs to be retracted
        order_idxs: Vec<Uint128>,
    },

    /// Claim filled orders
    WithdrawOrders {
        /// If provided, only the selected orders will be withdrawn.
        /// If omitted, the first 30 orders for the sending address
        /// will be withdrawn       
        order_idxs: Option<Vec<Uint128>>,
    },
}

/// Standard interface to query contract state
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Current config. Returns [ConfigResponse]
    Config {},

    /// Simulate an market swap based on the current order book. Returns [terraswap::pair::SimulationResponse]
    Simulation { offer_asset: Asset },

    /// Query a specific order by idx. Returns [OrderResponse]
    Order { order_idx: Uint128 },

    /// Paginate user orders. Upper limit of 30 per page. Returns [OrdersResponse]
    OrdersByUser {
        address: Addr,
        start_after: Option<Uint128>,
        limit: Option<u8>,
    },

    /// Query a specific price. Returns [PriceResponse]
    Price { price: Decimal256 },

    /// Returns the order totals of the current order book, paged out from the spread. Returns [BookResponse]
    Book {
        limit: Option<u8>,
        offset: Option<u8>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    /// See [InstantiateMsg::owner]
    pub owner: Addr,

    /// See [InstantiateMsg::denoms]
    pub denoms: [Denom; 2],

    /// See [InstantiateMsg::price_precision]
    pub price_precision: Precision,

    /// See [InstantiateMsg::decimal_delta]
    pub decimal_delta: i8,

    /// When a book is bootstrapping, it can accept orders but trades are not yet executed
    pub is_bootstrapping: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrderResponse {
    /// A unnique ID for the order
    pub idx: Uint128,

    /// The address used to place the order
    pub owner: Addr,

    /// THe quote price of this order
    pub quote_price: Decimal256,

    /// The denom offered
    pub offer_denom: Denom,

    /// The remaining order amount
    pub offer_amount: Uint256,

    /// Amount of filled order awaiting withdrawal
    pub filled_amount: Uint256,

    /// Timestamp of original creation
    pub created_at: Timestamp,

    /// Offer amount at time of creation
    pub original_offer_amount: Uint256,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrdersResponse {
    pub orders: Vec<OrderResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PoolResponse {
    /// THe quote price of this pool
    pub quote_price: Decimal256,

    /// The offer denom for this pool
    pub offer_denom: Denom,

    /// Total amount of all offers in this pool
    pub total_offer_amount: Uint256,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceResponse {
    /// The two offer pools for this price. The [PoolResponse::offer_denom] will match the order supplied in [InstantiateMsg::denoms]
    pub pools: [PoolResponse; 2],
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BookResponse {
    pub base: Vec<PoolResponse>,
    pub quote: Vec<PoolResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SimulationResponse {
    pub return_amount: Uint256,
    pub spread_amount: Uint256,
    pub commission_amount: Uint256,
}

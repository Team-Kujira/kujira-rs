//! [Kujira's](https://blue.kujira.app/) order-book style decentralised exchange
//! for all CosmWASM compatible Blockchains.

use cosmwasm_std::{Addr, Decimal256, Uint128, Uint256};
use cw20::{Cw20ReceiveMsg, Denom};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Create a new liquidation queue.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// Contract owner. Allowed to update parameters
    pub owner: Addr,

    /// The denoms of the pair
    pub denoms: [Denom; 2],

    /// Order slots are created at specific price points. To support good ranges in a
    /// performant way on a web-based UI, the price bands are trunced to a significant
    /// figure value.
    pub slot_trunc: u8,

    /// The trading fee
    pub fee_amount: Decimal256,

    /// The fee destination address
    pub fee_address: Addr,
}

/// Callable interfaces
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Hook to handle (Cw20ExecuteMsg::Send)
    Receive(Cw20ReceiveMsg),

    /// Update queue configuration
    UpdateConfig {
        /// Change the owner
        owner: Option<Addr>,

        /// Update trading fee
        fee_amount: Option<Decimal256>,

        /// Update fee destination
        fee_address: Option<Addr>,
    },

    /// Called by an end-user to place a bid
    SubmitOrder {
        /// The price of the ask asset, in terms of the offer asset provided
        ask_price: Decimal256,
    },

    /// Retract the bid and withdraw funds
    RetractOrder {
        /// The order idx to be retracted
        order_idx: Uint128,

        /// The amount of order to retract. IF omitted, the whole order is retracted
        amount: Option<Uint256>,
    },

    /// Claim filled orders
    WithdrawOrder {
        /// If provided, only the selected orders will be withdrawn.
        /// If omitted, the first 30 orders for the sending address
        /// will be withdrawn       
        order_idxs: Option<Vec<Uint128>>,
    },
}

/// Support for CW20 send messages.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    /// Called by an end-user to place a bid
    SubmitOrder {
        /// The price of the ask asset, in terms of the offer asset provided
        ask_price: Decimal256,
    },
}

/// Standard interface to query contract state
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Current config. Returns [ConfigResponse]
    Config {},

    /// Simulate an order placement based on the current order book. Returns [SimulationResponse]
    Simulate {
        offer_denom: Denom,
        offer_amount: Uint256,
    },

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

    /// Paginate prices. Upper limit of 30 per page. Returns [PricesResponse]
    Prices {
        start_after: Option<Decimal256>,
        limit: Option<u8>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    /// See [InstantiateMsg::owner]
    pub owner: Addr,

    /// See [InstantiateMsg::denoms]
    pub denoms: [Denom; 2],

    /// See [InstantiateMsg::fee_amount]
    pub fee_amount: Decimal256,

    /// See [InstantiateMsg::fee_address]
    pub fee_address: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SimulationResponse {
    /// The simulated state of the order following submission, including any instant fills
    pub order: OrderResponse,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrderResponse {
    /// A unnique ID for the order
    pub idx: Uint128,

    /// The address used to place the bid
    pub owner: Addr,

    /// The denom provided in [ExecuteMsg::SubmitOrder] or [Cw20HookMsg::SubmitOrder]
    pub ask_denom: Denom,

    /// The ask_price selected in [ExecuteMsg::SubmitOrder::ask_price] or [Cw20HookMsg::SubmitOrder::ask_price]
    pub ask_price: Uint256,

    /// The remaining order amount
    pub offer_amount: Uint256,

    /// Amount of filled bid awaiting withdrawal
    pub ask_amount: Uint256,

    pub product_snapshot: Decimal256,
    pub sum_snapshot: Decimal256,
    pub epoch_snapshot: Uint128,
    pub scale_snapshot: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrdersResponse {
    pub bids: Vec<OrderResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PoolResponse {
    /// The ask price of this pool
    pub ask_price: Decimal256,

    /// The offer denom for this pool
    pub offer_denom: Denom,

    /// Total amount of all offers in this pool
    pub total_offer_amount: Uint256,

    pub sum_snapshot: Decimal256,
    pub product_snapshot: Decimal256,
    pub current_epoch: Uint128,
    pub current_scale: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceResponse {
    /// The two offer pools for this price. The [PoolResponse::offer_denom] will match the order supplied in [InstantiateMsg::denoms]
    pub pools: [PoolResponse; 2],
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PricesResponse {
    pub prices: Vec<PriceResponse>,
}

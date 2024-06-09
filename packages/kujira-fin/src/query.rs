use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal256, SignedDecimal256, Timestamp, Uint128, Uint256};
use cw20::Denom;
use kujira_std::{Asset, Precision};

/// Standard interface to query contract state
#[cw_serde]
pub enum QueryMsg {
    /// Current config. Returns [ConfigResponse]
    Config {},

    /// Queries current oracle config
    Oracles {},

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

#[cw_serde]
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

    /// See [InstantiateMsg::fee_taker]    
    pub fee_taker: Decimal256,

    /// See [InstantiateMsg::fee_maker]
    pub fee_maker: Decimal256,

    /// See [InstantiateMsg::fee_address]
    pub fee_address: Addr,
}

#[cw_serde]
pub struct OraclesReponse {
    /// See [ExecuteMsg::SetOracles]
    pub oracles: Option<(String, String)>,
}

#[cw_serde]
pub struct OrderResponsePrice {
    /// A unique ID for the order
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

#[cw_serde]
pub struct OrderResponseOracle {
    /// A unique ID for the order
    pub idx: Uint128,

    /// The address used to place the order
    pub owner: Addr,

    /// The delta in basis points from the current oracle price
    pub delta: i8,

    /// The price of the order based on current oracle values
    pub price: Decimal256,

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

#[cw_serde]
#[serde(untagged)]
pub enum OrderResponse {
    Price(OrderResponsePrice),
    Oracle(OrderResponseOracle),
}

#[cw_serde]
pub struct OrdersResponse {
    pub orders: Vec<OrderResponse>,
}

#[cw_serde]
pub struct PoolResponse {
    /// THe quote price of this pool
    pub quote_price: Decimal256,

    /// The offer denom for this pool
    pub offer_denom: Denom,

    /// Total amount of all offers in this pool
    pub total_offer_amount: Uint256,
}

#[cw_serde]
pub struct PriceResponse {
    /// The two offer pools for this price. The [PoolResponse::offer_denom] will match the order supplied in [InstantiateMsg::denoms]
    pub pools: [PoolResponse; 2],
}

#[cw_serde]
pub struct BookResponse {
    pub base: Vec<PoolResponse>,
    pub quote: Vec<PoolResponse>,
}

#[cw_serde]
pub struct SimulationResponse {
    pub return_amount: Uint256,
    pub spread_amount: Uint256,
    pub commission_amount: Uint256,
}

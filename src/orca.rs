//! [Kujira's](https://orca.kujira.app/) plug-and-play liquidation queue contract
//! for all CosmWASM compatible Blockchains.
//!
//! Designed to create a backstop of solvency against liquidations on your Protocol, a liquidation
//! queue provides a community-focused and decentralized way of bidding for at-risk collateral. At the same time the
//! gamification of the bidding process creates competition between bidders, allowing market forces to determine
//! a discount percentage.
//!
//! If you're interested in deploying a liquidation queue for your Protocol and having it listed on Orca in front of
//! 50,000+ bidders, reach out in our [Telegragm Group](https://t.me/team_kujira).
//!
//! # Features
//! ### Direct Orca Integration
//! Any contract initialized with this interface can be integrated into the Orca UI at <https://orca.kujira.app/>.
//!
//! The easiest way to do this is by using one of the Code IDs from the deployed contract list below.
//!
//! ### Configurable Pools
//! Full control over the discounts available for bidders. Set the number of pools, and the incremental amount
//! of discount per-pool.
//!
//! ### Closable Pools
//! Optionally close off specific pools from new bids. This can be useful when bootstrapping liquidity in a new
//! liquidation queue, for example, where you close lower percentage pools for a fixed period of time,
//! guaranteeing bidders a minimum discount percentage on liquidations in that time.
//!
//! ### Custom Swappers
//! Allow bidders to place bids in a different asset to the one your Protocol requires repayment in.
//! On Terra we use these for example to support native aUST bidding, where a market requires repayment
//! in UST. The bids can be denominated in yield-bearing aUST right up until the point that the liquidation
//! ocurrs and the market is repaid.
//!
//! ### Delegated Activation
//! No more missed liquidations! When you submit a bid, you can optionally include a delegate address,
//! which is permitted to activate the bid on your behalf. Kujira will have a bot that liquidation queues can
//! be registered with, so that bidders can have their bids automatically activated as soon as the wait_end
//! period expires.
//!
//! # Getting Started
//! ## Deploy A Contract
//! The easiest way to get up and running to to instantiate our own audited contract with the code IDs below.
//! Here is a sample JSON-encoded init message, where the bid denomination is Terra's aUST, and the collateral
//! that is being bid on is the Native Luna, and fees connected to Kujira's [sKUJI Protocol Revenue Pool](https://blue.kujira.app/).
//! ```json
//!  {
//!    "owner": "terra123...",
//!    "market": "terra123...",
//!    "bid_denom": {
//!      "cw20": "terra1ajt556dpzvjwl0kl5tzku3fc3p3knkg9mkv8jl"
//!    },
//!    "collateral_denom": {
//!      "native": "uluna"
//!    },
//!    "bid_threshold": "10000000",
//!    "max_slot": 15,
//!    "premium_rate_per_slot": "0.01",
//!    "waiting_period": 600,
//!    "liquidation_fee": "0.01",
//!    "withdrawal_fee": "0.005",
//!    "fee_address": "terra1vvj874nwtmxk0u0spj83d364xyhqk2e652jrck"
//!  }

//! ```
//!
//! ## Integrating your Market
//!
//! 1. Add the package to Cargo.toml
//!
//! `kujira = "0.1.0"`
//!
//! 2. Extend your contract config to store the address of your new liquidation queue
//! ```rust
//! pub struct Config {
//!   liquidation_contract: Addr,
//!   ..,
//! }
//! ```
//! 3. Transfer funds to the liquidation queue at the point of liquidation
//! ```rust
//! fn liquidate_collateral(deps: DepsMut, ..) -> StdResult<Response> {
//!   // ..
//!   let msg = CosmosMsg::Wasm(WasmMsg::Execute {
//!     contract: config.liquidation_contract,
//!     msg: to_binary(&kujira::orca::ExecuteMsg::ExecuteLiquidation {
//!       exchange_rate,
//!       repay_denom: Denom::Cw20("terra1ajt556dpzvjwl0kl5tzku3fc3p3knkg9mkv8jl"),
//!     }),
//!     funds: coins(collateral_amount, "uluna")
//!   });
//!
//!   Ok(Response::default().add_message(msg))
//! }
//! ```
//!
//! ### Terra
//! - Mainnet: Code ID `3540`
//! - Testnet: Code ID `52647`

use crate::swapper::{SwapperExecuteMsg, SwapperQueryMsg};
use cosmwasm_std::{Addr, Decimal256, Uint128, Uint256};
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg, Denom};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Create a new liquidation queue.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// Contract owner. Allowed to call [ExecuteMsg::UpdateConfig] .
    pub owner: Addr,

    /// Market the holds collateral to be liquidated. Only the market can
    /// call [ExecuteMsg::ExecuteLiquidation] and [Cw20HookMsg::ExecuteLiquidation]
    pub market: Addr,

    /// The denomination of the bids. This is fixed at deployment, however with custom
    /// swappers, the market can be repaid in a different denomination to the bid
    pub bid_denom: Denom,

    /// The collateral that can be liquidated using this liquidation queue
    pub collateral_denom: Denom,

    /// The threshold under which bids are automatically activated when placed
    pub bid_threshold: Uint256,
    /// The total number of pools in this queue
    pub max_slot: u8,
    /// The incremental discount offered per-pool
    pub premium_rate_per_slot: Decimal256,
    /// The amount of time in seconds that a bid must wait until it can be activated
    pub waiting_period: u64,

    /// The amount of the repay amount that is sent to [fee_address](InstantiateMsg::fee_address) when executing a liquidation
    pub liquidation_fee: Decimal256,
    /// The amount of the collateral that is sent to [fee_address](InstantiateMsg::fee_address) when a bid is claimed
    pub withdrawal_fee: Decimal256,
    /// The fee destination address
    pub fee_address: Addr,
}

/// Callable interfaces
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum ExecuteMsg {
    /// Hook to handle (Cw20ExecuteMsg::Send)
    Receive(Cw20ReceiveMsg),

    /// Update queue configuration
    UpdateConfig {
        /// Change the owner
        owner: Option<Addr>,
        /// Change the market
        market: Option<Addr>,

        /// Change the amount of time to wait before a bid can be activated
        waiting_period: Option<u64>,
        /// Change the minimum total bid amount for a bid to require manual activation
        bid_threshold: Option<Uint256>,
        /// Change the discount received per pool
        premium_rate_per_slot: Option<Decimal256>,
        /// Close/open specific pools
        closed_slots: Option<Vec<u8>>,

        /// Update fee taken at liquidation
        liquidation_fee: Option<Decimal256>,
        /// Update fee taken at withdrawal
        withdrawal_fee: Option<Decimal256>,
        /// Update fee destination
        fee_address: Option<Addr>,
    },

    /// Called by an end-user to place a bid
    SubmitBid {
        /// The slot (and therefore discount) selected
        premium_slot: u8,
        /// An optional delegate, who can activate the bid on behalf of the user
        delegate: Option<Addr>,
    },
    /// Retract the bid and withdraw funds
    RetractBid {
        /// The bid idx to be retracted
        bid_idx: Uint128,
        /// The amount of bid to retract. IF omitted, the whole bid is retracted
        amount: Option<Uint256>,
    },

    /// Activate bids to be used for liquidation
    ActivateBids {
        /// If provided, only the selected bids will be activated.
        /// If omitted, the first 30 bids for the sending address
        /// will be activated
        bids_idx: Option<Vec<Uint128>>,
    },
    /// Claim liquidated collateral
    ClaimLiquidations {
        /// If provided, only the selected bids will be claimed.
        /// If omitted, the first 30 bids for the sending address
        /// will be claimed       
        bids_idx: Option<Vec<Uint128>>,
    },

    /// Swap liquidated collateral for the repay denomination.
    /// This is only callable by the `market` as defined in
    /// (InstantiateMsg)
    ExecuteLiquidation {
        /// Optional different address to send the repay amount to
        repay_address: Option<Addr>,

        /// The denomination that the market requires repaying in.
        /// By default this will be the [bid_denom](InstantiateMsg::bid_denom),
        /// however if a compatible Swapper has been
        /// registered with [ExecuteMsg::AddSwapper] then the market
        /// can be repaid in a different denom to that of the bids
        repay_denom: Denom,

        /// The market must provide an exchange rate between the repay
        /// denom and the collateral denom in the form `repay / collateral`
        exchange_rate: Decimal256,
    },
    /// Register a custom swapper to support different [repay](ExecuteMsg::ExecuteLiquidation::repay_denom)
    /// and [bid](InstantiateMsg::bid_denom) denoms
    AddSwapper {
        /// See [SwapperResponse::repay_denom]
        denom: Denom,

        /// See [SwapperResponse::addr]
        address: Addr,
    },

    /// Remove a previously regsitered swapper
    RemoveSwapper { denom: Denom },
}

/// Support for CW20 send messages.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    /// Functionally identical to [ExecuteMsg::ExecuteLiquidation]. Used when [collateral_denom](InstantiateMsg::collateral_denom) is a CW20 token
    ExecuteLiquidation {
        repay_address: Option<Addr>,
        repay_denom: Denom,
        exchange_rate: Decimal256,
    },

    /// Functionally identical to [ExecuteMsg::ExecuteLiquidation]. Used when [bid_denom](InstantiateMsg::bid_denom) is a CW20 token
    SubmitBid {
        premium_slot: u8,
        delegate: Option<Addr>,
    },
}

/// Standard interface to query contract state
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Current config. Returns [ConfigResponse]
    Config {},

    /// Simulate a liquidation based on the current pool balances. Returns [SimulationResponse]
    Simulate {
        collateral_amount: Uint256,
        repay_denom: Denom,
        exchange_rate: Decimal256,
    },

    /// Query a specific bid by idx. Returns [BidResponse]
    Bid { bid_idx: Uint128 },

    /// Paginate user bids. Upper limit of 30 per page. Returns [BidsResponse]
    BidsByUser {
        bidder: Addr,
        start_after: Option<Uint128>,
        limit: Option<u8>,
    },

    /// Query a specific bid pool. Returns [BidPoolResponse]
    BidPool { bid_slot: u8 },

    /// Paginate bid pools. Upper limit of 30 per page. Returns [BidPoolsResponse]
    BidPools {
        start_after: Option<u8>,
        limit: Option<u8>,
    },

    /// Paginate registered swappers. Upper limit of 30 per page. Returns [SwappersResponse]
    Swappers {
        start_after: Option<String>,
        limit: Option<u8>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    /// See [InstantiateMsg::owner]
    pub owner: Addr,
    /// See [InstantiateMsg::market]
    pub market: Addr,
    /// See [InstantiateMsg::bid_denom]
    pub bid_denom: Denom,
    /// See [InstantiateMsg::collateral_denom]
    pub collateral_denom: Denom,
    /// See [InstantiateMsg::bid_threshold]
    pub bid_threshold: Uint256,
    /// See [InstantiateMsg::max_slot]
    pub max_slot: u8,
    /// See [InstantiateMsg::premium_rate_per_slot]
    pub premium_rate_per_slot: Decimal256,
    /// See [ExecuteMsg::UpdateConfig::closed_slots]
    pub closed_slots: Vec<u8>,

    /// See [InstantiateMsg::waiting_period]
    pub waiting_period: u64,

    /// See [InstantiateMsg::liquidation_fee]
    pub liquidation_fee: Decimal256,
    /// See [InstantiateMsg::withdrawal_fee]
    pub withdrawal_fee: Decimal256,
    /// See [InstantiateMsg::fee_address]
    pub fee_address: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SimulationResponse {
    /// A confirmation of the amount of collateral consumed in this liquidation.
    /// The simulation will fail if there are insufficient bids to execute the
    /// liquidation
    pub collateral_amount: Uint256,

    /// The simulated amount repaid to the market
    pub repay_amount: Uint256,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BidResponse {
    /// A unnique ID for the bid
    pub idx: Uint128,

    /// The premium slot selected in [ExecuteMsg::SubmitBid::premium_slot]
    pub premium_slot: u8,

    /// The address used to place the bid
    pub bidder: Addr,

    /// The remaining bid amount
    pub amount: Uint256,
    /// Allocated and unclaimed liquidated collateral
    pub pending_liquidated_collateral: Uint256,
    /// The epoch timestamp at which the bid can be activated.
    /// IF None, it's already active
    pub wait_end: Option<u64>,

    /// An optionally selected delegate address who may activate the bid
    /// on behalf of the bidder
    pub delegate: Option<Addr>,

    pub product_snapshot: Decimal256,
    pub sum_snapshot: Decimal256,
    pub epoch_snapshot: Uint128,
    pub scale_snapshot: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BidsResponse {
    pub bids: Vec<BidResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BidPoolResponse {
    /// Total amount of all active bids in this pool
    pub total_bid_amount: Uint256,
    /// The discount applied to bids in this pool
    pub premium_rate: Decimal256,

    /// Whether or not this pool has been closed with [ExecuteMsg::UpdateConfig::closed_slots]
    pub is_closed: bool,
    pub sum_snapshot: Decimal256,
    pub product_snapshot: Decimal256,
    pub current_epoch: Uint128,
    pub current_scale: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BidPoolsResponse {
    pub bid_pools: Vec<BidPoolResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SwappersResponse {
    pub swappers: Vec<SwapperResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SwapperResponse {
    /// The repay denom that the swapper swaps the bid_denom into
    pub repay_denom: Denom,
    /// The contract address that implements [SwapperQueryMsg] and [SwapperExecuteMsg]
    pub addr: Addr,
}

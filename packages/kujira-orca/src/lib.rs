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
//! `kujira = "0.2.1"`
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
//! - Mainnet: Code ID `3541`
//! - Testnet: Code ID `52750`

pub mod execute;
pub mod instantiate;
pub mod query;
pub mod swapper;

pub use {execute::*, instantiate::*, query::*};

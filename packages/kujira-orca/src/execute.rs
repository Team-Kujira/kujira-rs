use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, Decimal, Uint128};
use kujira_std::{denom::Denom, merkle::Proof};

/// Callable interfaces
#[cw_serde]
#[allow(clippy::large_enum_variant)]
pub enum ExecuteMsg {
    /// Update queue configuration
    UpdateConfig {
        /// Change the owner
        owner: Option<Addr>,

        /// Change the amount of time to wait before a bid can be activated
        waiting_period: Option<u64>,
        /// Change the minimum total bid amount for a bid to require manual activation
        bid_threshold: Option<Uint128>,
        /// Change the discount received per pool
        premium_rate_per_slot: Option<Decimal>,
        /// Close/open specific pools
        closed_slots: Option<Vec<u8>>,

        /// Update fee taken at liquidation
        liquidation_fee: Option<Decimal>,
        /// Update fee taken at withdrawal
        withdrawal_fee: Option<Decimal>,
        /// Update fee destination
        fee_address: Option<Addr>,
    },

    /// Add a market which is allowed to use this queue for liquidations
    AddMarket { address: Addr },

    /// Remove permissions from a market
    RemoveMarket { address: Addr },

    /// Allows a verification step for placing bids
    SetMerkleRoot { root: String },

    /// Removes bid verification
    UnsetMerkleRoot {},

    /// Called by an end-user to place a bid
    SubmitBid {
        /// The slot (and therefore discount) selected
        premium_slot: u8,

        /// An optional delegate, who can activate the bid on behalf of the user
        delegate: Option<Addr>,

        /// Submit a merkle proof if there is a merkle root set
        proof: Option<Proof>,
    },
    /// Retract the bid and withdraw funds
    RetractBid {
        /// The bid idx to be retracted
        bid_idx: Uint128,
        /// The amount of bid to retract. IF omitted, the whole bid is retracted
        amount: Option<Uint128>,
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
        exchange_rate: Decimal,

        /// An optional callback that ORCA will execute with the funds from the liquidation.
        /// The callback is executed on the sender's address.
        /// NB: This is currently pre-release, and not yet available on production contracts
        callback: Option<Binary>,
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

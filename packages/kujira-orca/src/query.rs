use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use kujira_std::{Denom, Proof};

/// Standard interface to query contract state
#[cw_serde]
pub enum QueryMsg {
    /// Current config. Returns [ConfigResponse]
    Config {},

    /// Checks the validity of an address against the merkle root
    Verify { address: Addr, proof: Proof },

    /// Simulate a liquidation based on the current pool balances. Returns [SimulationResponse]
    Simulate {
        collateral_amount: Uint128,
        repay_denom: Denom,
        exchange_rate: Decimal,
    },

    /// Calculates the amount of collateral needed to return a required repay_amount,
    /// based on the current pool balances. Returns [SimulationResponse]
    SimulateReverse {
        repay_amount: Uint128,
        repay_denom: Denom,
        exchange_rate: Decimal,
    },

    /// Given the current collateral and debt amounts, calculates the amount of collateral
    /// that needs to be liquidated to bring the LTV to the target LTV. Returns [SimulationResponse]
    SimulateWithTarget {
        collateral_amount: Uint128,
        debt_amount: Uint128,
        target_ltv: Decimal,
        repay_denom: Denom,
        exchange_rate: Decimal,
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

#[cw_serde]
pub struct ConfigResponse {
    /// See [InstantiateMsg::owner]
    pub owner: Addr,
    /// See [ExecuteMsg::AddMarket]
    pub markets: Vec<Addr>,
    /// See [InstantiateMsg::bid_denom]
    pub bid_denom: Denom,
    /// See [InstantiateMsg::collateral_denom]
    pub collateral_denom: Denom,
    /// See [InstantiateMsg::bid_threshold]
    pub bid_threshold: Uint128,
    /// See [InstantiateMsg::max_slot]
    pub max_slot: u8,
    /// See [InstantiateMsg::premium_rate_per_slot]
    pub premium_rate_per_slot: Decimal,
    /// See [ExecuteMsg::UpdateConfig::closed_slots]
    pub closed_slots: Vec<u8>,

    /// See [InstantiateMsg::waiting_period]
    pub waiting_period: u64,

    /// See [InstantiateMsg::liquidation_fee]
    pub liquidation_fee: Decimal,
    /// See [InstantiateMsg::withdrawal_fee]
    pub withdrawal_fee: Decimal,
    /// See [InstantiateMsg::fee_address]
    pub fee_address: Addr,
}

#[cw_serde]
pub struct SimulationResponse {
    /// A confirmation of the amount of collateral consumed in this liquidation.
    /// The simulation will fail if there are insufficient bids to execute the
    /// liquidation
    pub collateral_amount: Uint128,

    /// The simulated amount repaid to the market
    pub repay_amount: Uint128,
}

#[cw_serde]
pub struct BidResponse {
    /// A unnique ID for the bid
    pub idx: Uint128,

    /// The premium slot selected in [ExecuteMsg::SubmitBid::premium_slot]
    pub premium_slot: u8,

    /// The address used to place the bid
    pub bidder: Addr,

    /// The remaining bid amount
    pub amount: Uint128,
    /// Allocated and unclaimed liquidated collateral
    pub pending_liquidated_collateral: Uint128,
    /// The epoch timestamp at which the bid can be activated.
    /// IF None, it's already active
    pub wait_end: Option<u64>,

    /// An optionally selected delegate address who may activate the bid
    /// on behalf of the bidder
    pub delegate: Option<Addr>,

    pub product_snapshot: Decimal,
    pub sum_snapshot: Decimal,
    pub epoch_snapshot: Uint128,
    pub scale_snapshot: Uint128,
}

#[cw_serde]
pub struct BidsResponse {
    pub bids: Vec<BidResponse>,
}

#[cw_serde]
pub struct BidPoolResponse {
    /// Total amount of all active bids in this pool
    pub total_bid_amount: Uint128,
    /// The discount applied to bids in this pool
    pub premium_rate: Decimal,

    /// Whether or not this pool has been closed with [ExecuteMsg::UpdateConfig::closed_slots]
    pub is_closed: bool,
    pub sum_snapshot: Decimal,
    pub product_snapshot: Decimal,
    pub current_epoch: Uint128,
    pub current_scale: Uint128,
}

#[cw_serde]
pub struct BidPoolsResponse {
    pub bid_pools: Vec<BidPoolResponse>,
}

#[cw_serde]
pub struct SwappersResponse {
    pub swappers: Vec<SwapperResponse>,
}

#[cw_serde]
pub struct SwapperResponse {
    /// The repay denom that the swapper swaps the bid_denom into
    pub repay_denom: Denom,
    /// The contract address that implements [SwapperQueryMsg] and [SwapperExecuteMsg]
    pub addr: Addr,
}

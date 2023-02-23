use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use kujira_std::Denom;

/// Create a new liquidation queue.
#[cw_serde]
pub struct InstantiateMsg {
    /// Contract owner. Allowed to call [ExecuteMsg::UpdateConfig] .
    pub owner: Addr,

    /// The denomination of the bids. This is fixed at deployment, however with custom
    /// swappers, the market can be repaid in a different denomination to the bid
    pub bid_denom: Denom,

    /// The collateral that can be liquidated using this liquidation queue
    pub collateral_denom: Denom,

    /// The threshold under which bids are automatically activated when placed
    pub bid_threshold: Uint128,
    /// The total number of pools in this queue
    pub max_slot: u8,
    /// The incremental discount offered per-pool
    pub premium_rate_per_slot: Decimal,
    /// The amount of time in seconds that a bid must wait until it can be activated
    pub waiting_period: u64,

    /// The amount of the repay amount that is sent to [fee_address](InstantiateMsg::fee_address) when executing a liquidation
    pub liquidation_fee: Decimal,
    /// The amount of the collateral that is sent to [fee_address](InstantiateMsg::fee_address) when a bid is claimed
    pub withdrawal_fee: Decimal,
    /// The fee destination address
    pub fee_address: Addr,
}

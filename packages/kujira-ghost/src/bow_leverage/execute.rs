use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use kujira_std::CallbackMsg;

#[cw_serde]
pub enum ExecuteMsg {
    /// 1. receive funds (info.funds) -> ephemeral balance ++
    /// 2. withdraw LP (lp_withdraw_amount) -> ephemeral balance ++
    /// 3. repay funds (repay_amounts) -> ephemeral balance --
    /// 4. if (should_provide):
    ///     a. borrow funds (borrow_amounts + perfect ratio calc) -> ephemeral balance ++
    ///     b. provide LP -> ephemeral balance --
    /// 5. else: All leftover ephemeral funds returned to position holder.
    /// Note: If the resulting position is left empty, it will be closed and removed.
    Adjust {
        /// Creates new position if none.
        position_idx: Option<Uint128>,
        /// Borrows to increase leverage, added to ephemeral position balance
        borrow_amounts: Vec<Coin>,
        /// How much to provide into the pool. The leftover ephemeral funds will be returned to the position holder.
        provide_amounts: Vec<Coin>,
        /// Repays from ephemeral position balance, excessive funds specified will
        /// be returned to position holder.
        repay_amounts: Vec<Coin>,
        /// Unbonds LP, resultant coins added to ephemeral balance.
        lp_withdraw_amount: Uint128,
        max_slippage: Option<Decimal>,
    },
    /// Liquidate a position. `liquidate_amount` only specifiable during self-liquidation.
    Liquidate {
        position_idx: Uint128,
        liquidate_amount: Option<Uint128>,
    },
    /// Callback entrypoint, only callable by other, permissioned contracts.
    Callback(CallbackMsg),
    /// Update contract config. Only callable by contract admin.
    UpdateConfig(ConfigUpdate),
}

#[cw_serde]
pub struct ConfigUpdate {
    pub owner: Option<Addr>,
    pub bow_contract: Option<Addr>,
    pub vaults: Option<[Option<Addr>; 2]>,
    pub orcas: Option<[Option<Addr>; 2]>,
    pub max_ltv: Option<Decimal>,
    pub full_liquidation_threshold: Option<Uint128>,
    pub partial_liq_fraction: Option<Decimal>,
    pub borrow_fee: Option<Decimal>,
}

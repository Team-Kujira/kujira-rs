use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use kujira_std::CallbackMsg;

#[cw_serde]
pub enum ExecuteMsg {
    /// Deposit a collateral asset into the custody contract.
    Deposit(DepositMsg),
    /// Withdraw a collateral asset from the custody contract.
    Withdraw(WithdrawMsg),
    /// Borrow the specified borrowable asset from the vault.
    Borrow(BorrowMsg),
    /// Repay a borrow.
    Repay(RepayMsg),
    /// Liquidate a position.
    Liquidate(LiquidateMsg),
    /// Self-Liquidate a position.
    SelfLiquidate { amount: Option<Uint128> },
    /// Callback entrypoint, only callable by other, permissioned contracts.
    Callback(CallbackMsg),
    /// Update contract config. Only callable by contract admin.
    UpdateConfig(ConfigUpdate),
}

#[cw_serde]
pub enum CallbackType {
    /// Callback for a deposit.
    BorrowCallback {
        receiver: Addr,
        added_debt_shares: Uint128,
        expected_borrow_return: Uint128,
    },
    /// Callback for a liquidation.
    LiquidateCallback { position_holder: Addr },
}

#[cw_serde]
pub struct DepositMsg {}

#[cw_serde]
pub struct WithdrawMsg {
    pub amount: Uint128,
    pub withdraw_to: Option<Addr>,
}

#[cw_serde]
pub struct BorrowMsg {
    pub amount: Uint128,
}

#[cw_serde]
pub struct RepayMsg {}

#[cw_serde]
pub struct LiquidateMsg {
    pub position_holder: Addr,
}

#[cw_serde]
pub struct ConfigUpdate {
    pub owner: Option<Addr>,
    pub orca_addr: Option<Addr>,
    pub collateral_oracle_denom: Option<String>,
    pub collateral_decimals: Option<u8>,
    pub max_ltv: Option<Decimal>,
    pub full_liquidation_threshold: Option<Uint128>,
    pub partial_liquidation_target: Option<Decimal>,
    pub borrow_fee: Option<Decimal>,
}

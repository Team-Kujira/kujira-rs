use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use kujira_std::{CallbackData, Denom};

#[cw_serde]
pub enum ExecuteMsg {
    /// Deposit the borrowable asset into the money market.
    Deposit(DepositMsg),
    /// Withdraw the borrowable asset from the money market.
    Withdraw(WithdrawMsg),
    /// Borrow the borrowable asset from the money market. Only callable by whitelisted market contracts.
    Borrow(BorrowMsg),
    /// Repay a borrow. Only callable by whitelisted market contracts.
    Repay(RepayMsg),
    /// Whitelist a new market, allowing it to borrow from the money market. Only callable by contract admin.
    WhitelistMarket(MarketConfigMsg),
    /// Update a whitelisted market's parameters. Only callable by contract admin.
    UpdateMarket(MarketConfigMsg),
    /// Update contract config. Only callable by contract admin.
    UpdateConfig(ConfigUpdate),
}

#[cw_serde]
pub struct DepositMsg {
    pub callback: Option<CallbackData>,
}

#[cw_serde]
pub struct WithdrawMsg {
    pub amount: Uint128,
    pub callback: Option<CallbackData>,
}

#[cw_serde]
pub struct BorrowMsg {
    pub amount: Uint128,
    pub callback: Option<CallbackData>,
}

#[cw_serde]
pub struct RepayMsg {
    pub callback: Option<CallbackData>,
}

#[cw_serde]
pub struct MarketConfigMsg {
    pub market: Addr,
    pub borrow_limit: Option<Uint128>,
}

#[cw_serde]
pub struct ConfigUpdate {
    pub owner: Option<Addr>,
    pub denom: Option<Denom>,
    pub oracle_denom: Option<String>,
    pub decimals: Option<u8>,
}

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};
use kujira_std::{CallbackData, Denom};

pub use crate::basic_vault::{BorrowMsg, DepositMsg, MarketConfigMsg, RepayMsg};
use crate::common::OracleType;

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
    /// Update contract interest parameters. Only callable by contract admin.
    UpdateInterest(InterestUpdate),
}

#[cw_serde]
pub struct WithdrawMsg {
    pub callback: Option<CallbackData>,
}

#[cw_serde]
pub struct ConfigUpdate {
    pub owner: Option<Addr>,
    pub denom: Option<Denom>,
    pub oracle: Option<OracleType>,
    pub decimals: Option<u8>,
}

#[cw_serde]
pub struct InterestUpdate {
    /// Interest rate per utilization point
    pub utilization_to_rate: Option<Vec<(Decimal, Decimal)>>,
}

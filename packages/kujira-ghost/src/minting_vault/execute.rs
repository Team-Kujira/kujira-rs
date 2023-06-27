use crate::interest::InterestCurveType;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Uint128};

pub use crate::basic_vault::{BorrowMsg, DepositMsg, MarketConfigMsg, RepayMsg};
pub use crate::receipt_vault::ConfigUpdate;

#[cw_serde]
pub enum ExecuteMsg {
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
pub struct InterestUpdate {
    /// Min-utilization to curve mapping.
    pub utilization_to_curve: Option<Vec<(Decimal, InterestCurveType)>>,
    /// Reference amount that corresponds to utilization = 1.0
    pub full_utilization_amount: Option<Uint128>,
}

use crate::market::ConfigUpdate as MarketConfigUpdate;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal256, Uint128};

#[cw_serde]
pub enum ExecuteMsg {
    /// Open a new position.
    Open(OpenMsg),
    /// Close a position.
    Close(CloseMsg),
    /// Liquidate a position.
    Liquidate(LiquidateMsg),
    /// Borrow callback. Only callable by the vault contract.
    BorrowCallback {
        idx: Uint128,
        added_debt_shares: Uint128,
        expected_borrow_return: Uint128,
    },
    /// Liquidate callback. Only callable by an orca contract.
    LiquidateCallback { position_holder: Addr },
    /// Update contract config. Only callable by contract admin.
    UpdateConfig(ConfigUpdate),
}

#[cw_serde]
pub struct OpenMsg {
    pub borrow_amount: Uint128,
    pub price: Decimal256,
}

#[cw_serde]
pub struct CloseMsg {
    pub idx: Uint128,
    pub belief_price: Option<Decimal256>,
    pub max_spread: Option<Decimal256>,
}

#[cw_serde]
pub struct LiquidateMsg {
    pub idx: Uint128,
}

#[cw_serde]
pub struct ConfigUpdate {
    pub market: MarketConfigUpdate,
    pub fin_addr: Option<Addr>,
}

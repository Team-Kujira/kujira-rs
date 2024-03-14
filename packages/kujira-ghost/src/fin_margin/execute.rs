use crate::market::ConfigUpdate as MarketConfigUpdate;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal256, Uint128};
use kujira_std::CallbackMsg;

#[cw_serde]
pub enum ExecuteMsg {
    /// Open a new position and optionally execute actions.
    Open(OpenMsg),
    /// Execute actions on an existing position. Closes the position if it is empty.
    DoActions(ActionMsg),
    /// Liquidate a position.
    Liquidate(LiquidateMsg),
    /// Post execute check. Internal message only callable by this contract.
    PostExecute { position_idx: Uint128 },
    /// Callback entrypoint, only callable by other, permissioned contracts.
    Callback(CallbackMsg),
    /// Update contract config. Only callable by contract admin.
    UpdateConfig(ConfigUpdate),
}

#[cw_serde]
pub enum MarginAction {
    /// Borrow an asset in a position.
    Borrow { amount: Uint128 },
    /// Repay a borrowed asset in a position.
    Repay { amount: Option<Uint128> },
    /// Swap assets in a position.
    Swap {
        belief_price: Option<Decimal256>,
        max_spread: Option<Decimal256>,
        amount: Coin,
    },
    /// Submit an order through a position.
    SubmitOrder { price: Decimal256, amount: Coin },
    /// Command a position to retract orders.
    RetractOrders {
        /// If None, will retract all orders on this position.
        idxs: Option<Vec<Uint128>>,
    },
    /// Command a position to withdraw orders.
    WithdrawOrders {
        /// If None, will withdraw all orders on this position.
        idxs: Option<Vec<Uint128>>,
    },
    /// Deposit assets into a position.
    Deposit {},
    /// Withdraw assets from a position.
    Withdraw { amount: Option<Vec<Coin>> },
}

#[cw_serde]
#[derive(Default)]
pub struct LiquidationSimulation {
    pub repay_amount: Uint128,
    pub liquidate_amount: Uint128,
    pub debt_shares_removed: Uint128,
}

#[cw_serde]
pub struct OpenMsg {
    pub actions: Vec<MarginAction>,
}

#[cw_serde]
pub struct ActionMsg {
    /// Index of the position to execute a trade on.
    pub idx: Uint128,
    pub actions: Vec<MarginAction>,
}

#[cw_serde]
pub struct CloseMsg {
    pub idx: Uint128,
}

#[cw_serde]
pub struct LiquidateMsg {
    pub idx: Uint128,
}

#[cw_serde]
pub struct ConfigUpdate {
    pub market: MarketConfigUpdate,
    pub fin_addr: Option<Addr>,
    pub max_orders_per_position: Option<u8>,
}

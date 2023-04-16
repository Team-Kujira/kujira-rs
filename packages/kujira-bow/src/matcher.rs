use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal, Timestamp, Uint128};
use kujira_std::Denom;

#[cw_serde]
pub struct InstantiateMsg {
    pub pool: Addr,
    pub owner: Addr,
    pub fee: Decimal,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<Addr>,
        fee: Option<Decimal>,
    },
    /// Deposit one side of the pair as liquidity. This will either be filled with an existing queue,
    /// or add to the current one
    SubmitOrder { ask: Denom },

    /// Cancels the order and returns the
    CancelOrder {
        idx: Uint128,
        amount: Option<Uint128>,
    },

    /// Withdraws filled amounts
    ClaimOrder { idx: Uint128 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Query a specific order
    #[returns(OrderResponse)]
    Order { idx: Uint128 },

    /// Query a user's orders
    #[returns(OrdersResponse)]
    Orders {
        addr: Addr,
        limit: Option<u8>,
        start_after: Option<Uint128>,
    },

    /// Query a specific MM contract
    #[returns(ConfigResponse)]
    Config {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub pool: Addr,
    pub token: Denom,
    pub denoms: [Denom; 2],
    /// the pools for this queue:
    /// 0: Deposit Base
    /// 1: Deposit Quote
    /// 2: Withdraw Base
    /// 3: Withdraw Quote
    pub pools: [PoolResponse; 4],
}

#[cw_serde]
pub struct PoolResponse {
    pub total: Uint128,
}

#[cw_serde]
pub struct OrdersResponse {
    pub orders: Vec<OrderResponse>,
}

#[cw_serde]
pub struct OrderResponse {
    pub idx: Uint128,
    pub owner: Addr,
    pub ask_denom: Denom,
    pub offer_denom: Denom,
    pub original_offer_amount: Uint128,
    pub offer_amount: Uint128,
    pub filled_amount: Uint128,
    pub created_at: Timestamp,
}

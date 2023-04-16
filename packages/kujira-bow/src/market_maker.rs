use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, Decimal, Uint128};
use kujira_std::{Denom, Precision};
// use kujira_legacy::precision::Precision;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub fin_contract: Addr,
    pub intervals: Vec<Decimal>,
    pub fee: Decimal,
    pub amp: Decimal,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<Addr>,
        intervals: Option<Vec<Decimal>>,
        fee: Option<Decimal>,
        amp: Option<Decimal>,
    },
    Run {},
    Deposit {
        max_slippage: Option<Decimal>,
        /// Optionally add a submsg that is called when the LP tokens are minted, used for auto-stake
        callback: Option<Callback>,
    },
    Withdraw {},
}

#[cw_serde]
pub struct Callback {
    pub contract_addr: String,
    pub msg: Binary,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},

    /// A shortcut for totalling both balances
    #[returns(PoolResponse)]
    Pool {},

    #[returns(kujira_fin::OrdersResponse)]
    Orders {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub denoms: [Denom; 2],
    pub price_precision: Precision,
    pub decimal_delta: i8,
    pub fin_contract: Addr,
    pub intervals: Vec<Decimal>,
    pub fee: Decimal,
    pub amp: Decimal,
}

#[cw_serde]
pub struct PoolResponse {
    pub balances: [Uint128; 2],
}

#[cw_serde]
pub struct MigrateMsg {
    pub intervals: Option<Vec<Decimal>>,
}

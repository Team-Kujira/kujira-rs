use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal, Uint128};
use kujira_std::{Denom, Precision};

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

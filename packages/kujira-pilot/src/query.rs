use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Decimal, Timestamp, Uint128};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},

    #[returns(SaleResponse)]
    Sale { idx: Uint128 },

    #[returns(SalesResponse)]
    Sales {
        start_after: Option<Uint128>,
        limit: Option<u8>,
    },
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub deposit: Coin,
    pub orca_code_id: u64,
    pub sale_fee: Decimal,
    pub withdrawal_fee: Decimal,
}

#[cw_serde]
pub struct SaleResponse {
    pub idx: Uint128,
    pub title: String,
    pub description: String,
    pub price: Decimal,
    pub owner: Addr,
    pub beneficiary: Addr,
    pub amount: Coin,
    pub opens: Timestamp,
    pub closes: Timestamp,
    pub executed: Option<Timestamp>,
    pub retracted: Option<Timestamp>,
    pub orca_address: Addr,
    pub orca_config: kujira_orca::ConfigResponse,
}

#[cw_serde]
pub struct SalesResponse {
    pub sales: Vec<SaleResponse>,
}

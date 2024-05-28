use cosmwasm_schema::cw_serde;
use cosmwasm_std::{QuerierWrapper, StdResult};
use kujira_std::{HumanPrice, KujiraQuerier, KujiraQuery};

#[cw_serde]
pub enum OracleType {
    Static(HumanPrice),
    Live(String),
}

impl OracleType {
    pub fn price(&self, querier: &QuerierWrapper<KujiraQuery>) -> StdResult<HumanPrice> {
        match self {
            OracleType::Static(price) => Ok(*price),
            OracleType::Live(ref key) => KujiraQuerier::new(querier).query_exchange_rate(key),
        }
    }
}

use crate::market::InstantiateMsg as MarketInstantiateMsg;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub market: MarketInstantiateMsg,
    pub fin_addr: Addr,
}

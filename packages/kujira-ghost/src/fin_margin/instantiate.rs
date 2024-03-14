use crate::market::InstantiateMsg as MarketInstantiateMsg;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use kujira_std::Denom;

#[cw_serde]
pub struct InstantiateMsg {
    pub market: MarketInstantiateMsg,
    pub fin_addr: Addr,
    pub bid_denom: Denom,
    pub ask_denom: Denom,
    pub max_orders_per_position: u8,
}

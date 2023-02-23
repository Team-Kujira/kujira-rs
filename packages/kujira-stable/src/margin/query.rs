use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

use crate::market;

#[cw_serde]
pub struct ConfigResponse {
    pub market: market::ConfigResponse,
    pub fin_address: Addr,
}

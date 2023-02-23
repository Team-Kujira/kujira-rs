use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

use crate::market;

#[cw_serde]
pub struct InstantiateMsg {
    /// Standard Market intantiation paramters
    pub market: market::InstantiateMsg,

    /// The address of the FIN Market that is used to place the limit orders
    pub fin_address: Addr,
}

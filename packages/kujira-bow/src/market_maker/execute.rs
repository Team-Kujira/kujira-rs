use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, Decimal};

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

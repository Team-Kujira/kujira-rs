use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};
use kujira_std::CallbackData;

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
        /// Optionally add a return message that is called when the LP tokens are minted
        callback: Option<CallbackData>,
    },
    Withdraw {
        /// Optionally add a return message that is called when the LP tokens are burned
        callback: Option<CallbackData>,
    },
}

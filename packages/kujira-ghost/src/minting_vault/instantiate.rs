use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    #[serde(flatten)]
    pub vault: crate::receipt_vault::InstantiateMsg,
    pub full_utilization_amount: Uint128,
    pub denom_admin: Addr,
}

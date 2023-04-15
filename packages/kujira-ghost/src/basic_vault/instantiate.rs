use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use kujira_std::Denom;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub denom: Denom,
    pub oracle_denom: String,
    pub decimals: u8,
}

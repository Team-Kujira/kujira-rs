use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use kujira_std::denom::Denom;

#[cw_serde]
pub enum QueryMsg {
    Config {},
}
#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub denom: Denom,
    pub permitted: Vec<Addr>,
}

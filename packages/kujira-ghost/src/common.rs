use cosmwasm_schema::cw_serde;
use kujira_std::HumanPrice;

#[cw_serde]
pub enum OracleType {
    Static(HumanPrice),
    Live(String),
}

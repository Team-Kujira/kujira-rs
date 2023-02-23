use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use kujira_std::denom::Denom;

#[cw_serde]
pub struct InstantiateMsg {
    /// The address permitted to call [ExecuteMsg::Permit]
    pub owner: Addr,

    /// The native denom that this contract is authorized to mint and burn
    pub denom: Denom,
}

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    /// The account permitted to update the config
    pub owner: Addr,

    /// The fee to provide an incentive
    pub incentive_fee: Coin,

    /// The minimum amount of a denom that must be provided
    pub incentive_min: Uint128,
}

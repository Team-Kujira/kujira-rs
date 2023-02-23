use cosmwasm_schema::cw_serde;

use cosmwasm_std::Uint128;

use crate::denom::Denom;

#[cw_serde]
pub struct Asset {
    pub info: AssetInfo,
    pub amount: Uint128,
}

#[cw_serde]
pub enum AssetInfo {
    NativeToken { denom: Denom },
}

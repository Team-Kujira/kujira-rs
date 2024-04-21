use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,

    pub deposit: Coin,

    pub orca_code_id: u64,

    pub orca_admin: Option<Addr>,

    pub orca_owner: Option<Addr>,

    /// The amount of the repay amount that is sent to [fee_address](InstantiateMsg::fee_address) when executing a liquidation
    pub sale_fee: Decimal,

    /// The amount of the collateral that is sent to [fee_address](InstantiateMsg::fee_address) when a bid is claimed
    pub withdrawal_fee: Decimal,

    /// The address that receives the fees
    pub fee_address: Addr,
}

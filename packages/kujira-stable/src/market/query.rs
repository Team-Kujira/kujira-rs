use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use kujira_std::denom::Denom;

#[cw_serde]
pub enum QueryMsg {
    Config {},
    Status {},
    Position {
        address: Addr,
    },
    Positions {
        start_after: Option<Addr>,
        limit: Option<u32>,
    },
    Liquidatable {
        limit: Option<u32>,
        offset: Option<u32>,
    },
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub stable_denom: Denom,
    pub stable_denom_admin: Addr,
    pub collateral_denom: Denom,
    pub oracle_denom: String,
    pub max_liquidation_ratio: Decimal,
    pub mint_fee: Decimal,
    pub interest_rate: Decimal,
    pub orca_address: Addr,
    pub max_debt: Uint128,
    pub liquidation_threshold: Uint128,
    pub partial_liquidation_target: Decimal,
}

#[cw_serde]
pub struct PositionsResponse {
    pub positions: Vec<PositionResponse>,
}

#[cw_serde]
pub struct PositionResponse {
    /// The address managing this position
    pub owner: Addr,

    /// The total amount of collateral denom deposited that can be borrowed against
    pub deposit_amount: Uint128,

    /// The principal debt on this position, ie the total amount of stable minted
    pub mint_amount: Uint128,

    /// The amount of interest accrued on the position, based on the current interest_rate,
    /// since the previous withdrawal or liquidation (as these actions both collect interest payments)
    pub interest_amount: Uint128,

    /// The price at which the LTV of this loan will exceed [InstantiateMsg::max_liquidation_ratio], and must be liquidated.
    pub liquidation_price: Option<Decimal>,
}

#[cw_serde]
pub struct StatusResponse {
    pub debt_amount: Uint128,
}

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use kujira_std::denom::Denom;

#[cw_serde]
pub struct InstantiateMsg {
    /// The owner of the market, able to change config params
    pub owner: Addr,

    /// The denom that is minted and burned
    pub stable_denom: Denom,

    /// The "Router" contract that burns and mints the stable on behalf of all markets
    pub stable_denom_admin: Addr,

    /// The collateral denom supported by this contract for deposits
    pub collateral_denom: Denom,

    /// The different in the decimals between the minted denom and the collateral denom.
    /// Eg usat (Nomic micro-satoshi) has 6 decimals, making 1 BTC 14 decimals.
    /// USK decimals = 6, collateral_decimal_delta = 8
    pub collateral_denom_decimal_delta: i8,

    /// The name of the Oracle price feed used to calculate loan health
    pub oracle_denom: String,

    /// The maximum LTV ratio that a loan can have before it needs liqudiating
    pub max_liquidation_ratio: Decimal,

    /// The amount charged when stable is minted
    pub mint_fee: Decimal,

    /// The yearly interest rate charged on the loan principal
    pub interest_rate: Decimal,

    /// The address of the Orca Liqudiation Queue used for liqudiations
    pub orca_address: Addr,

    /// The maximum amount of stable token that this market is able to mint
    pub max_debt: Uint128,

    /// The quantity of stable below which a position is 100% liquidated
    /// when called
    pub liquidation_threshold: Uint128,

    /// The percentage of collateral that is liquidated when the amount of debt on
    /// a position is above [InstantiateMsg::liquidation_threshold]
    pub partial_liquidation_target: Decimal,
}

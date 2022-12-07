//! Interfaces for the Market contract for Kujira's USK Stablecoin. Each instantiation of this
//! contract will manage debt positions for all users for a specific collateral type

use cosmwasm_std::{Addr, Decimal, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::denom::Denom;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
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

    /// The maximum amount of collateral that a position can have deposited, as a percentage
    /// of the total collateral locked.
    /// This is a soft constraint designed to keep positions smaller and more easily liquidated
    /// by Orca, particularly when the system is young, and a single position could constitute
    /// a significant percentage of the total locked.
    pub collateral_limit: Decimal,

    /// The name of the Oracle price feed used to calculate loan health
    pub oracle_denom: String,

    /// The maximum LTV ratio that a loan can have before it needs liqudiating
    pub max_ratio: Decimal,

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
    pub liquidation_ratio: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Deposit collateral to the position for an address. The amount of
    /// the [InstantiateMsg::collateral_denom] sent with the transaction is the amount
    /// deposited
    /// Collateral can be deposited onto another user's position
    Deposit { address: Option<Addr> },

    /// Withdraw collateral from a position for an address.
    /// When collateral is withdrawn, the [PositionResponse::interest_amount] is
    /// deducted from the total withdrawn at the current oracle rate,
    /// and sent to the `fee_collector` account.
    Withdraw { amount: Uint128 },

    /// Mint stable.
    /// When a borrower mints stable, the [InstantiateMsg::mint_fee] is deducted and sent
    /// to the `fee_collector` account
    Mint { amount: Uint128 },

    /// Burn stable.
    /// This repays the debt. The amount of [InstantiateMsg::stable_denom] sent with the transaction
    /// is the amount burned
    /// USK can be burned on another user's position
    Burn { address: Option<Addr> },

    /// Liquidate the sender's position.
    ///
    /// If the [Position::mint_amount] is less than the [InstantiateMsg::liquidation_threshold],
    /// all of the collateral is sold, otherwise only a [InstantiateMsg::liquidation_raio] amount
    /// of the collateral is sold.
    ///
    /// The [Position::interest_amount] is collected from the [Position::deposit_amount] at the
    /// current Oracle rate
    ///
    /// The remaining collateral is sold via the [InstantiateMsg::orca_address]  at a discount
    /// on the market rate, returning an amount of stable tokens
    ///
    /// The amount of stable burned & debt written off is equal to either the amount
    /// returned from Orca, or the total mint_amount on the position, whichever is smaller.
    ///
    /// In the case of a complete liquidation (ie [Position::mint_amount] < [InstantiateMsg::liquidation_threshold]),
    /// the stable returned will be greater than the [Position::mint_amount], due to over-collateralision.
    /// In this instance, the remaining stable will be deposited to the borrower's address
    ///
    /// In the case of a partial liquidation, the amount returned will be less than the
    /// mint_amount, and so only a portion of the debt will be written off.
    Liquidate { amount: Option<Uint128> },

    /// Executes multiple liquidations.
    Liquidates(Liquidates),

    /// Updates the config of the contract
    UpdateConfig(ConfigUpdate),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Liquidates {
    /// This will search for liquidatable positions, according to the paging
    /// parameters. And attempt to liquidate each one
    Auto {
        limit: Option<u32>,
        offset: Option<u32>,
    },
    /// Explicitly set which addresses need liquidating
    Manual { addresses: Vec<Addr> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConfigUpdate {
    pub owner: Option<Addr>,
    pub oracle_denom: Option<String>,
    pub max_ratio: Option<Decimal>,
    pub mint_fee: Option<Decimal>,
    pub interest_rate: Option<Decimal>,
    pub max_debt: Option<Uint128>,
    pub liquidation_threshold: Option<Uint128>,
    pub liquidation_ratio: Option<Decimal>,
    pub orca_address: Option<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConfigResponse {
    pub owner: Addr,
    pub stable_denom: Denom,
    pub stable_denom_admin: Addr,
    pub collateral_denom: Denom,
    pub oracle_denom: String,
    pub max_ratio: Decimal,
    pub mint_fee: Decimal,
    pub interest_rate: Decimal,
    pub orca_address: Addr,
    pub max_debt: Uint128,
    pub liquidation_threshold: Uint128,
    pub liquidation_ratio: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PositionsResponse {
    pub positions: Vec<PositionResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
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

    /// The price at which the LTV of this loan will exceed [InstantiateMsg::max_ratio], and must be liquidated.
    pub liquidation_price: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct StatusResponse {
    pub debt_amount: Uint128,
}

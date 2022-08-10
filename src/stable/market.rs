use cosmwasm_std::{Addr, Decimal, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    /// The owner of the market, able to change config params
    pub owner: Addr,

    /// The denom that is minted and burned
    pub stable_denom: String,

    /// The "Router" contract that burns and mints the stable on behalf of all markets
    pub stable_denom_admin: Addr,

    /// The collateral denom supported by this contract for deposits
    pub collateral_denom: String,

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
    /// a position is above `liquidation_threshold`
    pub liquidation_ratio: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Deposit collateral to the position for an address
    Deposit {},
    /// Withdraw collateral from a position for an address
    Withdraw {
        amount: Uint128,
    },
    /// Mint stable
    Mint {
        amount: Uint128,
    },
    /// Burn stable
    Burn {},

    // Liquidate the sender's position
    Liquidate {},

    /// Executes liquidations. If addresses is provided, it will attempt those,
    /// failing if any are still safe.
    /// If not provided, all unsafe positions will be liquidated
    Liquidates {
        addresses: Option<Vec<Addr>>,
    },

    /// Updates the config of the contract
    UpdateConfig(ConfigUpdate),
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
    Position {
        address: Addr,
    },
    Positions {
        start_after: Option<Addr>,
        limit: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConfigResponse {
    pub owner: Addr,
    pub denom_admin: Addr,
    pub collateral_denom: String,
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
    pub owner: Addr,
    pub deposit_amount: Uint128,
    pub mint_amount: Uint128,
    pub interest_amount: Uint128,
    pub liquidation_price: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}

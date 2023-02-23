use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};

#[cw_serde]
#[allow(clippy::large_enum_variant)]
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

#[cw_serde]
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

#[cw_serde]
pub struct ConfigUpdate {
    pub owner: Option<Addr>,
    pub oracle_denom: Option<String>,
    pub max_liquidation_ratio: Option<Decimal>,
    pub mint_fee: Option<Decimal>,
    pub interest_rate: Option<Decimal>,
    pub max_debt: Option<Uint128>,
    pub liquidation_threshold: Option<Uint128>,
    pub partial_liquidation_target: Option<Decimal>,
    pub orca_address: Option<Addr>,
}

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Uint128};
use kujira_std::{Denom, Schedule};

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<Addr>,
        incentive_fee: Option<Coin>,
        incentive_min: Option<Uint128>,
    },
    /// Provide a token to stake
    Stake { addr: Option<Addr> },
    /// Claim all staking rewards for the sender
    Claim { denom: Denom },
    /// Withdraw an amount of a staked asset
    Withdraw { amount: Coin },
    /// Add a staking schedule to a staked asset
    AddIncentive { denom: Denom, schedule: Schedule },
}

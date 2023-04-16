use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Timestamp, Uint128};
use kujira_std::Denom;

use crate::schedule::{Release, Schedule};

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct InstantiateMsg {
    /// The account permitted to update the config
    pub owner: Addr,

    /// The fee to provide an incentive
    pub incentive_fee: Coin,

    /// The minimum amount of a denom that must be provided
    pub incentive_min: Uint128,
}

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

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},

    #[returns(PoolResponse)]
    Pool { denom: Denom },

    #[returns(PoolsResponse)]
    Pools {},

    #[returns(IncentivesResponse)]
    Incentives {
        denom: Denom,
        start_after: Option<Timestamp>,
        limit: Option<u8>,
    },

    #[returns(StakeResponse)]
    Stake { denom: Denom, addr: Addr },

    #[returns(StakesResponse)]
    Stakes { addr: Addr },

    #[returns(Vec<Coin>)]
    Fills { denom: Denom, addr: Addr },
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub incentive_fee: Coin,
    pub incentive_min: Uint128,
}

#[cw_serde]
pub struct PoolResponse {
    pub denom: Denom,
    pub total: Uint128,
}

#[cw_serde]
pub struct PoolsResponse {
    pub pools: Vec<PoolResponse>,
}

#[cw_serde]
pub struct IncentiveResponse {
    pub denom: Denom,
    pub schedule: ScheduleResponse,
}

#[cw_serde]
pub struct IncentivesResponse {
    pub incentives: Vec<IncentiveResponse>,
}

#[cw_serde]
pub struct ScheduleResponse {
    pub start: Timestamp,
    pub end: Timestamp,
    pub release: Release,
    pub amount: Uint128,
}

#[cw_serde]
pub struct StakeResponse {
    pub owner: Addr,
    pub denom: Denom,
    pub amount: Uint128,
}

#[cw_serde]
pub struct StakesResponse {
    pub stakes: Vec<StakeResponse>,
}

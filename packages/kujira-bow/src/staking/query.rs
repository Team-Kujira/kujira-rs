use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Timestamp, Uint128};
use kujira_std::{Denom, Release};

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

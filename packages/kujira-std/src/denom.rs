//! Convenience functions for generically handling cw20::Denom
use std::fmt::Display;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, coins, Addr, BankMsg, Coin, CosmosMsg, QuerierWrapper, StdResult};

use crate::query::KujiraQuery;

#[cw_serde]
#[derive(Hash, Eq, PartialOrd, Ord)]
pub struct Denom(String);

impl Denom {
    pub fn coins<T>(&self, amount: &T) -> Vec<Coin>
    where
        T: Clone + Into<u128>,
    {
        coins(amount.clone().into(), self.0.clone())
    }

    pub fn coin<T>(&self, amount: &T) -> Coin
    where
        T: Clone + Into<u128>,
    {
        coin(amount.clone().into(), self.0.clone())
    }

    pub fn send<T, M>(&self, to: &Addr, amount: &T) -> CosmosMsg<M>
    where
        T: Into<u128> + Clone,
    {
        CosmosMsg::Bank(BankMsg::Send {
            to_address: to.to_string(),
            amount: self.coins(amount),
        })
    }

    pub fn query_balance(&self, q: QuerierWrapper<KujiraQuery>, addr: &Addr) -> StdResult<Coin> {
        q.query_balance(addr.clone(), self.0.to_string())
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn from_cw20(value: cw20::Denom) -> Self {
        match value {
            cw20::Denom::Native(x) => Self::from(x),
            cw20::Denom::Cw20(_) => panic!("CW20 Unsupported"),
        }
    }
}

impl Display for Denom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> From<T> for Denom
where
    T: Into<String>,
{
    fn from(t: T) -> Self {
        Self(t.into())
    }
}

impl AsRef<str> for Denom {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

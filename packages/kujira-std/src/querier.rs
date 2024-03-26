//!    Custom querier implementation for Kujira's chain core bindings

use cosmwasm_std::{Addr, Deps, DepsMut, QuerierWrapper, QueryRequest, StdResult};

use crate::{
    denom::Denom,
    price::HumanPrice,
    query::{
        AccountAddressResponse, BankQuery, ExchangeRateResponse, IcaQuery, KujiraQuery,
        OracleQuery, SupplyResponse,
    },
};

/// This is a helper wrapper to easily use our custom queries
pub struct KujiraQuerier<'a> {
    querier: &'a QuerierWrapper<'a, KujiraQuery>,
}

impl<'a> KujiraQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, KujiraQuery>) -> Self {
        KujiraQuerier { querier }
    }

    /// Queries the oracle module for the exchange rate of the specified denom.
    /// This returns a `HumanPrice`, which is a wrapper around `Decimal` that
    /// should be normalized before use in calculations.
    pub fn query_exchange_rate<T: Into<String>>(&self, denom: T) -> StdResult<HumanPrice> {
        let query = KujiraQuery::Oracle(OracleQuery::ExchangeRate {
            denom: denom.into(),
        });
        let request: QueryRequest<KujiraQuery> = KujiraQuery::into(query);
        let result: ExchangeRateResponse = self.querier.query(&request)?;

        Ok(result.rate.into())
    }

    pub fn query_supply_of(&self, denom: Denom) -> StdResult<SupplyResponse> {
        let query = KujiraQuery::Bank(BankQuery::Supply { denom });
        let request: QueryRequest<KujiraQuery> = KujiraQuery::into(query);
        self.querier.query(&request)
    }

    pub fn inner(&self) -> &QuerierWrapper<'a, KujiraQuery> {
        self.querier
    }

    pub fn query_interchain_address(
        &self,
        owner: Addr,
        connection_id: String,
        account_id: String,
    ) -> StdResult<AccountAddressResponse> {
        let query = KujiraQuery::Ica(IcaQuery::AccountAddress {
            owner,
            connection_id,
            account_id,
        });
        let request: QueryRequest<KujiraQuery> = KujiraQuery::into(query);
        let result: AccountAddressResponse = self.querier.query(&request)?;

        Ok(result)
    }
}

impl<'a> From<&'a QuerierWrapper<'a, KujiraQuery>> for KujiraQuerier<'a> {
    fn from(querier: &'a QuerierWrapper<KujiraQuery>) -> Self {
        KujiraQuerier::new(querier)
    }
}

impl<'a> From<&'a Deps<'a, KujiraQuery>> for KujiraQuerier<'a> {
    fn from(deps: &'a Deps<KujiraQuery>) -> Self {
        KujiraQuerier::new(&deps.querier)
    }
}

impl<'a> From<&'a DepsMut<'a, KujiraQuery>> for KujiraQuerier<'a> {
    fn from(deps: &'a DepsMut<KujiraQuery>) -> Self {
        KujiraQuerier::new(&deps.querier)
    }
}

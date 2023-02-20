//!    Custom querier implementation for Kujira's chain core bindings

use cosmwasm_std::{QuerierWrapper, QueryRequest, StdResult};

use crate::{
    denom::Denom,
    query::{BankQuery, ExchangeRateResponse, KujiraQuery, OracleQuery, SupplyResponse},
};

/// This is a helper wrapper to easily use our custom queries
pub struct KujiraQuerier<'a> {
    querier: &'a QuerierWrapper<'a, KujiraQuery>,
}

impl<'a> KujiraQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper<'a, KujiraQuery>) -> Self {
        KujiraQuerier { querier }
    }

    pub fn query_exchange_rate<T: Into<String>>(
        &self,
        denom: T,
    ) -> StdResult<ExchangeRateResponse> {
        let query = KujiraQuery::Oracle(OracleQuery::ExchangeRate {
            denom: denom.into(),
        });
        let request: QueryRequest<KujiraQuery> = KujiraQuery::into(query);
        self.querier.query(&request)
    }

    pub fn query_supply_of(&self, denom: Denom) -> StdResult<SupplyResponse> {
        let query = KujiraQuery::Bank(BankQuery::Supply { denom });
        let request: QueryRequest<KujiraQuery> = KujiraQuery::into(query);
        self.querier.query(&request)
    }

    pub fn inner(&self) -> &QuerierWrapper<'a, KujiraQuery> {
        self.querier
    }

    // pub fn query_exchange_rates<T: Into<String>>(
    //     &self,
    //     offer: T,
    // ) -> StdResult<Vec<ExchangeRateResponse>> {
    //     let request = OracleQuery::ExchangeRates {
    //         offer: offer.into(),
    //     };
    //     let res: ExchangeRatesResponse = self.querier.custom_query(&request.into())?;
    //     Ok(res.rates)
    // }
}

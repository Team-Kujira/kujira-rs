use anyhow::Result as AnyResult;
use cosmwasm_std::{
    attr, coin,
    testing::{MockApi, MockStorage},
    to_binary, Addr, Coin, Decimal, Empty, Event,
};

use cw_multi_test::{
    App, AppResponse, BankKeeper, BasicAppBuilder, CosmosRouter, FailingDistribution,
    FailingStaking, Module, WasmKeeper,
};

use crate::{
    msg::{DenomMsg, KujiraMsg},
    query::{BankQuery, ExchangeRateResponse, KujiraQuery, OracleQuery, SupplyResponse},
};

pub type CustomApp = App<
    BankKeeper,
    MockApi,
    MockStorage,
    KujiraModule,
    WasmKeeper<KujiraMsg, KujiraQuery>,
    FailingStaking,
    FailingDistribution,
>;

pub fn mock_app(balances: Vec<(Addr, Vec<Coin>)>) -> CustomApp {
    let custom = KujiraModule {
        oracle_price: Decimal::from_ratio(1425u128, 100u128),
    };
    BasicAppBuilder::new_custom()
        .with_custom(custom)
        .build(|router, _, storage| {
            for (addr, coins) in balances {
                router.bank.init_balance(storage, &addr, coins).unwrap();
            }
        })
}

pub struct KujiraModule {
    pub oracle_price: Decimal,
}

impl KujiraModule {
    pub fn set_oracle_price(&mut self, price: Decimal) {
        self.oracle_price = price;
    }
}

impl Module for KujiraModule {
    type ExecT = KujiraMsg;

    type QueryT = KujiraQuery;

    type SudoT = Empty;

    fn execute<ExecC, QueryC>(
        &self,
        _api: &dyn cosmwasm_std::Api,
        _storage: &mut dyn cosmwasm_std::Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &cosmwasm_std::BlockInfo,
        _sender: Addr,
        msg: Self::ExecT,
    ) -> AnyResult<AppResponse>
    where
        ExecC: std::fmt::Debug
            + Clone
            + PartialEq
            + schemars::JsonSchema
            + serde::de::DeserializeOwned
            + 'static,
        QueryC: cosmwasm_std::CustomQuery + serde::de::DeserializeOwned + 'static,
    {
        match msg {
            KujiraMsg::Denom(d) => match d {
                DenomMsg::Create { .. } => Ok(AppResponse {
                    events: vec![],
                    data: None,
                }),
                DenomMsg::Mint {
                    amount,
                    denom,
                    recipient,
                } => Ok(AppResponse {
                    events: vec![Event::new("mint").add_attributes(vec![
                        attr("amount", amount),
                        attr("denom", denom),
                        attr("recipient", recipient),
                    ])],
                    data: None,
                }),
                DenomMsg::Burn { denom, amount } => Ok(AppResponse {
                    events: vec![Event::new("burn")
                        .add_attributes(vec![attr("amount", amount), attr("denom", denom)])],
                    data: None,
                }),
                _ => todo!(),
            },
        }
    }

    fn sudo<ExecC, QueryC>(
        &self,
        _api: &dyn cosmwasm_std::Api,
        _storage: &mut dyn cosmwasm_std::Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &cosmwasm_std::BlockInfo,
        _msg: Self::SudoT,
    ) -> AnyResult<AppResponse>
    where
        ExecC: std::fmt::Debug
            + Clone
            + PartialEq
            + schemars::JsonSchema
            + serde::de::DeserializeOwned
            + 'static,
        QueryC: cosmwasm_std::CustomQuery + serde::de::DeserializeOwned + 'static,
    {
        todo!()
    }

    fn query(
        &self,
        _api: &dyn cosmwasm_std::Api,
        _storage: &dyn cosmwasm_std::Storage,
        _querier: &dyn cosmwasm_std::Querier,
        _block: &cosmwasm_std::BlockInfo,
        request: Self::QueryT,
    ) -> AnyResult<cosmwasm_std::Binary> {
        match request {
            KujiraQuery::Bank(b) => match b {
                BankQuery::Supply { denom } => Ok(to_binary(&SupplyResponse {
                    amount: coin(0u128, denom),
                })?),
            },
            KujiraQuery::Oracle(o) => match o {
                OracleQuery::ExchangeRate { .. } => Ok(to_binary(&ExchangeRateResponse {
                    rate: self.oracle_price,
                })?),
            },
        }
    }
}

use anyhow::Result as AnyResult;
use cosmwasm_std::{
    testing::{MockApi, MockStorage},
    Addr, Coin, Empty,
};

use cw_multi_test::{
    App, AppResponse, BankKeeper, BasicAppBuilder, CosmosRouter, FailingDistribution,
    FailingStaking, Module, WasmKeeper,
};

use crate::{
    msg::{DenomMsg, KujiraMsg},
    query::KujiraQuery,
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
    let custom = KujiraModule {};
    BasicAppBuilder::new_custom()
        .with_custom(custom)
        .build(|router, _, storage| {
            for (addr, coins) in balances {
                router.bank.init_balance(storage, &addr, coins).unwrap();
            }
        })
}

pub struct KujiraModule {}

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
                DenomMsg::Mint { .. } => Ok(AppResponse {
                    events: vec![],
                    data: None,
                }),
                DenomMsg::Burn { .. } => Ok(AppResponse {
                    events: vec![],
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
        _request: Self::QueryT,
    ) -> AnyResult<cosmwasm_std::Binary> {
        todo!()
    }
}

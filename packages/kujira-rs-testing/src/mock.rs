use std::{collections::HashMap, convert::TryInto};

use anyhow::Result as AnyResult;
use cosmwasm_std::{
    attr,
    testing::{MockApi, MockStorage},
    to_binary, Addr, BankMsg, Coin, CosmosMsg, Decimal, Empty, Event, Uint128,
};

use cw_multi_test::{
    App, AppResponse, BankKeeper, BankSudo, BasicAppBuilder, CosmosRouter, DistributionKeeper,
    Module, StakeKeeper, SudoMsg, WasmKeeper,
};

use kujira::{
    BankQuery, DenomMsg, ExchangeRateResponse, KujiraMsg, KujiraQuery, OracleQuery, SupplyResponse,
};

pub type CustomApp = App<
    BankKeeper,
    MockApi,
    MockStorage,
    KujiraModule,
    WasmKeeper<KujiraMsg, KujiraQuery>,
    StakeKeeper,
    DistributionKeeper,
>;

pub fn mock_app(balances: Vec<(Addr, Vec<Coin>)>) -> CustomApp {
    let mut custom = KujiraModule {
        oracle_prices: HashMap::new(),
    };
    custom.set_oracle_price(Decimal::from_ratio(1425u128, 100u128), "factory/owner/coll");
    custom.set_oracle_price(Decimal::one(), "factory/contract0/uusk");
    
    BasicAppBuilder::new_custom()
        .with_custom(custom)
        .build(|router, _, storage| {
            for (addr, coins) in balances {
                router.bank.init_balance(storage, &addr, coins).unwrap();
            }
        })
}

pub struct KujiraModule {
    pub oracle_prices: HashMap<String, Decimal>,
}

impl KujiraModule {
    pub fn set_oracle_price(&mut self, price: Decimal, denom: &str) {
        self.oracle_prices.insert(denom.to_string(), price);
    }
}

impl Module for KujiraModule {
    type ExecT = KujiraMsg;

    type QueryT = KujiraQuery;

    type SudoT = Empty;

    fn execute<ExecC, QueryC>(
        &self,
        api: &dyn cosmwasm_std::Api,
        storage: &mut dyn cosmwasm_std::Storage,
        router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &cosmwasm_std::BlockInfo,
        sender: Addr,
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
            KujiraMsg::Auth(_) => todo!(),
            KujiraMsg::Denom(d) => match d {
                DenomMsg::Create { subdenom } => {
                    storage.set(subdenom.as_bytes(), &Uint128::zero().to_be_bytes());

                    Ok(AppResponse {
                        events: vec![],
                        data: None,
                    })
                }
                DenomMsg::Mint {
                    amount,
                    denom,
                    recipient,
                } => {
                    let mut supply = storage
                        .get(denom.as_bytes())
                        .map(|bz| u128::from_be_bytes(bz.try_into().unwrap()))
                        .map(Uint128::from)
                        .unwrap_or_default();

                    supply += amount;
                    storage.set(denom.as_bytes(), &supply.to_be_bytes());
                    router.sudo(
                        api,
                        storage,
                        block,
                        SudoMsg::Bank(BankSudo::Mint {
                            to_address: recipient.to_string(),
                            amount: denom.coins(&amount),
                        }),
                    )?;
                    Ok(AppResponse {
                        events: vec![Event::new("mint").add_attributes(vec![
                            attr("amount", amount),
                            attr("denom", denom.to_string()),
                            attr("recipient", recipient),
                        ])],
                        data: None,
                    })
                }
                DenomMsg::Burn { denom, amount } => {
                    let mut supply = storage
                        .get(denom.as_bytes())
                        .map(|bz| u128::from_be_bytes(bz.try_into().unwrap()))
                        .map(Uint128::from)
                        .unwrap_or_default();
                    println!("Burning {} of {}", amount, denom);
                    supply -= amount;
                    storage.set(denom.as_bytes(), &supply.to_be_bytes());

                    router.execute(
                        api,
                        storage,
                        block,
                        sender,
                        CosmosMsg::Bank(BankMsg::Burn {
                            amount: denom.coins(&amount),
                        }),
                    )?;

                    Ok(AppResponse {
                        events: vec![Event::new("burn").add_attributes(vec![
                            attr("amount", amount),
                            attr("denom", denom.to_string()),
                        ])],
                        data: None,
                    })
                }
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
        storage: &dyn cosmwasm_std::Storage,
        _querier: &dyn cosmwasm_std::Querier,
        _block: &cosmwasm_std::BlockInfo,
        request: Self::QueryT,
    ) -> AnyResult<cosmwasm_std::Binary> {
        match request {
            KujiraQuery::Bank(b) => match b {
                BankQuery::Supply { denom } => {
                    let supply = storage
                        .get(denom.as_bytes())
                        .map(|bz| u128::from_be_bytes(bz.try_into().unwrap()))
                        .unwrap_or_default();

                    Ok(to_binary(&SupplyResponse {
                        amount: denom.coin(&Uint128::from(supply)),
                    })?)
                }
            },
            KujiraQuery::Oracle(o) => match o {
                OracleQuery::ExchangeRate { denom } => Ok(to_binary(&ExchangeRateResponse {
                    rate: *self.oracle_prices.get(&denom).unwrap_or(&Decimal::zero()),
                })?),
            },
        }
    }
}

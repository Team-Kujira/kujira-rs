use std::{collections::HashMap, convert::TryInto};

use anyhow::{Error, Result as AnyResult};
use cosmwasm_std::{
    attr, testing::MockStorage, to_json_binary, Addr, BankMsg, Coin, CosmosMsg, Decimal, Empty,
    Event, Uint128,
};

use cw_multi_test::{
    App, AppResponse, BankKeeper, BankSudo, BasicAppBuilder, CosmosRouter, DistributionKeeper,
    Module, StakeKeeper, SudoMsg, WasmKeeper,
};

use cw_storage_plus::Map;
use kujira::{
    BankQuery, DenomAdminResponse, DenomMsg, DenomQuery, ExchangeRateResponse, FullDenomResponse,
    KujiraMsg, KujiraQuery, OracleQuery, SupplyResponse,
};

use crate::{address::MockAddressGenerator, api::MockApiBech32};

pub type CustomApp = App<
    BankKeeper,
    MockApiBech32,
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
        .with_api(MockApiBech32::new("kujira"))
        .with_wasm(WasmKeeper::default().with_address_generator(MockAddressGenerator))
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

    fn subdenom_to_full(sender: impl Into<String>, subdenom: impl Into<String>) -> String {
        format!("factory/{}/{}", sender.into(), subdenom.into())
    }
}

static DENOM_ADMINS: Map<String, Addr> = Map::new("denom_admins");

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
            + cosmwasm_std::CustomMsg
            + 'static,
        QueryC: cosmwasm_std::CustomQuery + serde::de::DeserializeOwned + 'static,
    {
        match msg {
            KujiraMsg::Auth(_) | KujiraMsg::Batch(_) | KujiraMsg::Ica(_) => todo!(),
            KujiraMsg::Denom(d) => match d {
                DenomMsg::Create { subdenom } => {
                    let full = Self::subdenom_to_full(sender.clone(), subdenom.to_string());
                    storage.set(full.as_bytes(), &Uint128::zero().to_be_bytes());
                    DENOM_ADMINS.save(storage, full, &sender)?;

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
                    let admin = DENOM_ADMINS.load(storage, denom.to_string())?;
                    if admin != sender {
                        return Err(Error::msg("Unauthorized"));
                    }

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
                DenomMsg::ChangeAdmin { denom, address } => {
                    let admin = DENOM_ADMINS.load(storage, denom.to_string())?;
                    if admin != sender {
                        return Err(Error::msg("Unauthorized"));
                    }
                    DENOM_ADMINS.save(storage, denom.to_string(), &address)?;

                    Ok(AppResponse {
                        events: vec![],
                        data: None,
                    })
                }
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

                    Ok(to_json_binary(&SupplyResponse {
                        amount: denom.coin(&Uint128::from(supply)),
                    })?)
                }
            },
            KujiraQuery::Oracle(o) => match o {
                OracleQuery::ExchangeRate { denom } => Ok(to_json_binary(&ExchangeRateResponse {
                    rate: *self.oracle_prices.get(&denom).unwrap_or(&Decimal::zero()),
                })?),
            },
            KujiraQuery::Denom(msg) => match msg {
                DenomQuery::FullDenom {
                    creator_addr,
                    subdenom,
                } => Ok(to_json_binary(&FullDenomResponse {
                    denom: format!("factory/{}/{}", creator_addr, subdenom).into(),
                })?),
                DenomQuery::DenomAdmin { subdenom } => Ok(to_json_binary(&DenomAdminResponse {
                    admin: DENOM_ADMINS.load(storage, subdenom.to_string())?,
                })?),
            },
            KujiraQuery::Ica(_) => todo!("ICA queries not implemented in mock"),
            KujiraQuery::Ibc(_) => todo!("IBC-Verify queries not implemented in mock"),
        }
    }
}

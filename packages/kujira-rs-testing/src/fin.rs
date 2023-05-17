use std::convert::TryFrom;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    coins, to_binary, Addr, BankMsg, Binary, CosmosMsg, Decimal256, Deps, DepsMut, Env, Fraction,
    MessageInfo, Response, StdError, StdResult, Uint128, Uint256, WasmMsg,
};
use cw20::Denom;
use cw_storage_plus::{Item, Map};
use kujira::{
    fin::{ExecuteMsg, InstantiateMsg, NewOrderData, OrderResponse, QueryMsg},
    KujiraMsg, KujiraQuery,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MockMsg {
    PartialFill { idx: Uint128, amount: Uint128 },
    Fill { idx: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case", untagged)]
pub enum MockExecuteMsg {
    FIN(ExecuteMsg),
    Mock(MockMsg),
}

const STABLE: &str = "factory/contract0/uusk";
const COLLATERAL: &str = "factory/owner/coll";

const CUR_ORDER_IDX: Item<Uint128> = Item::new("cur_order_idx");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
struct Order {
    price: Decimal256,
    denom: String,
    amount: Uint128,
    filled: Uint128,
    owner: Addr,
}
const ORDERS: Map<u128, Order> = Map::new("orders");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<KujiraQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response<KujiraMsg>> {
    CUR_ORDER_IDX.save(deps.storage, &Uint128::zero())?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<KujiraQuery>,
    _env: Env,
    info: MessageInfo,
    msg: MockExecuteMsg,
) -> StdResult<Response<KujiraMsg>> {
    let sender = info.sender.clone();
    match msg {
        MockExecuteMsg::FIN(ExecuteMsg::Swap {
            belief_price,
            callback,
            ..
        }) => {
            let coin = info.funds[0].clone();
            let amount: Uint256 = coin.amount.into();
            let price = belief_price.unwrap_or_else(|| Decimal256::from_ratio(1425u128, 100u128));

            let (price, return_denom) = match coin.denom.as_str() {
                STABLE => (
                    Decimal256::from_ratio(price.denominator(), price.numerator()),
                    COLLATERAL,
                ),
                COLLATERAL => (price, STABLE),
                _ => return Err(StdError::generic_err("Invalid Denom")),
            };

            let return_amount: Uint128 = Uint128::try_from(amount * price)?;

            let message = match callback {
                Some(cb) => CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: sender.to_string(),
                    funds: coins(return_amount.u128(), return_denom),
                    msg: cb.0,
                }),
                None => CosmosMsg::Bank(BankMsg::Send {
                    to_address: sender.to_string(),
                    amount: coins(return_amount.u128(), return_denom),
                }),
            };

            Ok(Response::default()
                .add_message(message)
                .add_attribute("action", "fin-swap"))
        }
        MockExecuteMsg::FIN(ExecuteMsg::SubmitOrder { price, callback }) => {
            let idx = CUR_ORDER_IDX.load(deps.storage)?;
            CUR_ORDER_IDX.save(deps.storage, &(idx + Uint128::from(1u128)))?;
            let coin = info.funds[0].clone();
            let amount = coin.amount;
            let denom = coin.denom;
            let order = Order {
                price,
                amount,
                denom,
                filled: Uint128::zero(),
                owner: sender.clone(),
            };

            ORDERS.save(deps.storage, idx.u128(), &order)?;

            match callback {
                Some(cb) => Ok(Response::default()
                    .add_message(cb.to_message(&sender, &NewOrderData { idx }, vec![])?)
                    .add_attribute("order_idx", idx)
                    .add_attribute("action", "fin-submit-order")),
                None => Ok(Response::default()
                    .add_attribute("order_idx", idx)
                    .add_attribute("action", "fin-submit-order")),
            }
        }
        MockExecuteMsg::FIN(ExecuteMsg::WithdrawOrders {
            order_idxs,
            callback,
        }) => {
            let mut messages = vec![];
            for idx in order_idxs.unwrap_or_default() {
                let mut order = ORDERS.load(deps.storage, idx.u128())?;
                if order.owner != sender {
                    return Err(StdError::generic_err("Not your order"));
                }
                if order.filled.is_zero() {
                    continue;
                }
                let return_denom = match order.denom.as_str() {
                    STABLE => COLLATERAL,
                    COLLATERAL => STABLE,
                    _ => return Err(StdError::generic_err("Invalid Denom")),
                };
                let coin = coins(order.filled.u128(), return_denom);

                order.filled = Uint128::zero();
                ORDERS.save(deps.storage, idx.u128(), &order)?;
                match callback.clone() {
                    Some(cb) => messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                        contract_addr: sender.to_string(),
                        funds: coin,
                        msg: cb.0,
                    })),
                    None => messages.push(CosmosMsg::Bank(BankMsg::Send {
                        to_address: sender.to_string(),
                        amount: coin,
                    })),
                }
            }
            Ok(Response::default()
                .add_messages(messages)
                .add_attribute("action", "fin-withdraw"))
        }
        MockExecuteMsg::FIN(ExecuteMsg::RetractOrder {
            order_idx,
            amount,
            callback,
        }) => {
            let mut order = ORDERS.load(deps.storage, order_idx.u128())?;
            if order.owner != sender {
                return Err(StdError::generic_err("Not your order"));
            }
            let amount = amount
                .map(|a| Uint128::try_from(a).unwrap())
                .unwrap_or(order.amount);
            let return_denom = order.denom.clone();
            let mut coin = coins(amount.u128(), return_denom);
            order.amount = order.amount.checked_sub(amount)?;
            ORDERS.save(deps.storage, order_idx.u128(), &order)?;
            if amount.is_zero() {
                coin = vec![];
            }
            let mut messages = vec![];
            match callback {
                Some(cb) => messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: sender.to_string(),
                    funds: coin,
                    msg: cb.0,
                })),
                None => {
                    if !coin.is_empty() {
                        messages.push(CosmosMsg::Bank(BankMsg::Send {
                            to_address: sender.to_string(),
                            amount: coin,
                        }))
                    }
                }
            };

            Ok(Response::default()
                .add_messages(messages)
                .add_attribute("action", "fin-retract"))
        }
        MockExecuteMsg::Mock(MockMsg::PartialFill { idx, amount }) => {
            let mut order = ORDERS.load(deps.storage, idx.u128())?;
            let price = match order.denom.as_str() {
                STABLE => {
                    Decimal256::from_ratio(order.price.denominator(), order.price.numerator())
                }
                COLLATERAL => order.price,
                _ => return Err(StdError::generic_err("Invalid Denom")),
            };
            let return_amount: Uint128 = Uint128::try_from(Uint256::from(amount) * price)?;
            order.amount = order.amount.checked_sub(amount)?;
            order.filled += return_amount;
            ORDERS.save(deps.storage, idx.u128(), &order)?;

            Ok(Response::default().add_attribute("action", "mock-fill"))
        }
        MockExecuteMsg::Mock(MockMsg::Fill { idx }) => {
            let order = ORDERS.load(deps.storage, idx.u128())?;

            execute(
                deps,
                _env,
                info,
                MockExecuteMsg::Mock(MockMsg::PartialFill {
                    idx,
                    amount: order.amount,
                }),
            )
        }
        _ => Ok(Response::default().add_attribute("action", "fin-UNKNOWN-MSG")),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<KujiraQuery>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Order { order_idx } => {
            let order = ORDERS.load(deps.storage, order_idx.u128())?;
            let res = OrderResponse {
                idx: order_idx,
                owner: order.owner,
                quote_price: order.price,
                offer_denom: Denom::Native(order.denom),
                offer_amount: order.amount.into(),
                filled_amount: order.filled.into(),
                created_at: env.block.time,
                original_offer_amount: order.amount.into(),
            };
            Ok(to_binary(&res)?)
        }
        _ => Ok(Binary::default()),
    }
}

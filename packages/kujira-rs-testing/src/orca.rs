#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    attr, coins, to_binary, BankMsg, Binary, CosmosMsg, Decimal, Deps, DepsMut, Empty, Env,
    Fraction, MessageInfo, Response, StdError, StdResult, Uint128,
};
use cw_storage_plus::Item;
use kujira::{
    amount, fee_address,
    orca::{ExecuteMsg, InstantiateMsg, QueryMsg, SimulationResponse},
    Denom, KujiraMsg, KujiraQuery,
};

const COLLATERAL: &str = "factory/owner/coll";

const LIQUIDATION_FEE: Item<Decimal> = Item::new("liquidation_fee");
const REPAY_DENOM: Item<Denom> = Item::new("repay_denom");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<KujiraQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response<KujiraMsg>> {
    LIQUIDATION_FEE.save(deps.storage, &msg.liquidation_fee)?;
    REPAY_DENOM.save(deps.storage, &msg.bid_denom)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<KujiraQuery>,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<KujiraMsg>> {
    let sender = info.sender.clone();
    match msg {
        ExecuteMsg::ExecuteLiquidation {
            exchange_rate,
            callback,
            ..
        } => {
            let collateral_amount = amount(&COLLATERAL.into(), info.funds)?;

            let net_premium = Decimal::from_ratio(95u128, 100u128);
            let repay_amount = collateral_amount * exchange_rate * net_premium;
            let fee_amount = repay_amount * LIQUIDATION_FEE.load(deps.storage)?;
            let repay_amount = repay_amount - fee_amount;
            let repay_denom = REPAY_DENOM.load(deps.storage)?;

            let mut msgs = vec![];
            if fee_amount.gt(&Uint128::zero()) {
                msgs.push(CosmosMsg::Bank(BankMsg::Send {
                    to_address: fee_address().to_string(),
                    amount: coins(fee_amount.u128(), repay_denom.to_string()),
                }));
            }

            match callback {
                None => msgs.push(CosmosMsg::Bank(BankMsg::Send {
                    to_address: sender.to_string(),
                    amount: coins(repay_amount.u128(), repay_denom.to_string()),
                })),
                Some(cb) => msgs.push(cb.to_message(
                    &sender,
                    Empty {},
                    coins(repay_amount.u128(), repay_denom.to_string()),
                )?),
            }

            Ok(Response::default()
                .add_attributes(vec![
                    attr("action", "execute_liquidation"),
                    attr("collateral_amount", collateral_amount),
                    attr("repay_amount", repay_amount),
                    attr("fee_amount", fee_amount),
                ])
                .add_messages(msgs))
        }
        _ => unimplemented!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<KujiraQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Simulate {
            collateral_amount,
            exchange_rate,
            ..
        } => {
            let net_premium = Decimal::from_ratio(95u128, 100u128);
            let repay_amount = collateral_amount * exchange_rate * net_premium;
            let fee_amount = repay_amount * LIQUIDATION_FEE.load(deps.storage)?;
            let repay_amount = repay_amount - fee_amount;
            let res = SimulationResponse {
                collateral_amount,
                repay_amount,
            };

            to_binary(&res)
        }

        QueryMsg::SimulateReverse {
            repay_amount,
            exchange_rate,
            ..
        } => {
            let repay_amount = Decimal::from_ratio(
                LIQUIDATION_FEE.load(deps.storage)?.denominator(),
                Decimal::one().numerator() - LIQUIDATION_FEE.load(deps.storage)?.numerator(),
            ) * repay_amount;

            let collateral_value =
                repay_amount * Decimal::from_ratio(100u128, 95u128) + Uint128::from(1u128);

            let collateral_amount = Decimal::from_ratio(
                collateral_value * exchange_rate.denominator(),
                exchange_rate.numerator(),
            ) * Uint128::from(1u128)
                + Uint128::from(1u128);

            let res = SimulationResponse {
                collateral_amount,
                repay_amount,
            };

            to_binary(&res)
        }

        QueryMsg::SimulateWithTarget {
            collateral_amount,
            debt_amount,
            target_ltv,
            exchange_rate,
            ..
        } => {
            let current_ltv = Decimal::from_ratio(debt_amount, collateral_amount * exchange_rate);
            if current_ltv <= target_ltv {
                return Err(StdError::generic_err(format!(
                    "Current LTV is already lower than target LTV ({} <= {})",
                    current_ltv, target_ltv
                )));
            }
            if target_ltv >= Decimal::one() {
                return Err(StdError::generic_err("Target LTV must be less than 1"));
            }

            let mut remaining_collateral = collateral_amount;
            let mut remaining_debt = debt_amount;

            let premium_price = Decimal::from_ratio(95u128, 100u128) * exchange_rate;

            let cur_collateral_value = remaining_collateral * premium_price;
            if cur_collateral_value.lt(&remaining_debt) {
                return Err(StdError::generic_err(format!(
                    "Insufficient funds to cover debt ({} < {})",
                    cur_collateral_value, remaining_debt
                )));
            }
            let (numerator, numerator_negative) = {
                // Target * price * collateral - debt
                let lhs = remaining_collateral.mul_ceil(target_ltv * exchange_rate);
                if lhs.gt(&remaining_debt) {
                    (lhs - remaining_debt, false)
                } else {
                    (remaining_debt - lhs, true)
                }
            };
            let (denominator, denominator_negative) = {
                // Target * price - premium_price
                let lhs = target_ltv * exchange_rate;
                if lhs.gt(&premium_price) {
                    (lhs - premium_price, false)
                } else {
                    (premium_price - lhs, true)
                }
            };

            if numerator_negative != denominator_negative {
                return Err(StdError::generic_err(
                "Cannot liquidate to target LTV: numerator and denominator have different signs",
            ));
            }

            let liquidate_amount = numerator.mul_ceil(denominator.inv().unwrap());
            remaining_collateral -= liquidate_amount;
            remaining_debt -= liquidate_amount * premium_price;
            if remaining_collateral.is_zero() {
                return Err(StdError::generic_err("Insufficient funds to cover debt"));
            }
            let new_ltv = Decimal::from_ratio(remaining_debt, remaining_collateral * exchange_rate);
            if new_ltv.abs_diff(target_ltv) > Decimal::percent(2) {
                return Err(StdError::generic_err("Cannot liquidate to target LTV"));
            }

            let repay_amount = debt_amount - remaining_debt;
            let collateral_amount = collateral_amount - remaining_collateral;
            let fee_amount = repay_amount * LIQUIDATION_FEE.load(deps.storage)?;
            let repay_amount = repay_amount - fee_amount;
            let res = SimulationResponse {
                collateral_amount,
                repay_amount,
            };
            Ok(to_binary(&res)?)
        }
        _ => unimplemented!(),
    }
}

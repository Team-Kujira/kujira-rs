//! Terra-specific tax calculations

use cosmwasm_std::{Coin, Decimal256, Deps, Fraction, StdResult, Uint256};
use std::convert::TryInto;
use terra_cosmwasm::TerraQuerier;

pub fn compute_tax(deps: Deps, coin: &Coin) -> StdResult<Uint256> {
    let terra_querier = TerraQuerier::new(&deps.querier);
    let tax_rate = terra_querier.query_tax_rate()?.rate;
    let tax_rate = Decimal256::from_ratio(tax_rate.numerator(), tax_rate.denominator());
    let tax_cap = Uint256::from((terra_querier.query_tax_cap(coin.denom.to_string())?).cap);
    let amount = Uint256::from(coin.amount);

    Ok(std::cmp::min(tax_rate * amount, tax_cap))
}

pub fn deduct_tax(deps: Deps, coin: Coin) -> StdResult<Coin> {
    let tax_amount = compute_tax(deps, &coin)?;
    Ok(Coin {
        denom: coin.denom,
        amount: (Uint256::from(coin.amount) - tax_amount).try_into()?,
    })
}

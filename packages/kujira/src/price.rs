use std::{
    cmp::Ordering,
    ops::{Deref, DerefMut, Div, Mul},
};

use cosmwasm_std::{Decimal, Fraction, StdResult, Uint128};

use crate::{denom::Denom, querier::KujiraQuerier};

pub const REFERENCE_DECIMAL_PLACES: i8 = 6;

/// `HumanPrice` is returned from the oracle querier and is a decimal value
/// representing the exchange rate of the given denom, *without any normalization*.
///
/// # NOTE
/// Denominations with different decimals will have `value = amount * price.normalize(decimals)`
/// So do NOT use this value directly for calculations, but rather use the `NormalizedPrice`
pub struct HumanPrice(Decimal);

impl HumanPrice {
    pub fn normalize(&self, decimals: i8) -> NormalizedPrice {
        NormalizedPrice::from_raw(self.0, decimals)
    }
}

impl From<Decimal> for HumanPrice {
    fn from(value: Decimal) -> Self {
        HumanPrice(value)
    }
}

impl From<HumanPrice> for Decimal {
    fn from(value: HumanPrice) -> Self {
        value.0
    }
}

/// `NormalizedPrice` should be used in all operations involving
/// calculating the value of coins given the oracle price.
/// **When comparing values of non-standard denominations, failing
/// to normalize the price can cause unexpected and incorrect results.**
///
/// Standard denominations have 6 decimal places, so we use that as
/// the reference point.
pub struct NormalizedPrice(Decimal);

impl NormalizedPrice {
    /// This is unsafe because it does not check that the price is
    /// normalized to the reference decimal places.
    /// Most likely during testing.
    pub fn unsafe_unchecked(price: Decimal) -> Self {
        Self(price)
    }

    pub fn from_raw(price: Decimal, decimals: i8) -> Self {
        let delta = REFERENCE_DECIMAL_PLACES - decimals;
        match delta.cmp(&0) {
            Ordering::Equal => Self(price),
            Ordering::Greater => Self(Decimal::from_ratio(
                price.numerator() * Uint128::from(10u128.pow(u32::from(delta.unsigned_abs()))),
                price.denominator(),
            )),
            Ordering::Less => Self(Decimal::from_ratio(
                price.numerator(),
                price.denominator() * Uint128::from(10u128.pow(u32::from(delta.unsigned_abs()))),
            )),
        }
    }

    pub fn from_oracle(querier: KujiraQuerier, denom: &Denom, decimals: i8) -> StdResult<Self> {
        querier
            .query_exchange_rate(denom.to_string())
            .map(|price| price.normalize(decimals))
    }

    pub fn inner(&self) -> Decimal {
        self.0
    }
}

impl Deref for NormalizedPrice {
    type Target = Decimal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NormalizedPrice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<NormalizedPrice> for Decimal {
    fn from(price: NormalizedPrice) -> Self {
        price.0
    }
}

impl Mul<Uint128> for NormalizedPrice {
    type Output = Uint128;

    fn mul(self, rhs: Uint128) -> Self::Output {
        self.0 * rhs
    }
}

impl Mul<NormalizedPrice> for Uint128 {
    type Output = Uint128;

    fn mul(self, rhs: NormalizedPrice) -> Self::Output {
        rhs.0 * self
    }
}

impl Div<Uint128> for NormalizedPrice {
    type Output = Option<Uint128>;

    fn div(self, rhs: Uint128) -> Self::Output {
        self.0.inv().map(|inv| inv * rhs)
    }
}

impl Div<NormalizedPrice> for Uint128 {
    type Output = Option<Uint128>;

    fn div(self, rhs: NormalizedPrice) -> Self::Output {
        rhs.0.inv().map(|inv| inv * self)
    }
}

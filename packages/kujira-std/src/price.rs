use std::{
    cmp::Ordering,
    ops::{Deref, Div, DivAssign, Mul, MulAssign},
};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Fraction, StdResult, Uint128};

use crate::{denom::Denom, querier::KujiraQuerier};

pub const REFERENCE_DECIMAL_PLACES: u8 = 6;

/// `HumanPrice` is returned from the oracle querier and is a decimal value
/// representing the exchange rate of the given denom, *without any normalization*.
///
/// # NOTE
/// Denominations with different decimals will have `value = amount * price.normalize(decimals)`
/// So do NOT use this value directly for calculations, but rather use the `NormalizedPrice`
#[cw_serde]
#[derive(Copy, Eq, PartialOrd, Ord)]
pub struct HumanPrice(Decimal);

impl HumanPrice {
    pub fn normalize(&self, decimals: u8) -> NormalizedPrice {
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
#[cw_serde]
#[derive(Copy, Eq, PartialOrd, Ord)]
pub struct NormalizedPrice(Decimal);

impl NormalizedPrice {
    /// This is unsafe because it does not check that the price is
    /// normalized to the reference decimal places.
    /// Most likely during testing.
    pub fn unsafe_unchecked(price: Decimal) -> Self {
        Self(price)
    }

    pub fn from_raw(price: Decimal, decimals: u8) -> Self {
        // delta is i16 because we subtract two u8s
        let delta: i16 = i16::from(REFERENCE_DECIMAL_PLACES) - i16::from(decimals);
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

    pub fn from_oracle<T: Into<String>>(querier: &KujiraQuerier, denom: T, decimals: u8) -> StdResult<Self> {
        querier
            .query_exchange_rate(denom)
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

impl From<NormalizedPrice> for Decimal {
    fn from(price: NormalizedPrice) -> Self {
        price.0
    }
}

impl Mul<NormalizedPrice> for NormalizedPrice {
    type Output = NormalizedPrice;

    fn mul(self, rhs: NormalizedPrice) -> Self::Output {
        NormalizedPrice(self.0 * rhs.0)
    }
}

impl MulAssign<NormalizedPrice> for NormalizedPrice {
    fn mul_assign(&mut self, rhs: NormalizedPrice) {
        self.0 *= rhs.0
    }
}

impl Div<NormalizedPrice> for NormalizedPrice {
    type Output = NormalizedPrice;

    fn div(self, rhs: NormalizedPrice) -> Self::Output {
        NormalizedPrice(self.0 / rhs.0)
    }
}

impl DivAssign<NormalizedPrice> for NormalizedPrice {
    fn div_assign(&mut self, rhs: NormalizedPrice) {
        self.0 /= rhs.0
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

impl MulAssign<NormalizedPrice> for Uint128 {
    fn mul_assign(&mut self, rhs: NormalizedPrice) {
        *self = *self * rhs.0
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
#[cfg(test)]
mod tests {
    use cosmwasm_std::Decimal;

    use super::{HumanPrice, NormalizedPrice};

    #[test]
    fn serialize_human_price() {
        let price = HumanPrice(Decimal::percent(459));
        let serialized = serde_json::to_string(&price).unwrap();
        assert_eq!(serialized, r#""4.59""#);
    }

    #[test]
    fn deserialize_human_price() {
        let price = HumanPrice(Decimal::percent(459));
        let deserialized: HumanPrice = serde_json::from_str(r#""4.59""#).unwrap();
        assert_eq!(price, deserialized);
    }

    #[test]
    fn serialize_normalized_price() {
        let price = NormalizedPrice(Decimal::percent(459));
        let serialized = serde_json::to_string(&price).unwrap();
        assert_eq!(serialized, r#""4.59""#);
    }

    #[test]
    fn deserialize_normalized_price() {
        let price = NormalizedPrice(Decimal::percent(459));
        let deserialized: NormalizedPrice = serde_json::from_str(r#""4.59""#).unwrap();
        assert_eq!(price, deserialized);
    }
}

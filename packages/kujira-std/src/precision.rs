use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Decimal256, Fraction, Uint128, Uint256};

#[cw_serde]
pub enum Precision {
    SignificantFigures(u8),
    DecimalPlaces(u8),
}

impl Precision {
    pub fn validate<T>(&self, other: &T) -> Option<()>
    where
        T: Precise + PartialEq,
    {
        if &other.round(self) == other {
            Some(())
        } else {
            None
        }
    }
}

pub trait Precise {
    fn round(&self, other: &Precision) -> Self;
}

impl Precise for Decimal {
    fn round(&self, p: &Precision) -> Self {
        match p {
            Precision::SignificantFigures(sf) => {
                let int = self.numerator();
                let len = int.to_string().as_str().bytes().len() as u32;
                let decimals: u32 = len - *sf as u32;
                let pow = Uint128::from(10u128).pow(decimals);
                let truncated = Self::from_ratio(int, pow) * Uint128::from(1u128);
                Self::from_ratio(truncated * pow, self.denominator())
            }
            Precision::DecimalPlaces(dp) => {
                let pow = Uint128::from(10u128).pow(18 - *dp as u32);
                let x = Self::from_ratio(self.numerator(), self.denominator() * pow);
                Self::from_ratio(x.numerator() * pow, x.denominator())
            }
        }
    }
}

impl Precise for Decimal256 {
    fn round(&self, p: &Precision) -> Self {
        match p {
            Precision::SignificantFigures(sf) => {
                let int = self.numerator();
                let len = int.to_string().as_str().bytes().len() as u32;
                let decimals: u32 = len - *sf as u32;
                let pow = Uint256::from(10u128).pow(decimals);
                let truncated = Self::from_ratio(int, pow) * Uint256::from(1u128);
                Self::from_ratio(truncated * pow, self.denominator())
            }
            Precision::DecimalPlaces(dp) => {
                let pow = Uint256::from(10u128).pow(18 - *dp as u32);
                let x = Self::from_ratio(self.numerator(), self.denominator() * pow);
                Self::from_ratio(x.numerator() * pow, x.denominator())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_significant_figures() {
        let p = Precision::SignificantFigures(2);
        assert_eq!(p.validate(&Decimal::from_str("123").unwrap()), None);
        assert_eq!(p.validate(&Decimal::from_str("12").unwrap()), Some(()));
        assert_eq!(p.validate(&Decimal::from_str("12.3").unwrap()), None);
        assert_eq!(p.validate(&Decimal::from_str("1.2").unwrap()), Some(()));
    }

    #[test]
    fn test_decimal_places() {
        let p = Precision::DecimalPlaces(2);
        assert_eq!(p.validate(&Decimal::from_str("123").unwrap()), Some(()));
        assert_eq!(p.validate(&Decimal::from_str("1.23").unwrap()), Some(()));
        assert_eq!(p.validate(&Decimal::from_str("12.343").unwrap()), None);
        assert_eq!(p.validate(&Decimal::from_str("1.2").unwrap()), Some(()));
    }
}

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Decimal256, Fraction, Uint128, Uint256};

#[cw_serde]
pub enum Precision {
    SignificantFigures(u8),
    DecimalPlaces(u8),
}

impl Default for Precision {
    fn default() -> Self {
        Self::DecimalPlaces(0)
    }
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
                // early return for zero significant figures
                if sf == &0 {
                    return Self::zero();
                }
                let int = self.numerator();
                let figures = int.to_string().len() as u32;
                let pow = Uint128::new(10u128).pow(figures - *sf as u32);
                let significant_part = int / pow; // integer division truncates
                let truncated = significant_part * pow;
                Self::new(truncated)
            }
            Precision::DecimalPlaces(dp) => {
                let pow = Uint128::from(10u128).pow(18 - *dp as u32);
                let significant_part = self.numerator() / pow; // integer division truncates
                Self::new(significant_part * pow)
            }
        }
    }
}

impl Precise for Decimal256 {
    fn round(&self, p: &Precision) -> Self {
        match p {
            Precision::SignificantFigures(sf) => {
                // early return for zero significant figures
                if sf == &0 {
                    return Self::zero();
                }
                let int = self.numerator();
                let figures = int.to_string().len() as u32;
                let pow = Uint256::from(10u128).pow(figures - *sf as u32);
                let significant_part = int / pow; // integer division truncates
                let truncated = significant_part * pow;
                Self::new(truncated)
            }
            Precision::DecimalPlaces(dp) => {
                let pow = Uint256::from(10u128).pow(18 - *dp as u32);
                let significant_part = self.numerator() / pow; // integer division truncates
                Self::new(significant_part * pow)
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

    #[test]
    fn test_significant_figures_round() {
        const CASES: [(u8, &str, &str); 7] = [
            (2, "123.456", "120"),
            (4, "12.345678910111213141", "12.34"),
            (18, "0.123456789101213141", "0.123456789101213141"),
            (2, "0.123456", "0.12"),
            (18 + 3, "123.456", "123.456"),
            (0, "123.456", "0"),
            (1, "123", "100"),
        ];

        for (precision, unrounded, expected) in CASES {
            let p = Precision::SignificantFigures(precision);
            assert_eq!(
                Decimal::from_str(unrounded).unwrap().round(&p),
                Decimal::from_str(expected).unwrap()
            );
            assert_eq!(
                Decimal256::from_str(unrounded).unwrap().round(&p),
                Decimal256::from_str(expected).unwrap()
            );
        }
    }

    #[test]
    #[should_panic]
    fn test_significant_figures_round_panic() {
        // not sure if this is intended behavior, but it's what the old code did
        let p = Precision::SignificantFigures(18 + 4);
        _ = Decimal::from_str("123.456").unwrap().round(&p);
    }

    #[test]
    fn test_decimal_places_round() {
        const CASES: [(u8, &str, &str); 6] = [
            (2, "123.456", "123.45"),
            (4, "12.345678910111213141", "12.3456"),
            (18, "0.123456789101213141", "0.123456789101213141"),
            (2, "0.123456", "0.12"),
            (0, "123.456", "123"),
            (1, "123", "123"),
        ];

        for (precision, unrounded, expected) in CASES {
            let p = Precision::DecimalPlaces(precision);
            assert_eq!(
                Decimal::from_str(unrounded).unwrap().round(&p),
                Decimal::from_str(expected).unwrap()
            );
            assert_eq!(
                Decimal256::from_str(unrounded).unwrap().round(&p),
                Decimal256::from_str(expected).unwrap()
            );
        }
    }
}

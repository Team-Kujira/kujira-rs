use std::cmp::{max, min};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Decimal256, Fraction, Timestamp, Uint128, Uint256};

#[cw_serde]
pub struct Schedule {
    pub start: Timestamp,
    pub end: Timestamp,
    pub amount: Uint128,
    pub release: Release,
}

#[cw_serde]
pub enum Release {
    Fixed,
    Decay,
}

impl Schedule {
    pub fn released(&self, start: &Timestamp, end: &Timestamp) -> Uint128 {
        if self.start.seconds() > end.seconds() {
            return Uint128::zero();
        }
        match self.release {
            Release::Fixed => {
                let total_duration = self.end.seconds() - self.start.seconds();
                let start = max(self.start.seconds(), start.seconds());
                let end = min(self.end.seconds(), end.seconds());
                if end <= start {
                    return Uint128::zero();
                }
                let duration = end - start;

                self.amount
                    .mul_floor(Decimal::from_ratio(duration, total_duration))
            }
            Release::Decay => {
                let total_duration = self.end.seconds() - self.start.seconds();
                let start = max(self.start.seconds(), start.seconds());
                let end = min(self.end.seconds(), end.seconds());
                if end <= start {
                    return Uint128::zero();
                }
                let c = Decimal256::from_ratio(
                    Uint256::from(self.amount) * Uint256::from(2u128),
                    total_duration,
                );
                let div = Decimal256::from_ratio(total_duration * total_duration, self.amount);

                let b = Uint256::from(end - self.start.seconds());
                let a = Uint256::from(start - self.start.seconds());
                let b = b.mul_floor(c)
                    - Uint256::one().mul_floor(Decimal256::from_ratio(
                        b * b * div.denominator(),
                        div.numerator(),
                    ));

                let a = a.mul_floor(c)
                    - Uint256::one().mul_floor(Decimal256::from_ratio(
                        a * a * div.denominator(),
                        div.numerator(),
                    ));

                let diff = b.checked_sub(a).unwrap_or_default();
                diff.try_into().unwrap()
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn decay_schedule() {
        let s = Schedule {
            start: Timestamp::from_seconds(1670940900),
            end: Timestamp::from_seconds(1671545400),
            amount: Uint128::from(3000000000u128),
            release: Release::Decay,
        };

        let extra = s.released(
            &Timestamp::from_seconds(1671545401),
            &Timestamp::from_seconds(1671963618),
        );

        println!("extra {extra}");

        let s = Schedule {
            start: Timestamp::from_seconds(0),
            end: Timestamp::from_seconds(1000),
            amount: Uint128::from(5000u128),
            release: Release::Decay,
        };

        //

        assert_eq!(
            s.released(&Timestamp::from_seconds(0), &Timestamp::from_seconds(0)),
            Uint128::zero()
        );
        assert_eq!(
            s.released(&Timestamp::from_seconds(0), &Timestamp::from_seconds(1000)),
            Uint128::from(5000u128)
        );
        assert_eq!(
            s.released(&Timestamp::from_seconds(0), &Timestamp::from_seconds(100)),
            Uint128::from(950u128)
        );
        assert_eq!(
            s.released(&Timestamp::from_seconds(400), &Timestamp::from_seconds(600)),
            Uint128::from(1000u128)
        );

        // last 400 seconds out of 1000
        assert_eq!(
            s.released(&Timestamp::from_seconds(0), &Timestamp::from_seconds(600)),
            Uint128::from(4200u128)
        );
        assert_eq!(
            s.released(
                &Timestamp::from_seconds(600),
                &Timestamp::from_seconds(1200)
            ),
            Uint128::from(800u128)
        );

        // assert_eq!(
        //     s.released(
        //         &Timestamp::from_seconds(1200),
        //         &Timestamp::from_seconds(1400)
        //     ),
        //     Uint128::zero()
        // );

        let s = Schedule {
            start: Timestamp::from_seconds(0),
            end: Timestamp::from_seconds(45000),
            amount: Uint128::from(154600u128),
            release: Release::Decay,
        };

        //

        assert_eq!(
            s.released(&Timestamp::from_seconds(0), &Timestamp::from_seconds(6750)),
            Uint128::from(42901u128)
        );

        let s = Schedule {
            start: Timestamp::from_seconds(20000),
            end: Timestamp::from_seconds(45000),
            amount: Uint128::from(154600u128),
            release: Release::Decay,
        };

        //

        assert_eq!(
            s.released(&Timestamp::from_seconds(0), &Timestamp::from_seconds(6750)),
            Uint128::zero()
        );

        let s = Schedule {
            start: Timestamp::from_seconds(1670940900),
            end: Timestamp::from_seconds(1671545400),
            amount: Uint128::from(3000000000u128),
            release: Release::Decay,
        };

        // total 3k distribution
        // start 1670940900
        // last distributed 1671780641
        // end 1671545400

        // This was distributed since it ended. make sure it's not offering extra rewards

        let extra = s.released(
            &Timestamp::from_seconds(1671545401),
            &Timestamp::from_seconds(1671963618),
        );

        println!("extra {extra}");

        assert_eq!(
            s.released(
                &Timestamp::from_seconds(1671780641),
                &Timestamp::from_seconds(1671785290)
            ),
            Uint128::zero()
        );
    }

    #[test]
    fn decimals() {
        let s = Schedule {
            start: Timestamp::from_seconds(1703083387),
            end: Timestamp::from_seconds(1710974700),
            amount: Uint128::from(65_000_000_000_000_000_000_000u128),
            release: Release::Decay,
        };

        s.released(
            &Timestamp::from_seconds(1703083387),
            &Timestamp::from_seconds(1710974700),
        );
    }
}

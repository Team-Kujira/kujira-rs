use std::cmp::{max, min};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Fraction, Timestamp, Uint128};

use crate::staking::ScheduleResponse;

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

                Decimal::from_ratio(duration, total_duration) * self.amount
            }
            Release::Decay => {
                let total_duration = self.end.seconds() - self.start.seconds();
                let start = max(self.start.seconds(), start.seconds());
                let end = min(self.end.seconds(), end.seconds());
                if end <= start {
                    return Uint128::zero();
                }
                let c = Decimal::from_ratio(self.amount * Uint128::from(2u128), total_duration);
                let div = Decimal::from_ratio(total_duration * total_duration, self.amount);

                let b = Uint128::from(end - self.start.seconds());
                let a = Uint128::from(start - self.start.seconds());

                let b = c * b
                    - Decimal::from_ratio(b * b * div.denominator(), div.numerator())
                        * Uint128::one();

                let a = c * a
                    - Decimal::from_ratio(a * a * div.denominator(), div.numerator())
                        * Uint128::one();

                b.checked_sub(a).unwrap_or_default()
            }
        }
    }
}

impl From<Schedule> for ScheduleResponse {
    fn from(s: Schedule) -> Self {
        Self {
            start: s.start,
            end: s.end,
            release: s.release,
            amount: s.amount,
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

        let _extra = s.released(
            &Timestamp::from_seconds(1671545401),
            &Timestamp::from_seconds(1671963618),
        );

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
}

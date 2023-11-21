use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal;

#[cw_serde]
pub enum InterestCurveType {
    Linear(Linear),
    Exponential(Exponential),
}

impl InterestCurve for InterestCurveType {
    fn get_rate(&self, utilization: Decimal) -> Decimal {
        match self {
            InterestCurveType::Linear(curve) => curve.get_rate(utilization),
            InterestCurveType::Exponential(curve) => curve.get_rate(utilization),
        }
    }
}

pub trait InterestCurve {
    fn get_rate(&self, utilization: Decimal) -> Decimal;
}

#[cw_serde]
/// Linear interest curve
pub struct Linear {
    /// start point on this curve (util, rate)
    pub start: (Decimal, Decimal),
    /// end point on this curve (util, rate)
    pub end: (Decimal, Decimal),
}

impl InterestCurve for Linear {
    fn get_rate(&self, utilization: Decimal) -> Decimal {
        let (start_util, start_rate) = self.start;
        let (end_util, end_rate) = self.end;
        let slope = (end_rate - start_rate) / (end_util - start_util);
        start_rate + slope * (utilization - start_util)
    }
}

#[cw_serde]
/// Exponential interest curve
/// y = (a-1) + c^(b * x)
/// We use (a-1) instead of a to ensure that y(0) = 0
pub struct Exponential {
    /// a: constant
    pub intercept: Decimal,
    /// b: utilization
    pub coefficient: u32,
    /// c: exponent
    pub exponent: Decimal,
}

impl InterestCurve for Exponential {
    fn get_rate(&self, utilization: Decimal) -> Decimal {
        (self.intercept - Decimal::one()) + (self.exponent * utilization).pow(self.coefficient)
    }
}

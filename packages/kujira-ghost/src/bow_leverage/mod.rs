pub mod execute;
pub mod instantiate;
pub mod query;

use cosmwasm_schema::cw_serde;
use kujira_std::Denom;

pub use crate::bow_leverage::{execute::*, instantiate::*, query::*};

#[cw_serde]
pub struct DenomInfo {
    pub denom: Denom,
    pub decimals: u8,
    pub oracle: String,
}

impl ToString for DenomInfo {
    fn to_string(&self) -> String {
        self.denom.to_string()
    }
}

impl ToString for &DenomInfo {
    fn to_string(&self) -> String {
        self.denom.to_string()
    }
}

//! Interfaces for the Margin contract for Kujira's USK Stablecoin. This contract
//! works in a very similar way to Market, except positions are opened with a margin
//! amount of stable, and collateral bought from  FIN, instead of depositing collateral directly
//!
//! These can be paired with Orca markets with much smaller potential premiums, allowing greater leverage

pub mod execute;
pub mod instantiate;
pub mod query;

pub use {
    execute::*,
    instantiate::*,
    query::*,
};
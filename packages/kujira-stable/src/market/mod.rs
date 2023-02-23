//! Interfaces for the Market contract for Kujira's USK Stablecoin. Each instantiation of this
//! contract will manage debt positions for all users for a specific collateral type

pub mod execute;
pub mod instantiate;
pub mod query;

pub use {execute::*, instantiate::*, query::*};

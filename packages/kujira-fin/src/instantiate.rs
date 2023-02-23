use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal256};
use kujira_std::{Denom, Precision};

#[cw_serde]
pub struct InstantiateMsg {
    /// Contract owner. Allowed to update parameters
    pub owner: Addr,

    /// The denoms of the pair. The second denom is the quote denom:__rust_force_expr!
    /// Price of pools[0].denom in terms of pools[1].denom
    /// eg if Denoms == [Kuji, UST] then this pool quotes the UST price of Kuji
    pub denoms: [Denom; 2],

    /// The difference in the decimals counts of the quote base base denom
    /// ie base.decimals - quote.decimals
    /// This is used when validating the quote price of an order against the
    /// price_precision setting
    pub decimal_delta: Option<i8>,

    /// In order to prevent basically a DoS attack with hundreds of pools being created at
    /// insignificant price points, we require a limit to the precision of the pricing
    pub price_precision: Precision,

    /// The fee charged on swaps, and instantly filled limit orders
    pub fee_taker: Decimal256,

    /// The fee charged on withdrawals from filled limit orders
    pub fee_maker: Decimal256,

    /// If true, the fee_taker amount is deducted from the fee_maker fee during swaps, and
    /// sent to the trader when their order is filled and withdrawn
    pub fee_maker_negative: bool,
}

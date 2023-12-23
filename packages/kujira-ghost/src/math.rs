use cosmwasm_std::{Decimal, Uint128};

/// Utility to handle the math of adding debt shares (maximize debt)
pub fn calculate_added_debt(borrow_amount: Uint128, debt_share_price: Decimal) -> Uint128 {
    borrow_amount.div_ceil(debt_share_price)
}

/// Utility to handle the math of removing debt shares (maximize debt)
pub fn calculate_removed_debt(repay_amount: Uint128, debt_share_price: Decimal) -> Uint128 {
    repay_amount.div_floor(debt_share_price)
}

/// Utility to convert debt shares to value of debt (maximize debt)
pub fn debt_to_liability(shares: Uint128, rate: Decimal) -> Uint128 {
    shares.mul_ceil(rate)
}

/// Utility to convert receipt tokens to value of deposit (minimize redemption amount)
pub fn rcpt_tokens_to_owed(tokens: Uint128, rate: Decimal) -> Uint128 {
    tokens.mul_floor(rate)
}

/// Utility to convert deposit value to receipt tokens (minimize receipt tokens received)
pub fn amt_to_rcpt_tokens(amt: Uint128, rate: Decimal) -> Uint128 {
    amt.div_floor(rate)
}

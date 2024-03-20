use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Decimal, Timestamp, Uint128};
use kujira_std::{CallbackMsg, Denom};

#[cw_serde]
pub enum ExecuteMsg {
    /// Creates a new pilot sale
    Create {
        sale: CreateSale,
        orca: CreateOrca,
    },
    /// Executes a pilot sale
    Execute {
        idx: Uint128,
    },
    /// Retracts a pilot sale
    Retract {
        idx: Uint128,
    },
    Orca {
        sale: Uint128,
        msg: kujira_orca::ExecuteMsg,
    },
    UpdateConfig {
        owner: Option<Addr>,
        orca_code_id: Option<u64>,
        sale_fee: Option<Decimal>,
        withdrawal_fee: Option<Decimal>,
        deposit: Option<Coin>,
    },
    /// Updates the sale description
    UpdateSaleDescription {
        idx: Uint128,
        description: String,
    },
    Callback(CallbackMsg),
}

#[cw_serde]
pub struct CreateSale {
    pub title: String,

    pub description: String,

    pub url: String,

    /// The address that the raise will be sent to
    pub beneficiary: Addr,

    /// Base price of the token at sale
    pub price: Decimal,

    pub opens: Timestamp,

    /// The time after which the sale can be executed
    pub closes: Timestamp,
}

#[cw_serde]
pub struct CreateOrca {
    /// The raise token, that the price is quoted in
    pub bid_denom: Denom,

    /// The threshold under which bids are automatically activated when placed
    pub bid_threshold: Uint128,

    /// The total number of pools in this queue
    pub max_slot: u8,

    /// The incremental discount offered per-pool
    pub premium_rate_per_slot: Decimal,

    /// The amount of time in seconds that a bid must wait until it can be activated
    pub waiting_period: u64,
}

#[cw_serde]
pub enum Status {
    /// The sale is live and can be executed
    Live { closes_at: Timestamp },
    /// The sale has been retracted
    Retracted(Timestamp),
    /// The sale has been executed
    Executed {
        at: Timestamp,
        raise_total: Uint128,
        raise_fee: Uint128,
        raise_amount: Uint128,
    },
}

#[cw_serde]
pub enum CallbackType {
    LiquidationCallback { idx: Uint128 },
}

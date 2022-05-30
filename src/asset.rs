use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Uint128;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Asset {
    pub info: AssetInfo,
    pub amount: Uint128,
}

/// AssetInfo contract_addr is usually passed from the cw20 hook
/// so we can trust the contract_addr is properly validated.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AssetInfo {
    Token { contract_addr: String },
    NativeToken { denom: String },
}

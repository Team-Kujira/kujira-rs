use cosmwasm_schema::{
    cw_serde,
    serde::{de::DeserializeOwned, Serialize},
};
use cosmwasm_std::{from_binary, to_binary, Addr, Binary, Coin, CosmosMsg, StdResult, WasmMsg};

#[cw_serde]
pub struct CallbackMsg {
    pub data: Binary,
    pub callback: CallbackData,
}
#[cw_serde]
pub struct CallbackData(pub Binary);

#[cw_serde]
/// Serialization Helper for Callbacks
enum ReceiverExecuteMsg {
    Callback(CallbackMsg),
}

impl CallbackData {
    pub fn to_message<T>(
        &self,
        cb_addr: &Addr,
        data: impl Serialize,
        funds: impl Into<Vec<Coin>>,
    ) -> StdResult<CosmosMsg<T>> {
        let msg = CallbackMsg {
            data: to_binary(&data)?,
            callback: self.clone(),
        };
        Ok(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: cb_addr.to_string(),
            msg: to_binary(&ReceiverExecuteMsg::Callback(msg))?,
            funds: funds.into(),
        }))
    }

    pub fn into_binary(self) -> Binary {
        self.0
    }
}

impl CallbackMsg {
    pub fn deserialize<D: DeserializeOwned, CB: DeserializeOwned>(self) -> StdResult<(D, CB)> {
        let data = from_binary(&self.data)?;
        let callback = from_binary(&self.callback.into_binary())?;
        Ok((data, callback))
    }

    pub fn deserialize_data<D: DeserializeOwned>(&self) -> StdResult<D> {
        let data = from_binary(&self.data)?;
        Ok(data)
    }

    pub fn deserialize_callback<CB: DeserializeOwned>(&self) -> StdResult<CB> {
        let callback = from_binary(&self.callback.clone().into_binary())?;
        Ok(callback)
    }
}

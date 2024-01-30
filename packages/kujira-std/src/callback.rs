use cosmwasm_schema::{
    cw_serde,
    serde::{de::DeserializeOwned, Serialize},
};
use cosmwasm_std::{
    from_json, to_json_binary, Addr, Binary, Coin, CosmosMsg, Empty, StdResult, WasmMsg,
};

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
            data: to_json_binary(&data)?,
            callback: self.clone(),
        };
        Ok(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: cb_addr.to_string(),
            msg: to_json_binary(&ReceiverExecuteMsg::Callback(msg))?,
            funds: funds.into(),
        }))
    }

    pub fn into_json_binary(self) -> Binary {
        self.0
    }
}

impl CallbackMsg {
    pub fn new<D: Serialize>(data: D, callback: CallbackData) -> StdResult<Self> {
        let data = to_json_binary(&data)?;
        Ok(Self { data, callback })
    }

    pub fn new_without_data(callback: CallbackData) -> Self {
        Self {
            data: to_json_binary(&Empty {}).unwrap(),
            callback,
        }
    }

    pub fn deserialize<D: DeserializeOwned, CB: DeserializeOwned>(self) -> StdResult<(D, CB)> {
        let data = from_json(&self.data)?;
        let callback = from_json(self.callback.into_json_binary())?;
        Ok((data, callback))
    }

    pub fn deserialize_data<D: DeserializeOwned>(&self) -> StdResult<D> {
        let data = from_json(&self.data)?;
        Ok(data)
    }

    pub fn deserialize_callback<CB: DeserializeOwned>(&self) -> StdResult<CB> {
        let callback = from_json(self.callback.clone().into_json_binary())?;
        Ok(callback)
    }
}

impl From<Binary> for CallbackData {
    fn from(b: Binary) -> Self {
        Self(b)
    }
}

use cosmwasm_schema::{
    cw_serde,
    serde::{de::DeserializeOwned, Serialize},
};
use cosmwasm_std::{
    from_binary, to_binary, Addr, Binary, Coin, CosmosMsg, Empty, StdError, StdResult, Storage,
    Uint128, WasmMsg,
};
use cw_storage_plus::Item;

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
    pub fn new<D: Serialize>(data: D, callback: CallbackData) -> StdResult<Self> {
        let data = to_binary(&data)?;
        Ok(Self { data, callback })
    }

    pub fn new_without_data(callback: CallbackData) -> Self {
        Self {
            data: to_binary(&Empty {}).unwrap(),
            callback,
        }
    }

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

impl From<Binary> for CallbackData {
    fn from(b: Binary) -> Self {
        Self(b)
    }
}

pub fn add_expecting_callback(storage: &mut dyn Storage) -> StdResult<Uint128> {
    let semaphore: Item<Uint128> = Item::new("_expecting_callback");
    let value = semaphore.may_load(storage).map(|v| v.unwrap_or_default())? + Uint128::one();
    semaphore.save(storage, &value)?;
    Ok(value)
}

pub fn received_expecting_callback(storage: &mut dyn Storage) -> StdResult<Uint128> {
    let semaphore: Item<Uint128> = Item::new("_expecting_callback");
    let mut value = semaphore.may_load(storage).map(|v| v.unwrap_or_default())?;
    if value > Uint128::zero() {
        value -= Uint128::one();
        semaphore.save(storage, &value)?;
    } else {
        return Err(StdError::generic_err(
            "Received callback when not expecting one",
        ));
    }
    Ok(value)
}

pub fn is_expecting_callback(storage: &dyn Storage) -> StdResult<bool> {
    let semaphore: Item<Uint128> = Item::new("_expecting_callback");
    let value = semaphore.may_load(storage).map(|v| v.unwrap_or_default())?;
    Ok(!value.is_zero())
}

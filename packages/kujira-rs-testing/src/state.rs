use std::{fs::File, io::BufReader};

use cosmwasm_std::{
    testing::{MockApi, MockQuerier, MockStorage},
    CustomQuery, OwnedDeps, Storage,
};
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
struct StateExport {
    models: Vec<StateModel>,
    pagination: StatePagination,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]

struct StateModel {
    key: String,
    value: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
struct StatePagination {
    next_key: Option<Vec<u8>>,
    total: String,
}

pub fn mock_state<C: CustomQuery + DeserializeOwned>(
    deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier<C>, C>,
    file: &str,
) {
    let file = File::open(format!("./src/testing/states/{file}.json")).unwrap();
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u: StateExport = serde_json::from_reader(reader).unwrap();

    for m in u.models {
        let k = hex::decode(&m.key).unwrap();
        let v = base64::decode(&m.value).unwrap();

        deps.storage.set(&k, &v)
    }
}

use godot::prelude::*;

use tezos_contract::ContractFetcher;
use tezos_contract::{contract::MappedEntrypoints, Contract, Error, Storage};
use tezos_core::types::encoded::ContractHash;
use tezos_michelson::micheline::{sequence::Sequence, Micheline};
use tezos_michelson::michelson::types::Parameter;
use tezos_operation::operations::Parameters;
use tezos_rpc::models::contract::ContractScript;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};

#[derive(GodotClass, Debug, Serialize, Deserialize)]
#[class(base=RefCounted)]
struct TezosScript {
    code: String,
    storage: String,
}

fn make(address: ContractHash, script: String) -> Contract {
    let script: ContractScript = from_str(&script).expect("Expected valid script JSON");
    let storage = Storage::new(script).expect("Expected this to work");
    let parameter: Parameter = script
        .code
        .values()
        .iter()
        .nth(0)
        .ok_or(Error::InvalidContractScript)
        .expect("to work")
        .clone()
        .try_into()
        .expect("to work");
    let entrypoints = MappedEntrypoints::new(parameter);

    Contract {
        address: address,
        storage: storage,
        entrypoints: entrypoints,
    }
}

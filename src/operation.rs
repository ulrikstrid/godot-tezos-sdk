use godot::prelude::*;
use tezos_core::types::{
    encoded::{Address, ImplicitAddress, SecretKey},
    mutez::Mutez,
    number::Nat,
};
use tezos_michelson::micheline::Micheline;
use tezos_operation::operations::{Entrypoint, Parameters, Transaction, UnsignedOperation};
use tezos_rpc::models::operation::operation_contents_and_result::transaction::TransactionParameters;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};

#[derive(GodotClass, FromGodot, ToGodot, GodotConvert, Debug, Serialize, Deserialize)]
#[class(base=RefCounted)]
pub struct TezosParametersJson {
    entrypoint: String,
    value: String,
}

fn param_to_json(params: Parameters) -> TezosParametersJson {
    TezosParametersJson {
        entrypoint: params.entrypoint.to_str().to_string(),
        value: json!(params.value).to_string(),
    }
}

fn json_to_param(params: &TezosParametersJson) -> Parameters {
    let m: Micheline = from_str(&params.value).expect("Expected valid micheline");
    Parameters {
        entrypoint: Entrypoint::from_str(&params.entrypoint),
        value: m,
    }
}

#[godot_api]
impl TezosParametersJson {
    #[func]
    fn to_dict(&self) -> Dictionary {
        dict! {
            "entrypoint": self.entrypoint.to_string(),
            "value": self.value.to_string(),
        }
    }

    #[func]
    fn to_string(&self) -> String {
        let params = json_to_param(self);
        json!(TransactionParameters::from(params)).to_string()
    }

    #[func]
    fn make(entrypoint: String, value: String) -> Gd<TezosParametersJson> {
        let m: Micheline = from_str(&value).expect("Expected valid micheline");
        Gd::new(TezosParametersJson {
            entrypoint: entrypoint,
            value: json!(m).to_string(),
        })
    }
}

#[godot_api]
impl IRefCounted for TezosParametersJson {
    fn init(_base: Base<RefCounted>) -> Self {
        TezosParametersJson {
            entrypoint: "".to_string(),
            value: "".to_string(),
        }
    }
}

#[derive(GodotClass, FromGodot, ToGodot, GodotConvert, Debug, Serialize, Deserialize)]
#[class(base=RefCounted)]
pub struct TezosTransaction {
    source: String,
    fee: i32,
    counter: i32,
    gas_limit: i32,
    storage_limit: i32,
    amount: i32,
    destination: String,
    parameters: TezosParametersJson,
}

#[godot_api]
impl TezosTransaction {
    #[func]
    pub fn make(
        source: String,
        destination: String,
        counter: i32,
        amount: i32,
        godot_parameters: TezosParametersJson,
        fee: i32,
        gas_limit: i32,
        storage_limit: i32,
    ) -> TezosTransaction {
        let value = godot_parameters.value.to_string();
        let entrypoint = godot_parameters.entrypoint.to_string();
        let value: Micheline = from_str(&value).expect("Expected valid Micheline JSON");
        let params: Option<Parameters> = Some(Parameters {
            entrypoint: tezos_operation::operations::Entrypoint::Named(entrypoint),
            value: value,
        });

        TezosTransaction {
            source,
            destination,
            counter,
            amount,
            parameters: godot_parameters,
            fee,
            gas_limit,
            storage_limit,
        }
    }

    fn to_transaction(&self) -> Transaction {
        let params: Option<Parameters> = Some(Parameters {
            entrypoint: tezos_operation::operations::Entrypoint::Named(
                self.parameters.entrypoint.to_string(),
            ),
            value: from_str(&self.parameters.value).expect("Expected valid Micheline JSON"),
        });

        let fee: u8 = self.fee.try_into().expect("Expected fee to be u8");
        let counter: u8 = self.counter.try_into().expect("Expected counter to be u8");
        let gas_limit: u8 = self
            .gas_limit
            .try_into()
            .expect("Expected gas_limit to be u8");
        let storage_limit: u8 = self
            .gas_limit
            .try_into()
            .expect("Expected storage_limit to be u8");
        let amount: u16 = self
            .amount
            .try_into()
            .expect("Expected storage_limit to be u6");

        Transaction::new(
            self.source
                .to_string()
                .try_into()
                .expect("valid conversion to ImplicitAddress"),
            fee.into(),
            counter.into(),
            gas_limit.into(),
            storage_limit.into(),
            amount.into(),
            self.destination
                .to_string()
                .try_into()
                .expect("valid conversion to ImplicitAddress"),
            params,
        )
    }
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct TezosOperation {}

#[godot_api]
impl TezosOperation {}

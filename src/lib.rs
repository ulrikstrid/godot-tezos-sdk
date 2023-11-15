use godot::prelude::*;

struct TezosSDK;

#[gdextension]
unsafe impl ExtensionLibrary for TezosSDK {}

// mod contract;
mod michelson;
mod operation;

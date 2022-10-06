mod rpc;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Default, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[serde(default)]
pub struct KeyboardShortcuts {
    pub special: String,
    pub key: String,
    pub widget: String,
}

#[derive(Clone, Serialize, Default, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[serde(default)]
pub struct SbbwConfig {
    pub port: u16,
    pub shortcuts: Vec<KeyboardShortcuts>,
}

pub mod prelude {
    pub use super::{rpc::*, *};
}

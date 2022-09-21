mod rpc;

use serde::{Serialize, Deserialize};

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
    pub use super::*;
    pub use super::rpc::*;
}

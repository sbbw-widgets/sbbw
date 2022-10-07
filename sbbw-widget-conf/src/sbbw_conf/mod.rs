mod rpc;

use serde::{Deserialize, Serialize};

use crate::RpcAction;

#[derive(Clone, Serialize, Default, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[serde(default)]
pub struct KeyboardShortcuts {
    pub keys: Vec<String>,
    pub widget: String,
    pub action: RpcAction,
    pub url: Option<String>,
    pub widget_args: String,
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

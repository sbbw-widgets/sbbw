use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Default, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RpcAction {
    Open,
    Close,
    Test,
    #[default]
    Toggle,
}

impl FromStr for RpcAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "open" => Ok(Self::Open),
            "close" => Ok(Self::Close),
            "test" => Ok(Self::Test),
            "toggle" => Ok(Self::Toggle),
            x => Err(format!("\"{x}\" not recognized"))
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub struct RpcDataRequest {
    pub widget_name: String,
    pub action: RpcAction,
    pub url: String,
    pub widget_params: Option<String>,
}

impl RpcDataRequest {
    pub fn get_args(self) -> Vec<String> {
        let mut args = Vec::new();
        if self.action == RpcAction::Test {
            args.push("--test".to_string());
        }
        if let Some(a) = self.widget_params {
            args.push("--args".to_string());
            args.push(a);
        }
        args.push("--widget-name".to_string());
        args.push(self.widget_name);
        args.push(self.url);
        args
    }
}

use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RpcAction {
    Open,
    Close,
    Test,
    Toggle,
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


#![allow(dead_code)]
use std::str::FromStr;

use sbbw_widget_conf::RpcAction;

const ACCEPT: &[&str] = &["Yes", "yes", "Y", "y", "true", "True"];
const DECLINE: &[&str] = &["No", "no", "N", "n", "false", "False", ""];

pub struct MyBool(pub bool);

impl FromStr for MyBool {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if ACCEPT.contains(&s) {
            Ok(Self(true))
        } else if DECLINE.contains(&s) {
            Ok(Self(false))
        } else {
            Err(format!("{s} Not recognized"))
        }
    }
}

pub fn accept(s: &str) -> bool {
    !s.is_empty() || MyBool::from_str(s.trim()).map_or(false, |_| true)
}

pub fn is_widget(s: &str, widgets: &[String]) -> bool {
    !s.is_empty() || widgets.contains(&s.trim().to_string())
}

pub fn is_number(s: &str) -> bool {
    s.trim().parse::<i32>().map_or(false, |_| true)
}

pub fn is_float(s: &str) -> bool {
    s.trim().parse::<f32>().map_or(false, |_| true)
}

pub fn is_rpc_action(s: &str) -> bool {
    !s.is_empty() || RpcAction::from_str(s).map_or(false, |_| true)
}

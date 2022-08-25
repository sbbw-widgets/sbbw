#![allow(dead_code)]

pub mod base;
pub mod bat;
pub mod initial;
pub mod sysinfo;
pub mod widget;

use std::collections::HashMap;

use sbbw_exec::Params;
use serde::{Deserialize, Serialize};
use tao::window::Window;
use wry::http::status::StatusCode;

#[derive(Default, Deserialize, Serialize)]
pub struct SbbwResponse {
    pub status: u16,
    pub data: String,
}

pub type MethodActions = HashMap<&'static str, Box<dyn Fn(&Window, String, &Params) -> SbbwResponse>>;

fn get_actions() -> MethodActions {
    let mut actions = MethodActions::new();

    base::register(&mut actions);
    bat::register(&mut actions);

    actions
}

pub fn parse_params(res: &mut SbbwResponse, msg: String) -> Option<Params> {
    if let Ok(params) = serde_json::from_str(msg.as_str()) {
        res.status = StatusCode::OK.as_u16();
        res.data = "".to_string();
        Some(params)
    } else {
        res.status = StatusCode::BAD_REQUEST.as_u16();
        res.data = "Invalid JSON sended".to_string();
        None
    }
}

pub fn process_ipc(win: &Window, widget_name: String, params: Params) -> SbbwResponse {
    let methods = get_actions();
    let mut res = SbbwResponse::default();

    if let Some(f) = methods.get(&params.method.as_str()) {
        res = f(win, widget_name, &params);
    } else {
        res.status = StatusCode::NOT_FOUND.as_u16();
        res.data = "Invalid command".to_string();
    }

    res
}
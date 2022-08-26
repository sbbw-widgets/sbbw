#![allow(dead_code)]

pub mod base;
pub mod bat;
pub mod initial;
pub mod sys_info;
pub mod widget;

use std::collections::HashMap;

use log::error;
use sbbw_exec::Params;
use serde::{Deserialize, Serialize};
use tao::window::Window;
use wry::http::status::StatusCode;

#[derive(Default, Deserialize, Serialize)]
pub struct SbbwResponse {
    pub status: u16,
    pub data: String,
}

pub type MethodActions =
    HashMap<&'static str, Box<dyn Fn(&Window, String, &Params) -> SbbwResponse>>;

fn get_actions() -> MethodActions {
    let mut actions = MethodActions::new();

    base::register(&mut actions);
    bat::register(&mut actions);
    sys_info::register(&mut actions);
    widget::register(&mut actions);

    actions
}

pub fn parse_params(res: &mut SbbwResponse, msg: String) -> Option<Params> {
    match serde_json::from_str::<Params>(msg.as_str()) {
        Ok(params) => {
            res.status = StatusCode::OK.as_u16();
            res.data = "".to_string();
            Some(params)
        }
        Err(e) => {
            error!("Parse params error: {e}");
            res.status = StatusCode::BAD_REQUEST.as_u16();
            res.data = "Invalid JSON sended".to_string();
            None
        }
    }
}

pub fn process_ipc(win: &Window, widget_name: String, params: &Params) -> SbbwResponse {
    let methods = get_actions();
    let mut res = SbbwResponse::default();

    if let Some(f) = methods.get(params.method.as_str()) {
        res = f(win, widget_name, params);
    } else {
        res.status = StatusCode::NOT_FOUND.as_u16();
        res.data = "Invalid command".to_string();
    }

    res
}

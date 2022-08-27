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

pub type ActionsFn = fn(&Window, String, &Params) -> SbbwResponse;

static ACTIONS: &[(&str, ActionsFn)] = &[
    /*
     * Base
     */
    ("exec", base::exec),
    /*
     * Battery
     */
    ("battery.counts", bat::batery_counts),
    ("battery.all", bat::bateries),
    ("battery.main", bat::main_batery),
    /*
     * System Information
     */
    ("sys.disks", sys_info::disks),
    ("sys.net", sys_info::network),
    ("sys.info", sys_info::info),
    ("sys.memory", sys_info::memory),
    ("sys.cpu", sys_info::cpu),
    /*
     * Widget
     */
    ("widget.info", widget::info),
    ("widget.move", widget::move_window),
    ("widget.resize", widget::resize_window),
];

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
    let mut res = SbbwResponse::default();

    if let Some((_name, f)) = ACTIONS
        .iter()
        .find(|(name, _)| &params.method.as_str() == name)
    {
        res = f(win, widget_name, params);
    } else {
        res.status = StatusCode::NOT_FOUND.as_u16();
        res.data = "Invalid command".to_string();
    }

    res
}

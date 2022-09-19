#![allow(dead_code)]

pub mod initial;
use crate::builtin::{
    base, bat, brightness_ctl::prelude as bright, media_ctl::prelude as media, sys_info, widget,
    wifi::prelude as wifi,
};

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

pub type ActionsFn = fn(&Window, String, &str) -> SbbwResponse;

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
     * Brightness
     */
    #[cfg(not(target_os = "macos"))]
    ("brightness.main", bright::get_main_brightness),
    #[cfg(not(target_os = "macos"))]
    ("brightness.all", bright::get_all_brightness),
    #[cfg(not(target_os = "macos"))]
    ("brightness.set_main", bright::set_main_brightness),
    #[cfg(not(target_os = "macos"))]
    ("brightness.set_all", bright::set_all_brightness),
    /*
     * Media Control
     */
    #[cfg(target_os = "linux")]
    ("media.play_pause", media::set_play_pause),
    #[cfg(target_os = "linux")]
    ("media.state", media::get_state),
    #[cfg(target_os = "linux")]
    ("media.next", media::set_next),
    #[cfg(target_os = "linux")]
    ("media.prev", media::set_prev),
    #[cfg(target_os = "linux")]
    ("media.set_volume", media::set_volume),
    #[cfg(target_os = "linux")]
    ("media.get_volume", media::get_volume),
    #[cfg(target_os = "linux")]
    ("media.active", media::is_player_active),
    /*
     * System Information
     */
    ("sys.disks", sys_info::disks),
    ("sys.net", sys_info::network),
    ("sys.info", sys_info::info),
    ("sys.memory", sys_info::memory),
    ("sys.cpu", sys_info::cpu),
    /*
     * Get Wifi Information
     */
    #[cfg(target_os = "linux")]
    ("sys.wifi", wifi::get_wifi_info),
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
        res = f(win, widget_name, params.data.as_str());
    } else {
        res.status = StatusCode::NOT_FOUND.as_u16();
        res.data = "Invalid command".to_string();
    }

    res
}

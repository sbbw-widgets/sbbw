use iwlib::get_wireless_info;
use log::trace;
use mpris::{Player, PlayerFinder};
use sbbw_exec::Params;
use serde::{Deserialize, Serialize};
use tao::window::Window;
use wry::http::status::StatusCode;

use crate::ipc::SbbwResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WirelessInfo {
    pub ssid: String,
    pub quality: u8,
}

pub fn get_wifi_info(_win: &Window, _name: String, ssid: &str) -> SbbwResponse {
    trace!("Requesting wifi information");

    let info = get_wireless_info(ssid.to_string()).unwrap();
    let info = WirelessInfo {
        ssid: info.wi_essid.clone(),
        quality: info.wi_quality,
    };

    SbbwResponse {
        status: StatusCode::OK.as_u16(),
        data: serde_json::to_string(&info).unwrap(),
    }
}

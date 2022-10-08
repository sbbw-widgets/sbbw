use brightness::blocking::{Brightness, BrightnessDevice};
use log::{error, trace};
use sbbw_exec::Params;
use serde::{Deserialize, Serialize};
use tao::window::Window;
use wry::http::status::StatusCode;

use crate::ipc::SbbwResponse;

use super::SbbwBrightnessDevice;

pub fn get_main_brightness(_win: &Window, _name: String, _params: &str) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    trace!("Request brightness_ctl of main");

    match get_all_devices() {
        Ok(devices) => {
            if let Some(device) = devices.first() {
                res.status = StatusCode::OK.as_u16();
                res.data = serde_json::to_string(&device).unwrap();
            } else {
                res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
                res.data = "Cannot get brightness devices".to_string();
            }
        }
        Err(e) => {
            res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
            res.data = e;
        }
    }

    res
}

pub fn get_all_brightness(_win: &Window, _name: String, _params: &str) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    trace!("Request brightness_ctl of all");

    match get_all_devices() {
        Ok(devices) => {
            res.status = StatusCode::OK.as_u16();
            res.data = serde_json::to_string(&devices).unwrap();
        }
        Err(e) => {
            res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
            res.data = e;
        }
    }

    res
}

pub fn set_main_brightness(_win: &Window, _name: String, params: &str) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    trace!("Request change brightness_ctl of main");

    match serde_json::from_str::<u32>(params) {
        Ok(value) => {
            let mut devices = brightness::blocking::brightness_devices();
            if let Some(device) = devices.next() {
                if device.unwrap().set(value).is_ok() {
                    res.status = StatusCode::OK.as_u16();
                    res.data = "".to_string();
                } else {
                    res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
                    res.data = "Cannot set brightness value".to_string();
                }
            } else {
                res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
                res.data = "Cannot set brightness value".to_string();
            }
        }
        Err(_) => {
            error!("Bad params");
            res.status = StatusCode::BAD_REQUEST.as_u16();
            res.data = "This require percent of brightness to set".to_string();
        }
    }

    res
}

pub fn set_all_brightness(_win: &Window, _name: String, params: &str) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    trace!("Request change brightness_ctl of all");

    match serde_json::from_str::<u32>(params) {
        Ok(value) => {
            if brightness::blocking::brightness_devices()
                .try_for_each(|device| device.unwrap().set(value))
                .is_ok()
            {
                res.status = StatusCode::OK.as_u16();
                res.data = "".to_string();
            } else {
                res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
                res.data = "Cannot set brightness value".to_string();
            }
        }
        Err(_) => {
            error!("Bad params");
            res.status = StatusCode::BAD_REQUEST.as_u16();
            res.data = "This require percent of brightness to set".to_string();
        }
    }

    res
}

fn get_all_devices() -> Result<Vec<SbbwBrightnessDevice>, String> {
    let m = brightness::blocking::brightness_devices()
        .map(|dev| {
            if let Ok(dev) = dev {
                Some(SbbwBrightnessDevice {
                    name: dev.device_name().unwrap_or_else(|_| "".to_string()),
                    value: dev.get().unwrap_or_default(),
                })
            } else {
                None
            }
        })
        .flatten();
    Ok(m.collect::<Vec<SbbwBrightnessDevice>>())
}

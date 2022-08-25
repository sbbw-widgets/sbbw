use std::sync::Mutex;

use battery::Manager;
use sbbw_exec::Params;
use serde::{Deserialize, Serialize};
use tao::window::Window;
use wry::http::status::StatusCode;

use super::{MethodActions, SbbwResponse};

#[derive(Serialize, Deserialize, Clone)]
struct SbbwBattery {
    pub vendor: String,
    pub model: String,
    pub serial: String,
    pub percentage: f32,
    pub energy: f32,
    pub energy_full: f32,
    pub voltage: f32,
    pub state: String,
    pub health: f32,
    pub technology: String,
    pub temperature: f32,
    pub cycle_count: u32,
    pub time_to_full: f32,
    pub time_to_empty: f32,
}

pub fn register(action: &mut MethodActions) {
    action.insert("battery.counts", Box::new(batery_counts));
    action.insert("battery.all", Box::new(bateries));
    action.insert("battery.main", Box::new(main_batery));
}

fn batery_counts(_win: &Window, name: String, params: &Params) -> SbbwResponse {
    let mut res = SbbwResponse::default();

    match Manager::new() {
        Ok(manager) => match manager.batteries() {
            Ok(bats) => {
                res.status = StatusCode::OK.as_u16();
                res.data = bats.count().to_string();
            }
            Err(e) => {
                res.status = StatusCode::NO_CONTENT.as_u16();
                res.data = e.to_string();
            }
        },
        Err(e) => {
            res.status = StatusCode::NOT_FOUND.as_u16();
            res.data = e.to_string();
        }
    }

    res
}

fn bateries(_win: &Window, name: String, params: &Params) -> SbbwResponse {
    let mut res = SbbwResponse::default();

    match Manager::new() {
        Ok(manager) => match manager.batteries() {
            Ok(bats) => {
                let bats = bats
                    .map(|b| {
                        let b = b.unwrap();
                        SbbwBattery {
                            vendor: b.vendor().unwrap_or("").to_string(),
                            model: b.model().unwrap_or("").to_string(),
                            serial: b.serial_number().unwrap_or("").to_string(),
                            percentage: b.energy().value,
                            energy: b.energy().value,
                            energy_full: b.energy_full().value,
                            voltage: b.voltage().value,
                            state: b.state().to_string(),
                            health: b.state_of_health().value,
                            technology: b.technology().to_string(),
                            temperature: b.temperature().unwrap_or_default().value,
                            cycle_count: b.cycle_count().unwrap_or_default(),
                            time_to_full: b.time_to_full().unwrap_or_default().value,
                            time_to_empty: b.time_to_empty().unwrap_or_default().value,
                        }
                    })
                    .collect::<Vec<SbbwBattery>>();
                res.status = StatusCode::OK.as_u16();
                res.data = serde_json::to_string(&bats).unwrap();
            }
            Err(e) => {
                res.status = StatusCode::NO_CONTENT.as_u16();
                res.data = e.to_string();
            }
        },
        Err(e) => {
            res.status = StatusCode::NOT_FOUND.as_u16();
            res.data = e.to_string();
        }
    }

    res
}

fn main_batery(_win: &Window, name: String, params: &Params) -> SbbwResponse {
    let mut res = SbbwResponse::default();

    match Manager::new() {
        Ok(manager) => match manager.batteries() {
            Ok(bats) => {
                let b = bats.next().unwrap().unwrap();
                let bat = SbbwBattery {
                    vendor: b.vendor().unwrap_or("").to_string(),
                    model: b.model().unwrap_or("").to_string(),
                    serial: b.serial_number().unwrap_or("").to_string(),
                    percentage: b.energy().value,
                    energy: b.energy().value,
                    energy_full: b.energy_full().value,
                    voltage: b.voltage().value,
                    state: b.state().to_string(),
                    health: b.state_of_health().value,
                    technology: b.technology().to_string(),
                    temperature: b.temperature().unwrap_or_default().value,
                    cycle_count: b.cycle_count().unwrap_or_default(),
                    time_to_full: b.time_to_full().unwrap_or_default().value,
                    time_to_empty: b.time_to_empty().unwrap_or_default().value,
                };
                res.status = StatusCode::OK.as_u16();
                res.data = serde_json::to_string(&bat).unwrap();
            }
            Err(e) => {
                res.status = StatusCode::NO_CONTENT.as_u16();
                res.data = e.to_string();
            }
        },
        Err(e) => {
            res.status = StatusCode::NOT_FOUND.as_u16();
            res.data = e.to_string();
        }
    }

    res
}
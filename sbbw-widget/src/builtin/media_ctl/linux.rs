use log::trace;
use mpris::{Player, PlayerFinder};
use sbbw_exec::Params;
use serde::{Deserialize, Serialize};
use tao::window::Window;
use wry::http::status::StatusCode;

use crate::{
    builtin::media_ctl::{SbbwMediaMetadata, SbbwMediaState},
    ipc::SbbwResponse,
};

pub fn is_player_active(_win: &Window, _name: String, _params: &str) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    trace!("Requesting to check if player is currently active");

    if let Some(player) = get_player(&mut res) {
        let value = player.is_running();
        res.status = StatusCode::OK.as_u16();
        res.data = serde_json::to_string(&value).unwrap_or_else(|_| "false".to_string());
    }

    res
}

pub fn set_volume(_win: &Window, _name: String, params: &str) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    trace!("Requesting to change volume");

    if let Some(volume) = get_from_str::<f64>(&mut res, params) {
        if let Some(player) = get_player(&mut res) {
            if player.set_volume_checked(volume).unwrap_or(false) {
                res.status = StatusCode::OK.as_u16();
                res.data = "".to_string();
            } else {
                res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
                res.data = "Cannot change volume".to_string();
            }
        }
    }

    res
}

pub fn set_play_pause(_win: &Window, _name: String, params: &str) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    trace!("Requesting to play/pause media");

    if let Some(play) = get_from_str::<bool>(&mut res, params) {
        if let Some(player) = get_player(&mut res) {
            if play {
                if player.can_play().unwrap_or(false) {
                    if player.checked_play().is_ok_and(|v| v) {
                        res.status = StatusCode::OK.as_u16();
                        res.data = "".to_string();
                    } else {
                        res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
                        res.data = "Error to send play event".to_string();
                    }
                } else {
                    res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
                    res.data = "Cannot send play event".to_string();
                }
            } else if player.can_pause().unwrap_or(false) {
                if player.checked_pause().is_ok_and(|v| v) {
                    res.status = StatusCode::OK.as_u16();
                    res.data = "".to_string();
                } else {
                    res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
                    res.data = "Error to send pause event".to_string();
                }
            } else {
                res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
                res.data = "Cannot send pause event".to_string();
            }
        }
    }

    res
}

pub fn set_next(_win: &Window, _name: String, _params: &str) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    trace!("Requesting next media");

    if let Some(player) = get_player(&mut res) {
        if player.can_go_next().unwrap_or(false) {
            if player.checked_next().is_ok_and(|v| v) {
                let state = internal_get_state(&player);
                if let Ok(json) = serde_json::to_string(&state) {
                    res.status = StatusCode::OK.as_u16();
                    res.data = json;
                } else {
                    res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
                    res.data = format!("Cannot serialize state: {:?}", state);
                }
            } else {
                res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
                res.data = "Error to get next track".to_string();
            }
        } else {
            res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
            res.data = "Cannot change volume".to_string();
        }
    }

    res
}

pub fn set_prev(_win: &Window, _name: String, _params: &str) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    trace!("Requesting previous media");

    if let Some(player) = get_player(&mut res) {
        if player.can_go_previous().unwrap_or(false) {
            if player.checked_previous().is_ok_and(|v| v) {
                let state = internal_get_state(&player);
                if let Ok(json) = serde_json::to_string(&state) {
                    res.status = StatusCode::OK.as_u16();
                    res.data = json;
                } else {
                    res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
                    res.data = format!("Cannot serialize state: {:?}", state);
                }
            } else {
                res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
                res.data = "Error to get previous media".to_string();
            }
        } else {
            res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
            res.data = "Cannot change volume".to_string();
        }
    }

    res
}

pub fn get_volume(_win: &Window, _name: String, _params: &str) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    trace!("Requesting to change player's volume");

    if let Some(player) = get_player(&mut res) {
        if let Ok(value) = player.get_volume() {
            res.status = StatusCode::OK.as_u16();
            res.data = serde_json::to_string(&value).unwrap_or_else(|_| "0.0".to_string());
        } else {
            res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
            res.data = "Cannot set player's volume".to_string();
        }
    }

    res
}

fn get_from_str<'a, T>(res: &mut SbbwResponse, params: &'a str) -> Option<T>
where
    T: Deserialize<'a> + Serialize,
{
    if let Ok(value) = serde_json::from_str::<T>(params) {
        Some(value)
    } else {
        res.status = StatusCode::BAD_REQUEST.as_u16();
        res.data = "Cannot get parameter".to_string();
        None
    }
}

pub fn get_state(_win: &Window, _name: String, _params: &str) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    trace!("Requesting to get current media player state");

    if let Some(player) = get_player(&mut res) {
        let state = internal_get_state(&player);
        if let Ok(json) = serde_json::to_string(&state) {
            res.status = StatusCode::OK.as_u16();
            res.data = json;
        } else {
            res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16();
            res.data = format!("Cannot serialize state: {:?}", state);
        }
    }

    res
}

//
// Internals to make less code
//

fn get_player(res: &mut SbbwResponse) -> Option<Player> {
    match PlayerFinder::new() {
        Ok(manager) => match manager.find_active() {
            Ok(player) => Some(player),
            Err(_) => {
                res.status = StatusCode::NOT_FOUND.as_u16();
                res.data = "Player not found".to_string();
                None
            }
        },
        Err(_) => {
            res.status = StatusCode::NOT_FOUND.as_u16();
            res.data = "Player not found".to_string();
            None
        }
    }
}

fn internal_get_state(player: &Player) -> SbbwMediaState {
    let metadata = player.get_metadata().unwrap_or_default();
    SbbwMediaState {
        id: player.unique_name().to_string(),
        player_name: player.identity().to_string(),
        volume: player.get_volume().ok(),
        metadata: if player.is_running() {
            Some(SbbwMediaMetadata {
                track_id: metadata.track_id().to_string(),
                title: metadata.title().unwrap_or("").to_string(),
                album_name: metadata.album_name().unwrap_or("").to_string(),
                album_artists: metadata.album_artists().unwrap_or(&vec![]).to_vec(),
                artists: metadata.artists().unwrap_or(&vec![]).to_vec(),
                art_url: metadata.art_url().map(|u| u.to_string()),
                track_length: metadata.length_in_microseconds(),
            })
        } else {
            None
        },
        track_progress: player.get_position_in_microseconds().ok(),
        shuffle: player.get_shuffle().unwrap_or(false),
    }
}

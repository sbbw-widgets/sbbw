use clap::Parser;
use log::{error, info, trace};
use sbbw_exec::Params;
use serde::{Deserialize, Serialize};
use tao::{
    dpi::{LogicalPosition, LogicalSize, Position, Size},
    window::Window,
};
use wry::http::status::StatusCode;

use crate::cmd::Args;

use crate::SbbwResponse;

#[derive(Deserialize)]
struct SbbwWidgetVectorParam {
    x: f64,
    y: f64,
}

#[derive(Serialize, Clone)]
struct SbbwWidgetInfo {
    pub name: String,
    pub widget_args: String,
}

pub fn info(_win: &Window, name: String, _params: &str) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    let args = Args::parse();
    trace!("Request Widget data");

    let info = SbbwWidgetInfo {
        name,
        widget_args: args.args.unwrap_or_default(),
    };

    res.status = StatusCode::OK.as_u16();
    res.data = serde_json::to_string(&info).unwrap_or_default();

    res
}

pub fn move_window(win: &Window, _name: String, params: &str) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    trace!("Request Widget move position: {:?}", params);

    match serde_json::from_str::<SbbwWidgetVectorParam>(params) {
        Ok(value) => {
            let new_pos = Position::Logical(LogicalPosition::new(value.x, value.y));
            info!("Position data created: {:?}", &new_pos);

            win.set_outer_position(new_pos);

            res.status = StatusCode::OK.as_u16();
            res.data = "".to_string();
        }
        Err(_) => {
            error!("Bad params");
            res.status = StatusCode::BAD_REQUEST.as_u16();
            res.data = "This require X and Y as param".to_string();
        }
    }

    res
}

pub fn resize_window(win: &Window, _name: String, params: &str) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    trace!("Request Widget Resize: {:?}", params);

    match serde_json::from_str::<SbbwWidgetVectorParam>(params) {
        Ok(value) => {
            let new_size = Size::Logical(LogicalSize::new(value.x, value.y));
            info!("Size data created: {:?}", &new_size);

            win.set_resizable(true);
            win.set_inner_size(new_size);

            res.status = StatusCode::OK.as_u16();
            res.data = "".to_string();
        }
        Err(_) => {
            error!("Bad params");
            res.status = StatusCode::BAD_REQUEST.as_u16();
            res.data = "This require X and Y as param".to_string();
        }
    }

    res
}

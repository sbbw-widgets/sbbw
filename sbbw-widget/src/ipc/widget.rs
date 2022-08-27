use log::{error, info, trace};
use sbbw_exec::Params;
use serde::Serialize;
use tao::{
    dpi::{LogicalPosition, LogicalSize, Position, Size},
    window::Window,
};
use wry::http::status::StatusCode;

use super::SbbwResponse;

#[derive(Serialize, Clone)]
struct SbbwWidgetInfo {
    pub name: String,
    pub widget_args: Vec<String>,
}

pub fn info(_win: &Window, name: String, params: &Params) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    trace!("Request Widget data");

    let info = SbbwWidgetInfo {
        name,
        widget_args: params.args.to_vec(),
    };

    res.status = StatusCode::OK.as_u16();
    res.data = serde_json::to_string(&info).unwrap_or_default();

    res
}

pub fn move_window(win: &Window, _name: String, params: &Params) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    trace!("Request Widget move position: {:?}", params);

    if params.args.len() == 2 {
        let x = params.args.get(0).unwrap();
        let y = params.args.get(1).unwrap();

        let new_pos = Position::Logical(LogicalPosition::new(
            x.parse::<f64>().unwrap_or_default(),
            y.parse::<f64>().unwrap_or_default(),
        ));
        info!("Position data created: {:?}", &new_pos);

        win.set_outer_position(new_pos);

        res.status = StatusCode::OK.as_u16();
        res.data = "".to_string();
    } else {
        error!("Bad params");
        res.status = StatusCode::BAD_REQUEST.as_u16();
        res.data = "This require X and Y as param".to_string();
    }

    res
}

pub fn resize_window(win: &Window, _name: String, params: &Params) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    trace!("Request Widget Resize: {:?}", params);

    if params.args.len() == 2 {
        let width = params.args.get(0).unwrap();
        let height = params.args.get(1).unwrap();

        let new_size = Size::Logical(LogicalSize::new(
            width.parse::<f64>().unwrap_or_default(),
            height.parse::<f64>().unwrap_or_default(),
        ));
        info!("Size data created: {:?}", &new_size);

        win.set_resizable(true);
        win.set_inner_size(new_size);

        res.status = StatusCode::OK.as_u16();
        res.data = "".to_string();
    } else {
        error!("Bad params");
        res.status = StatusCode::BAD_REQUEST.as_u16();
        res.data = "This require X and Y as param".to_string();
    }

    res
}

use sbbw_exec::Params;
use serde::Serialize;
use tao::{
    dpi::{LogicalPosition, LogicalSize, Position, Size},
    window::Window,
};
use wry::http::status::StatusCode;

use super::{MethodActions, SbbwResponse};

pub fn register(action: &mut MethodActions) {
    action.insert("widget.info", Box::new(info));
    action.insert("widget.move", Box::new(move_window));
    action.insert("widget.resize", Box::new(resize_window));
}

#[derive(Serialize, Clone)]
struct SbbwWidgetInfo {
    pub name: String,
    pub widget_args: Vec<String>,
}

fn info(_win: &Window, name: String, params: &Params) -> SbbwResponse {
    let mut res = SbbwResponse::default();

    let info = SbbwWidgetInfo {
        name,
        widget_args: params.args.to_vec(),
    };

    res.status = StatusCode::OK.as_u16();
    res.data = serde_json::to_string(&info).unwrap_or_default();

    res
}

fn move_window(win: &Window, _name: String, params: &Params) -> SbbwResponse {
    let mut res = SbbwResponse::default();

    if params.args.len() == 2 {
        let x = params.args.get(0).unwrap();
        let y = params.args.get(1).unwrap();

        let new_pos = Position::Logical(LogicalPosition::new(
            x.parse::<f64>().unwrap_or_default(),
            y.parse::<f64>().unwrap_or_default(),
        ));

        win.set_outer_position(new_pos);

        res.status = StatusCode::OK.as_u16();
        res.data = "".to_string();
    } else {
        res.status = StatusCode::BAD_REQUEST.as_u16();
        res.data = "This require X and Y as param".to_string();
    }

    res
}

fn resize_window(win: &Window, _name: String, params: &Params) -> SbbwResponse {
    let mut res = SbbwResponse::default();

    if params.args.len() == 2 {
        let width = params.args.get(0).unwrap();
        let height = params.args.get(1).unwrap();

        let new_size = Size::Logical(LogicalSize::new(
            width.parse::<f64>().unwrap_or_default(),
            height.parse::<f64>().unwrap_or_default(),
        ));

        win.set_resizable(true);
        win.set_inner_size(new_size);

        res.status = StatusCode::OK.as_u16();
        res.data = "".to_string();
    } else {
        res.status = StatusCode::BAD_REQUEST.as_u16();
        res.data = "This require X and Y as param".to_string();
    }

    res
}

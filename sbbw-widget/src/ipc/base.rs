use log::{error, info, trace};
use sbbw_exec::{exec_command, Params};
use sbbw_widget_conf::get_widgets_path;
use tao::window::Window;
use wry::http::status::StatusCode;

use super::SbbwResponse;

pub fn exec(_win: &Window, name: String, params: &Params) -> SbbwResponse {
    let mut res = SbbwResponse::default();
    let path_scripts = get_widgets_path().join(&name).join("scripts");
    trace!("Execute \"exec\" internal");

    match exec_command(String::from(path_scripts.to_str().unwrap()), params.clone()) {
        Ok(data) => {
            info!("Output of execution: {data}");
            res.status = StatusCode::OK.as_u16();
            res.data = data;
        }
        Err(e) => {
            error!("Error on execution: {e}");
            res.status = StatusCode::EXPECTATION_FAILED.as_u16();
            res.data = "Failed to excecute command".to_string();
        }
    }

    res
}

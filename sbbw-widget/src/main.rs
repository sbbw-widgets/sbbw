#![feature(proc_macro_hygiene, decl_macro, is_some_and)]
#![allow(unused_imports)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod builtin;
mod cmd;
mod exts;
mod ipc;

use clap::Parser;
use cmd::Args;
use exts::*;
use log::{info, trace};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tao::window::WindowId;
use url::Url;

use std::{
    cell::RefCell,
    collections::HashMap,
    env,
    fs::{self, File},
    path::{Path, PathBuf},
    rc::Rc,
    sync::{Arc, Mutex},
};

use colored::*;
use sbbw_exec::{exec_command, Params};
use sbbw_widget_conf::{get_widgets, get_widgets_path};

use wry::{
    application::{
        dpi::{LogicalPosition, Position},
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Window, WindowBuilder},
    },
    http::{
        header::{CONTENT_TYPE, ORIGIN},
        status::StatusCode,
        Request, Response, ResponseBuilder,
    },
    webview::{WebView, WebViewBuilder},
    Value,
};

use crate::ipc::{initial::get_initial_js, parse_params, process_ipc, SbbwResponse};

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("trace"));
    let args = Args::parse();

    let widgets = get_widgets();
    if widgets.iter().any(|(w, _)| w == &args.widget_name) {
        let widget_name = args.widget_name.clone();
        let path_to_widget_conf = get_widgets_path().join(&widget_name).join("config.toml");
        let widget_conf = sbbw_widget_conf::validate_config_toml(path_to_widget_conf).unwrap();
        let url_ui = args.url.clone();
        let widget_conf_clone = widget_conf.clone();
        trace!("Widget configuration loaded: {:?}", &widget_conf);

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_decorations(false)
            .with_title(&widget_conf.name)
            .with_always_on_top(widget_conf.always_on_top)
            .with_position(Position::Logical(LogicalPosition::new(
                widget_conf.x as f64,
                widget_conf.y as f64,
            )))
            .with_transparent(widget_conf.transparent)
            .build(&event_loop)
            .unwrap();

        window.update_size(&widget_conf);
        window.set_role(&widget_conf_clone.name, &widget_conf_clone.class_name);
        if widget_conf_clone.stick {
            window.stick();
        }
        if widget_conf_clone.blur {
            window.blur_background();
        }

        thread_local! {
            static WEBVIEWS: RefCell<Option<WebView>> = RefCell::new(None);
        }

        let webview = WebViewBuilder::new(window)
            .unwrap()
            .with_url(&url_ui)
            .unwrap()
            .with_initialization_script(get_initial_js().as_str())
            .with_ipc_handler(move |win, msg| {
                let mut response = SbbwResponse::default();
                let params: Option<Params> = parse_params(&mut response, msg);
                info!("Frontend params have been parsed: {:?}", &params);

                if response.status != 0 {
                    response = process_ipc(win, widget_name.clone(), params.as_ref().unwrap());
                }

                WEBVIEWS.with(|ref_webview| {
                    let webviews = ref_webview.borrow();
                    let webview = webviews.as_ref().unwrap();
                    let response_json = serde_json::to_string(&response).unwrap();
                    trace!("Frontend response: {}", &response_json);
                    let js = format!(
                        r#"window.external.rpc._result({}, {})"#,
                        params.unwrap().method_id,
                        response_json
                    );
                    webview.evaluate_script(js.as_str()).unwrap();
                });
            })
            .with_transparent(widget_conf.transparent)
            .with_dev_tool(args.test)
            .build()
            .unwrap();

        if args.test {
            webview.devtool();
        }

        WEBVIEWS.with(|ref_webview| {
            ref_webview.replace(Some(webview));
        });

        event_loop.run(move |_event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
        });
    } else {
        println!(
            "{}",
            "Widget {} not found or doesn't have config.toml file"
                .red()
                .replace("{}", &args.widget_name.yellow().bold())
        );
    }
}

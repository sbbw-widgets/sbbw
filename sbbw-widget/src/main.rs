#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_imports)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod cmd;
mod ipc;
mod exts;

use clap::Parser;
use cmd::Args;
use exts::*;
use serde::{Deserialize, Serialize};
use tao::window::WindowId;
use url::Url;

use std::{
    cell::RefCell,
    collections::HashMap,
    env,
    fs::{self, File},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use colored::*;
use sbbw_exec::{exec_command, Params};
use sbbw_widget_conf::{get_widgets, get_widgets_path, WidgetSize};
use tauri_plugin_vibrancy::Vibrancy;

use wry::{
    application::{
        dpi::{LogicalPosition, LogicalSize, Position, Size},
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Fullscreen, Window, WindowBuilder},
    },
    http::{
        header::{CONTENT_TYPE, ORIGIN},
        status::StatusCode,
        Request, Response, ResponseBuilder,
    },
    webview::{WebView, WebViewBuilder},
    Value,
};

use crate::ipc::get_initial_js;

#[derive(Serialize)]
struct SbbwResponse {
    pub status: u16,
    pub data: String,
}

fn main() {
    let args = Args::parse();

    let widgets = get_widgets();
    if widgets.contains(&args.widget_name) {
        let widget_name = args.widget_name.clone();
        let path_to_widget_conf = get_widgets_path().join(&widget_name).join("config.toml");
        let path_scripts = get_widgets_path().join(&widget_name).join("scripts");
        let widget_conf = sbbw_widget_conf::validate_config_toml(path_to_widget_conf).unwrap();
        let url_ui = args.url.clone();
        let widget_conf_clone = widget_conf.clone();

        // let widget_scripts_vec: Vec<String> = fs::read_dir(path_scripts)
        //     .unwrap()
        //     .filter_map(|path| {
        //         let path = path.unwrap().path();
        //         if !path.is_dir() {
        //             Some(String::from(
        //                 fs::canonicalize(path).unwrap().to_str().unwrap(),
        //             ))
        //         } else {
        //             None
        //         }
        //     })
        //     .collect();

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

        if widget_conf.width == WidgetSize::Max && widget_conf.height == WidgetSize::Max {
            window.set_fullscreen(Some(Fullscreen::Borderless(window.current_monitor())));
        } else {
            let monitor_size = &window.current_monitor().unwrap().size();
            let width = match widget_conf_clone.width {
                WidgetSize::Max => monitor_size.width as f64,
                WidgetSize::Value(v) => v,
            };
            let height = match widget_conf_clone.height {
                WidgetSize::Max => monitor_size.height as f64,
                WidgetSize::Value(v) => v,
            };
            window.set_inner_size(Size::Logical(LogicalSize::new(width, height)))
        }

        window.set_role(&widget_conf_clone.name, &widget_conf_clone.class_name);
        // window.set_resizable(false).unwrap();
        if widget_conf_clone.stick {
            window.stick();
        }
        if widget_conf_clone.blur {
            #[cfg(target_os = "windows")]
            window.apply_acrylic();
            #[cfg(target_os = "macos")]
            {
                use tauri_plugin_vibrancy::MacOSVibrancy;
                window.apply_vibrancy(tauri_plugin_vibrancy::MacOSVibrancy::AppearanceBased);
            }
        }

        thread_local! {
            static WEBVIEWS: RefCell<Option<WebView>> = RefCell::new(None);
        }

        let webview = WebViewBuilder::new(window)
            .unwrap()
            .with_url(&url_ui)
            .unwrap()
            .with_initialization_script(get_initial_js().as_str())
            .with_ipc_handler(move |_win, msg| {
                let mut response = SbbwResponse {
                    status: StatusCode::OK.as_u16(),
                    data: "{}".to_string(),
                };
                let params: Option<Params> =
                    if let Ok(params) = serde_json::from_str(msg.as_str()) {
                        Some(params)
                    } else {
                        response.status = StatusCode::BAD_REQUEST.as_u16();
                        response.data = "Invalid JSON sended".to_string();
                        Some(Params {
                            method_id: 0,
                            method: "".to_string(),
                            command: "".to_string(),
                            args: Vec::new(),
                        })
                    };

                let method = &params.as_ref().unwrap().method;
                let params_clone = Some(params.as_ref().unwrap().clone());
                if method.is_empty() {
                    response.status = StatusCode::NOT_FOUND.as_u16();
                    response.data = "Invalid command".to_string();
                } else {
                    if method.trim().eq("exec") {
                        if response.status == StatusCode::OK {
                            response.status = StatusCode::OK.as_u16();
                            response.data = exec_command(
                                String::from(path_scripts.to_str().unwrap()),
                                params.unwrap().clone(),
                            )
                            .unwrap();
                        }
                    } else {
                        response.status = StatusCode::NOT_FOUND.as_u16();
                        response.data =
                            format!("Command \"{}\" not found", &method).to_string();
                    };
                }
                WEBVIEWS.with(|ref_webview| {
                    let webviews = ref_webview.borrow();
                    let webview = webviews.as_ref().unwrap();
                    let response_json = serde_json::to_string(&response).unwrap();
                    // println!("response: {}", &response_json);
                    let js = format!(
                        r#"
window.external.rpc._result({}, {})"#,
                        params_clone.unwrap().method_id,
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

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                _ => {}
            }
        });
    } else {
        println!(
            "{}",
            "Widget {} not found or not have config.toml file"
                .red()
                .replace("{}", &args.widget_name.yellow().bold())
        );
        return;
    }
}

#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_imports)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod exts;
mod lua_mod;
use exts::*;
use hlua::{Lua, LuaError, LuaTable};
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Params {
    pub method: String,
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Serialize)]
struct SbbwResponse {
    pub status: u16,
    pub data: String,
}

#[allow(dead_code)]
fn exec_lua(params: Params, lua: &mut Lua<'static>) -> String {
    let file = params.command;
    let file_name = if file.ends_with(".lua") {
        file
    } else {
        format!("{}.lua", file)
    };
    let path = Path::new(&file_name);
    let script = File::open(path).unwrap();
    lua.execute_from_reader::<String, _>(script).unwrap()
}

fn exec_command(pwd: String, params: Params) -> String {
    let file = params.command;
    println!("{}", file);
    let mut args = params.args;
    if file.starts_with("./") {
        args.insert(0, file.clone());
    }
    println!("{:?}", args);
    let output = if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(&["/C", "start"])
            .args(&args)
            .output()
    } else {
        if file.starts_with("./") {
            println!("Execute sh command");
            std::process::Command::new("sh")
                .arg("-c")
                .arg(&args.join(" "))
                .current_dir(pwd)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .output()
        } else {
            println!("Execute command");
            std::process::Command::new(file)
                .args(&args)
                .current_dir(pwd)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .output()
        }
    };

    let stdout = String::from_utf8_lossy(&output.as_ref().unwrap().stdout);
    let stderr = String::from_utf8_lossy(&output.as_ref().unwrap().stderr);

    if !stderr.is_empty() {
        println!("{}", stderr.red());
    }
    if !&stdout.is_empty() {
        println!("{}", stdout.green());
    }

    println!(
        "{}",
        String::from_utf8_lossy(&output.as_ref().unwrap().stdout)
    );

    stdout.to_string()
}

fn main() {
    let args: Vec<_> = env::args().collect();
    // println!("{:?}", args.len());
    // println!("{:?}", args);
    if args.len() > 1 {
        let widgets = get_widgets();
        if widgets.contains(&args[1]) {
            let widget_name = args[1].to_string();
            let path_to_widget_conf = get_widgets_path().join(&widget_name).join("config.toml");
            let path_scripts = get_widgets_path().join(&widget_name).join("scripts");
            let widget_conf = sbbw_widget_conf::validate_config_toml(path_to_widget_conf).unwrap();
            let mut is_testing = false;
            let url_ui = if args.len() == 3 {
                if args[2].contains("http") {
                    is_testing = true;
                    args[2].to_string()
                } else {
                    format!("http://localhost:8000/{}/ui", widget_name)
                }
            } else {
                format!("http://localhost:8000/{}/ui", widget_name)
            };
            // println!("{:?}", url_ui);
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

            let mut lua = Lua::new();
            lua.openlibs();
            let mut sbbw_table = LuaTable::registry(lua);
            sbbw_table.set("widget_name", widget_name);
            // sbbw_table.set("widget_conf", widget_conf.clone().into());
            // sbbw_table.set("widget_scripts", widget_scripts_vec.into());
            // lua.globals_table().set("sbbw", &mut sbbw_table.into());
            //
            thread_local! {
                static WEBVIEWS: RefCell<Option<WebView>> = RefCell::new(None);
            }

            let webview = WebViewBuilder::new(window)
                .unwrap()
                .with_url(&url_ui)
                .unwrap()
                .with_initialization_script(
                    r#"
document.addEventListener('contextmenu', event => event.preventDefault());
                    const executeCommand = (command, args) => {
                      return new Promise((resolve, reject) => {
                          window.ipc.postMessage(JSON.stringify({ method: "exec", command, args }))
                          document.addEventListener("sbbw-response", (e) => {
                            let response = e.detail.response
                            if (response.status == 200)
                              resolve(response.data)
                            else
                              reject({ code: response.status, data: response.data })
                          })
                      })
                    }
                "#,
                )
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
                                method: "".to_string(),
                                command: "".to_string(),
                                args: Vec::new(),
                            })
                        };
                    // println!("params: {:?}", &params.as_ref().unwrap());

                    let method = &params.as_ref().unwrap().method;
                    // let mut response = if &req.method == "exec-lua" {
                    //     Some(RpcResponse::new_result(
                    //         req.id.take(),
                    //         Some(Value::String(exec_lua(params.unwrap(), lua))),
                    //     ))
                    // } else
                    if method.is_empty() {
                        response.status = StatusCode::NOT_FOUND.as_u16();
                        response.data = "Invalid command".to_string();
                    } else {
                        if method.trim().eq("exec") {
                            if response.status == StatusCode::OK {
                                response.status = StatusCode::OK.as_u16();
                                response.data = exec_command(
                                    String::from(path_scripts.to_str().unwrap()),
                                    params.unwrap(),
                                );
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
                            r#"document.dispatchEvent(new CustomEvent("sbbw-response", {{ "detail": {{
                                method: "exec",
                                response: {}
                            }} }}));
                            "#,
                            response_json
                        );
                        // println!("js: {}", js.as_str());
                        webview.evaluate_script(js.as_str()).unwrap();
                    });
                })
                .with_transparent(widget_conf.transparent)
                .with_dev_tool(is_testing)
                .build()
                .unwrap();

            if is_testing {
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
                    .replace("{}", &args[1].yellow().bold())
            );
            return;
        }
    }
}

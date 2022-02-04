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

use std::{
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
    webview::{RpcRequest, RpcResponse, WebViewBuilder},
    Value,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Params {
    pub file: String,
    pub args: Vec<String>,
}

#[allow(dead_code)]
fn exec_lua(params: Params, lua: &mut Lua<'static>) -> String {
    let file = params.file;
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
    let file = params.file;
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
    if args.len() > 1 {
        let widgets = get_widgets();
        if widgets.contains(&args[1]) {
            let widget_name = args[1].to_string();
            let path_to_widget_conf = get_widgets_path().join(&widget_name).join("config.toml");
            let path_scripts = get_widgets_path().join(&widget_name).join("scripts");
            let widget_conf = sbbw_widget_conf::validate_config_toml(path_to_widget_conf).unwrap();
            let url_ui = format!("http://localhost:8000/{}/ui", widget_name);
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

            let handler = move |_window: &Window, mut req: RpcRequest| {
                println!("{:?}", &req.params);
                let params: Option<Params> = if let Some(params) = req.params.take() {
                    let a = if let Ok(mut args) =
                        serde_json::from_value::<Vec<Params>>(params)
                    {
                        println!("{:?}", args[0]);
                        Some(args.remove(0))
                    } else {
                        println!("{:?}", req.params);
                        None
                    };
                    Some(a.unwrap())
                } else {
                    println!("No params");
                    None
                };
                // let mut response = if &req.method == "exec-lua" {
                //     Some(RpcResponse::new_result(
                //         req.id.take(),
                //         Some(Value::String(exec_lua(params.unwrap(), lua))),
                //     ))
                // } else 
                let response = if &req.method == "exec" {
                    Some(RpcResponse::new_result(
                        req.id.take(),
                        Some(Value::String(exec_command(String::from(path_scripts.to_str().unwrap()), params.unwrap()))),
                    ))
                } else {
                    Some(RpcResponse::new_result(
                        req.id.take(),
                        Some(Value::String(format!("Command \"{}\" not found", &req.method).to_string())),
                    ))
                };
                response
            };

            let _webview = WebViewBuilder::new(window)
                .unwrap()
                .with_url(&url_ui)
                .unwrap()
                .with_rpc_handler(handler)
                .with_transparent(widget_conf.transparent)
                .build()
                .unwrap();

            event_loop.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Wait;

                match event {
                    _ => { }
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

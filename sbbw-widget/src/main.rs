#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_imports)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod exts;
use exts::*;

use std::env;

use colored::*;
use sbbw_widget_conf::{get_widgets, get_widgets_path, WidgetSize};
use tauri::{generate_context, App, LogicalSize, Manager, Size, WindowUrl, PhysicalSize};
use tauri_plugin_positioner::Positioner;
use tauri_plugin_vibrancy::Vibrancy;

use tauri::WindowBuilder;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let widgets = get_widgets();
        if widgets.contains(&args[1]) {
            let widget_name = args[1].to_string();
            let path_to_widget_conf = get_widgets_path().join(&widget_name).join("config.toml");
            let widget_conf = sbbw_widget_conf::validate_config_toml(path_to_widget_conf).unwrap();

            let main_app = tauri::Builder::default()
                .plugin(Positioner::default())
                .build(generate_context!())
                .unwrap();

                let url_ui = format!("http://localhost:8000/{}/ui", widget_name);
                let app_handle = main_app.handle();
                let widget_conf_clone = widget_conf.clone();
                tauri::async_runtime::spawn(async move {
                    app_handle
                        .clone()
                        .create_window(
                            "main",
                            WindowUrl::External(url::Url::parse(url_ui.as_str()).unwrap()),
                            |w_builder, attr| {
                                let builder = w_builder
                                    .clone()
                                    .title(&widget_conf.name)
                                    .decorations(false)
                                    .skip_taskbar(true)
                                    .title(&widget_conf_clone.name)
                                    .position(
                                        widget_conf_clone.x as f64,
                                        widget_conf_clone.y as f64,
                                    )
                                    .fullscreen(widget_conf_clone.width == WidgetSize::Max && widget_conf_clone.height == WidgetSize::Max)
                                    .always_on_top(widget_conf_clone.always_on_top)
                                    .transparent(widget_conf_clone.transparent);
                                (builder, attr)
                            },
                        )
                        .unwrap();
                    let window = app_handle.get_window("main").unwrap();
                    let monitor_size = *window.current_monitor().unwrap().unwrap().size();
                    let width = match widget_conf_clone.width {
                        WidgetSize::Max => {
                            monitor_size.width as f64
                        }
                        WidgetSize::Value(v) => v,
                    };
                    let height = match widget_conf_clone.height {
                        WidgetSize::Max => {
                            monitor_size.height as f64
                        }
                        WidgetSize::Value(v) => v,
                    };
                    println!("Height: {}", format!("{}", &height).green());
                    let size = Size::Logical(LogicalSize { width, height });
                    window.set_min_size(Some(size)).unwrap();
                    window.set_max_size(Some(size)).unwrap();
                    window.set_size(size).unwrap();
                    window.set_role(&widget_conf.name, &widget_conf.class_name);
                    window.set_resizable(false).unwrap();
                    if widget_conf_clone.stick {
                        window.stick();
                    }
                    if widget_conf_clone.blur {
                        #[cfg(target_os = "windows")]
                        window.apply_acrylic();
                        #[cfg(target_os = "macos")]
                        {
                            use tauri_plugin_vibrancy::MacOSVibrancy;
                            window.apply_vibrancy(
                                tauri_plugin_vibrancy::MacOSVibrancy::AppearanceBased,
                            );
                        }
                    }
                });

                main_app.run(|_app_handler, _event| { });
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

#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_imports)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::env;

use colored::*;
use sbbw_widget_conf::{get_widgets, get_widgets_path};
use tauri::{generate_context, App, Manager, WindowUrl};
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

            tauri::Builder::default()
                .setup(move |app| {
                    let url_ui = format!("http://localhost:8000/{}/ui", widget_name);
                    app.create_window(
                        "main",
                        WindowUrl::External(url::Url::parse(url_ui.as_str()).unwrap()),
                        |w_builder, attr| {
                            let widget_conf = widget_conf.clone();
                            let builder = w_builder.clone()
                                .resizable(false)
                                .decorations(false)
                                .skip_taskbar(true)
                                .title(&widget_conf.name)
                                .position(widget_conf.x as f64, widget_conf.y as f64)
                                .always_on_top(widget_conf.always_on_top)
                                .transparent(widget_conf.transparent);
                            (builder, attr)
                        },
                    ).unwrap();
                    if widget_conf.blur {
                        let window = app.get_window("main").unwrap();
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
                    Ok(())
                })
                .run(generate_context!())
                .expect("error while running tauri application");
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

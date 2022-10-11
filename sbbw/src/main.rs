#![feature(proc_macro_hygiene, decl_macro, str_split_as_str)]
use actix_web::{App, HttpServer};
use cmd::{
    args::{get_args, to_request, WidgetCommands},
    install::install_widget,
};
use colored::*;
use fork::{fork, Fork};
use lazy_static::lazy_static;
use log::error;
use sbbw_exec::autostarts;
use sbbw_widget_conf::{
    exits_widget, generate_config_sbbw, get_config_sbbw, get_widgets, get_widgets_path,
    validate_config_toml, SbbwConfig,
};
use std::{
    env,
    process::Command,
    sync::{Arc, Mutex},
};
use sysinfo::{System, SystemExt};
use widget::rpc::routes;

use crate::widget::prelude::listen_keybinds;

mod cmd;
mod widget;

const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

lazy_static! {
    pub static ref SBBW_CONFIG: Arc<Mutex<Option<SbbwConfig>>> = Arc::new(Mutex::new(None));
}

fn main() {
    let widgets = get_widgets();
    let args = get_args().lock().unwrap();
    env_logger::Builder::new()
        // .filter_level(args.verbose.log_level_filter())
        .filter_level(log::LevelFilter::Trace)
        .init();

    if args.show_windows {
        println!("{}", "Installed widgets:".blue().bold());
        for (widget, _) in &widgets {
            println!("{} {}", "-".blue(), widget);
        }
    }

    match Command::new("sbbw-widget").args(["-h"]).spawn() {
        Ok(mut c) => {
            c.kill().unwrap();
            drop(c);
        }
        Err(e) => {
            println!(
                "{} reason: {:?}",
                "Binary for launch Widgets not found".red().bold(),
                e
            );
        }
    }

    autostarts();

    if let Some(cmd) = &args.widget_cmd {
        let mut port = args.port;
        let mut conf = SBBW_CONFIG.lock().unwrap();
        *conf = match get_config_sbbw() {
            Ok(cfg) => {
                port = cfg.port;
                Some(cfg)
            }
            Err(_) => {
                println!("Generating default config file");
                let cfg = SbbwConfig::default();
                generate_config_sbbw(cfg.clone()).expect("Failed generating config File");
                Some(cfg)
            }
        };

        match cmd {
            WidgetCommands::Run => {
                {
                    let s = System::new_all();
                    let processes = s.processes_by_exact_name("sbbw");
                    if processes.count() > 1 {
                        println!("{}", "Other Sbbw Daemon is running".red().bold());
                        std::process::exit(1);
                    }
                }
                println!("{}", "Sbbw Daemon".green());
                if !args.no_fork {
                    match fork() {
                        Ok(Fork::Parent(_)) => std::process::exit(0),
                        _ => println!("Cannot create fork"),
                    }
                }

                let conf = conf.clone();
                if let Some(cfg) = conf {
                    if args.port != port {
                        generate_config_sbbw(SbbwConfig {
                            port: args.port,
                            ..cfg.clone()
                        })
                        .unwrap();
                    }
                    listen_keybinds(cfg);
                }

                let server = HttpServer::new(move || App::new().configure(routes))
                    .bind(("0.0.0.0", port))
                    .unwrap()
                    .workers(4)
                    .run();
                match actix_rt::System::new().block_on(server) {
                    Ok(_) => {} // Call when the server is closed
                    Err(e) => println!("Error: {}", e),
                }
            }
            WidgetCommands::Check { widget_name } => {
                if exits_widget(widget_name.to_string()) {
                    let path_conf = get_widgets_path().join(widget_name).join("config.toml");
                    match validate_config_toml(path_conf) {
                        Ok(_) => {
                            println!(
                                "{}",
                                "Config of widget {} is valid"
                                    .green()
                                    .replace("{}", &widget_name.yellow().bold())
                            );
                        }
                        Err(e) => {
                            error!("Error on check config {e}");
                            println!(
                                "{}",
                                "Config of widget {} is not valid"
                                    .red()
                                    .replace("{}", &widget_name.yellow().bold())
                            );
                        }
                    }
                }
            }
            WidgetCommands::Install {
                repo,
                new_name,
                service,
            } => {
                match install_widget(WidgetCommands::Install {
                    repo: repo.to_string(),
                    new_name: new_name.to_owned(),
                    service: service.to_owned(),
                }) {
                    Err(e) => println!("{e}"),
                    Ok(_) => println!("Success Widget installed"),
                }
            }
            x => match to_request(x, format!("http://localhost:{}", port)) {
                Ok(req) => {
                    let client = reqwest::Client::new();
                    let post = client
                        .post(format!("http://localhost:{}/rpc", port))
                        .json(&req)
                        .send();
                    match actix_rt::System::new().block_on(post) {
                        Ok(res) => {
                            if !res.status().is_success() {
                                println!("Cannot comunicate with daemon");
                            } else {
                                std::process::exit(1);
                            }
                        }
                        Err(e) => println!("Error calling daemon: {}", e),
                    }
                }
                Err(e) => {
                    error!("Failed map request: {e}")
                }
            },
        };
    }
}

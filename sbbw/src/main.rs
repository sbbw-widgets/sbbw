#![feature(proc_macro_hygiene, decl_macro, str_split_as_str)]
use actix_web::{App, HttpServer};
use cmd::{
    args::{get_args, to_request, WidgetCommands},
    install::install_widget,
};
use colored::*;
use fork::{fork, Fork};
use log::error;
use sbbw_exec::autostarts;
use sbbw_widget_conf::{
    exits_widget, generate_config_sbbw, generate_pid_file, get_config_sbbw, get_pid, get_widgets,
    get_widgets_path, remove_pid_file, validate_config_toml, SbbwConfig,
};
use std::{env, process::Command};
use widget::routes;

mod cmd;
mod widget;

const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let widgets = get_widgets();
    let args = get_args().lock().unwrap();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    if args.show_windows {
        println!("{}", "Installed widgets:".blue().bold());
        for widget in &widgets {
            println!("{} {}", "-".blue(), widget);
        }
    }

    match Command::new("sbbw-widget").spawn() {
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
        let port = match get_config_sbbw() {
            Ok(cfg) => cfg.port,
            Err(_) => args.port,
        };

        match cmd {
            WidgetCommands::Run => {
                if !get_pid().is_err() {
                    println!("{}", "Other Sbbw Daemon is running".red().bold());
                    std::process::exit(1);
                }
                println!("{}", "Sbbw Daemon".green());
                if !args.no_fork {
                    match fork() {
                        Ok(Fork::Parent(child)) => {
                            generate_pid_file(child.to_string());
                            std::process::exit(0);
                        }
                        _ => println!("Cannot create fork"),
                    }
                } else {
                    generate_pid_file(std::process::id().to_string());
                }
                generate_config_sbbw(SbbwConfig {
                    port: args.port,
                    ..Default::default()
                })
                .unwrap();

                let server = HttpServer::new(move || App::new().configure(routes))
                    .bind(("0.0.0.0", port))
                    .unwrap()
                    .workers(4)
                    .run();
                match actix_rt::System::new().block_on(server) {
                    Ok(_) => remove_pid_file(), // Call when the server is closed
                    Err(e) => {
                        remove_pid_file();
                        println!("Error: {}", e)
                    }
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

#![feature(proc_macro_hygiene, decl_macro, str_split_as_str)]
use actix_web::{App, HttpServer};
use cmd::get_args;
use colored::*;
use sbbw_exec::autostarts;
use sbbw_widget_conf::{get_widgets, get_widgets_path, validate_config_toml};
use std::{env, process::Command};
use fork::fork;
use widget::routes;

mod cmd;
mod daemon;
mod widget;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let widgets = get_widgets();
    let args = get_args().lock().unwrap();

    if args.show_windows {
        println!("{}", "Installed widgets:".blue().bold());
        for widget in widgets {
            println!("{} {}", "-".blue(), widget);
        }
        return Ok(());
    }

    println!("{}", "Sbbw Daemon".green());

    if let Some(value) = &args.check_config {
        if widgets.contains(&value.clone()) {
            let path_conf = get_widgets_path().join(value).join("config.toml");
            if path_conf.exists() {
                if validate_config_toml(path_conf).is_err() {
                    println!(
                        "{}",
                        "Config of widget {} is not valid"
                            .red()
                            .replace("{}", &value.yellow().bold())
                    );
                    return Ok(());
                } else {
                    println!(
                        "{}",
                        "Config of widget {} is valid"
                            .green()
                            .replace("{}", &value.yellow().bold())
                    );
                    return Ok(());
                }
            }
        }
        println!(
            "{}",
            "Widget {} not found"
                .red()
                .replace("{}", &value.yellow().bold())
        );
        return Ok(());
    }

    match Command::new("sbbw-widget").spawn() {
        Ok(_) => println!("{}", "Binary for launch Widgets alredy exists".green()),
        Err(e) => {
            println!(
                "{} reason: {:?}",
                "Binary for launch Widgets not found".red().bold(),
                e
            );
            return Ok(());
        }
    }

    autostarts();

    if !args.no_fork {
        fork().unwrap();
    }

    HttpServer::new(move || {
        App::new()
            // .wrap(Logger::default())
            // .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(routes)
    })
    .bind(("0.0.0.0", args.port))
    .unwrap()
    .run()
    .await
}

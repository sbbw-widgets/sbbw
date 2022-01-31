#![feature(proc_macro_hygiene, decl_macro)]
use clap::{App, Arg};
use colored::*;
use daemon::{Daemon, TransferData};
use rocket::response::{status::NotFound, NamedFile};
use sbbw_widget_conf::{get_widgets, get_widgets_path, validate_config_toml};
use std::{net::IpAddr, path::PathBuf, rc::Rc, process::Command, sync::{Arc, Mutex}, collections::HashMap};

#[macro_use]
extern crate rocket;

mod daemon;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[get("/<file..>")]
fn load_widget(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = get_widgets_path().join(file);
    NamedFile::open(&path).map_err(|e| NotFound(e.to_string()))
}

#[tokio::main]
async fn main() {
    // convert themes into &[&str]
    let widgets = get_widgets();
    let widgets: Vec<&str> = widgets.iter().map(|s| s.as_str()).collect();
    let matches = App::new("Sbbw Daemon")
        .about(DESCRIPTION)
        .version(VERSION)
        .author(AUTHORS)
        .args(&[
            // Arg::new("ip")
            //     .short('i')
            //     .long("ip")
            //     .value_name("IP")
            //     .help("IP address to listen on")
            //     .takes_value(true)
            //     .default_value("0.0.0.0"),
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Port to listen on")
                .takes_value(true)
                .default_value("8111"),
            Arg::new("open")
                .short('o')
                .long("open")
                .help("Open the widget")
                .takes_value(true)
                .possible_values(&widgets),
            Arg::new("close")
                .short('c')
                .long("close")
                .help("Close the widget")
                .takes_value(true)
                .possible_values(&widgets),
            Arg::new("check-config")
                .long("check-config")
                .help("Check config of the widget")
                .takes_value(true)
                .possible_values(&widgets),
            Arg::new("show-windows")
                .long("show-windows")
                .help("Show all widgets installeds")
                .takes_value(false),
        ])
        .get_matches();

    if matches.is_present("show-windows") {
        println!("{}", "Installed widgets:".blue().bold());
        for widget in widgets {
            println!("{} {}", "-".blue(), widget);
        }
        return;
    }

    let mut command = String::new();
    let mut value_command = String::new();

    println!("{}", "Sbbw Daemon".green());

    if let Some(value) = matches.value_of("open") {
        if widgets.contains(&value) {
            command.push_str("open");
            value_command.push_str(value);
        } else {
            println!(
                "{}",
                "Widget {} not found"
                    .red()
                    .replace("{}", &value.yellow().bold())
            );
            return;
        }
    }

    if let Some(value) = matches.value_of("close") {
        if widgets.contains(&value) {
            command.push_str("close");
            value_command.push_str(value);
        } else {
            println!(
                "{}",
                "Widget {} not found"
                    .red()
                    .replace("{}", &value.yellow().bold())
            );
            return;
        }
    }

    if let Some(value) = matches.value_of("check-config") {
        if widgets.contains(&value) {
            let path_conf = get_widgets_path().join(value).join("config.toml");
            if path_conf.exists() {
                if validate_config_toml(path_conf).is_err() {
                    println!(
                        "{}",
                        "Config of widget {} is not valid"
                            .red()
                            .replace("{}", &value.yellow().bold())
                    );
                    return;
                } else {
                    println!(
                        "{}",
                        "Config of widget {} is valid"
                            .green()
                            .replace("{}", &value.yellow().bold())
                    );
                    return;
                }
            }
        }
        println!(
            "{}",
            "Widget {} not found"
                .red()
                .replace("{}", &value.yellow().bold())
        );
        return;
    }

    match Command::new("sbbw-widget").spawn() {
        Ok(_) => println!("{}", "Binary for launch Widgets alredy exists".green()),
        Err(e) => {
            println!(
                "{} reason: {:?}",
                "Binary for launch Widgets not found".red().bold(),
                e
            );
            return;
        }
    }

    let ip = "0.0.0.0".parse::<IpAddr>().unwrap();
    let port: u16 = matches.value_of("port").unwrap().parse::<u16>().unwrap();

    let mut daemon = Daemon::new(ip, port);
    if command.len() > 0 && value_command.len() > 0 {
        daemon.set_command(command, value_command);
    }

    // create hashmap for save all subprocess excecuted with widget-name as key
    let subprocesses = Arc::new(Mutex::new(HashMap::new()));

    let receiver_data_callback = Rc::new(move |response: TransferData| match response {
        TransferData::Get((command, data)) => match command.as_str() {
            "open" => {
                if subprocesses.lock().unwrap().contains_key(&data) {
                    println!(
                        "{}",
                        "Widget {} already opened".red().replace("{}", &data.yellow().bold())
                    );
                    return;
                }
                println!("Open: {:?}", data);
                let subprocess = Command::new("sbbw-widget")
                    .arg(data.as_str())
                    .spawn()
                    .unwrap();
                subprocesses.lock().unwrap().insert(data, subprocess);
            }
            "close" => {
                if !subprocesses.lock().unwrap().contains_key(&data) {
                    println!(
                        "{}",
                        "Widget {} not running".red().replace("{}", &data.yellow().bold())
                    );
                    return;
                }
                println!("Close: {:?}", data);
                if let Some(mut subprocess) = subprocesses.lock().unwrap().remove(&data) {
                    subprocess.kill().unwrap();
                }
            }
            _ => {
                panic!("{}", "Unknown command".red().bold());
            }
        },
        _ => {}
    });
    daemon.set_callbacks(receiver_data_callback);

    tokio::spawn(async { rocket::ignite().mount("/", routes![load_widget]).launch() });
    tokio::join!(async move { daemon.run().await });
}

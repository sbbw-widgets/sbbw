#![feature(proc_macro_hygiene, decl_macro, str_split_as_str)]
use clap::{App, Arg};
use colored::*;
use daemon::{Daemon, TransferData};
use rocket::response::{content, status::NotFound, NamedFile};
use sbbw_widget_conf::{get_config_path, get_widgets, get_widgets_path, validate_config_toml};
use sbbw_exec::autostarts;
use std::{
    collections::HashMap,
    env,
    fs::{File, OpenOptions},
    net::{IpAddr, TcpStream},
    path::PathBuf,
    process::{Command, Stdio},
    rc::Rc,
    sync::{Arc, Mutex},
};

#[macro_use]
extern crate rocket;

mod daemon;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[get("/<file..>")]
fn load_widget(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    println!("{} {}", "Loading:".green().bold(), file.to_str().unwrap());
    let path = get_widgets_path().join(&file);
    if path.is_file() {
        NamedFile::open(&path).map_err(|e| NotFound(e.to_string()))
    } else if path.is_dir() {
        NamedFile::open(&path.join("index.html")).map_err(|e| NotFound(e.to_string()))
    } else {
        let mut path_arr = file.to_str().unwrap().split("/");
        let widget_name = path_arr.next().unwrap();
        let file = PathBuf::from(path_arr.as_str());
        let path = get_widgets_path().join(widget_name).join("ui").join(&file);
        println!(
            "{} {}",
            "Path converted:".green().bold(),
            &path.to_str().unwrap()
        );
        NamedFile::open(&path).map_err(|e| NotFound(e.to_string()))
    }
}

#[catch(404)]
fn default_catcher() -> content::Html<&'static str> {
    content::Html(
        r#"
<html lang="en">
<head>
    <meta charset="UTF-8">
    <style type="text/css">
        * {
            margin: 0;
            padding: 0;
        }
        body {
            align-items: center;
            background: #fafafa;
            display: flex;
            flex-flow: column wrap;
            height: 100vh;
            justify-content: center;
            overflow: hidden;
            width: 100%;
        }
        h1 {
            color: #404040;
            font-size: 8em;
            font-weight: 200;
        }
        span {
            color: #404040;
            font-size: 1.5em;
            font-weight: 200;
        }
    </style>
    </head>
    <body>
        <h1>404</h1>
        <span>A error ocurred</span>
    </body>
</html>
    "#,
    )
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
                .conflicts_with_all(&["close", "test"])
                .possible_values(&widgets),
            Arg::new("close")
                .short('c')
                .long("close")
                .help("Close the widget")
                .takes_value(true)
                .conflicts_with_all(&["open", "test"])
                .possible_values(&widgets),
            Arg::new("toggle")
                .short('t')
                .long("toggle")
                .help("Toggle view the widget")
                .conflicts_with_all(&["open", "close", "test"])
                .takes_value(true)
                .possible_values(&widgets),
            Arg::new("test")
                .long("test")
                .help("Test the widget")
                .takes_value(true)
                .min_values(2)
                .multiple_values(true)
                .conflicts_with_all(&["open", "close"])
                .value_names(&["widget_name", "local_server"]),
            Arg::new("check-config")
                .long("check-config")
                .help("Check config of the widget")
                .takes_value(true)
                .conflicts_with_all(&["open", "close", "test", "toggle"])
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

    if let Some(value) = matches.value_of("toggle") {
        if widgets.contains(&value) {
            command.push_str("toggle");
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

    if let Some(value) = matches.values_of("test") {
        let values: Vec<&str> = value.collect();
        if values.len() == 2 {
            command.push_str("test");
            value_command.push_str(values.join(" ").as_str())
        } else {
            println!("{}", "Widget not found".red());
            return;
        }
        println!("Command: {}, Values: {}", command, value_command);
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

    autostarts();

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
                        "Widget {} already opened"
                            .red()
                            .replace("{}", &data.yellow().bold())
                    );
                    return;
                }
                println!("Open: {:?}", data);
                let file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(get_config_path().join(".log"))
                    .unwrap();
                let out = Stdio::from(file);
                let subprocess = Command::new("sbbw-widget")
                    .arg(data.as_str())
                    .stderr(out)
                    .spawn()
                    .unwrap();
                subprocesses.lock().unwrap().insert(data, subprocess);
            }
            "close" => {
                if !subprocesses.lock().unwrap().contains_key(&data) {
                    println!(
                        "{}",
                        "Widget {} not running"
                            .red()
                            .replace("{}", &data.yellow().bold())
                    );
                    return;
                }
                println!("Close: {:?}", data);
                if let Some(mut subprocess) = subprocesses.lock().unwrap().remove(&data) {
                    subprocess.kill().unwrap();
                }
            }
            "toggle" => {
                if !subprocesses.lock().unwrap().contains_key(&data) {
                    println!("Open: {:?}", data);
                    let file = OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(get_config_path().join(".log"))
                        .unwrap();
                    let out = Stdio::from(file);
                    let subprocess = Command::new("sbbw-widget")
                        .arg(data.as_str())
                        .stderr(out)
                        .spawn()
                        .unwrap();
                    subprocesses.lock().unwrap().insert(data, subprocess);
                } else {
                    println!("Close: {:?}", data);
                    if let Some(mut subprocess) = subprocesses.lock().unwrap().remove(&data) {
                        subprocess.kill().unwrap();
                    }
                }
            }
            "test" => {
                if subprocesses.lock().unwrap().contains_key(&data) {
                    println!(
                        "{}",
                        "Widget {} already opened"
                            .red()
                            .replace("{}", &data.yellow().bold())
                    );
                    return;
                }
                println!("Open to Test: {:?}", data);
                let f = File::create(get_config_path().join(".log")).unwrap();
                let out = Stdio::from(f);
                let subprocess = Command::new("sbbw-widget")
                    .args(data.split(" "))
                    .stderr(out)
                    .spawn()
                    .unwrap();
                println!("Args to test: {:?}", data);
                subprocesses.lock().unwrap().insert(data, subprocess);
            }
            _ => {
                println!("{}", "Unknown command".red().bold());
            }
        },
        _ => {}
    });
    daemon.set_callbacks(receiver_data_callback);

    tokio::spawn(async move {
        // let mut config = rocket::Config::new(rocket::config::Environment::Production);
        // config.set_log_level(rocket::config::LoggingLevel::Debug);
        match TcpStream::connect(("0.0.0.0", port)) {
            Ok(_) => {}
            Err(_) => {
                let err = rocket::ignite()
                    .mount("/", routes![load_widget])
                    .register(catchers![default_catcher])
                    .launch();
                drop(err);
            }
        }
    });
    tokio::join!(async move { daemon.run().await });
}

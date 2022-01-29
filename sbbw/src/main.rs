use clap::{App, Arg};
use daemon::{Daemon, TransferData};
use std::{net::IpAddr, rc::Rc};

use utils::get_widgets;

mod daemon;
mod utils;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[tokio::main]
async fn main() {
    // convert themes into &[&str]
    let mut widgets = get_widgets();
    widgets.push("internal".to_string());
    let widgets: Vec<&str> = widgets.iter().map(|s| s.as_str()).collect();
    let matches = App::new("Sbbw Daemon")
        .about(DESCRIPTION)
        .version(VERSION)
        .author(AUTHORS)
        .args(&[
            Arg::new("ip")
                .short('i')
                .long("ip")
                .value_name("IP")
                .help("IP address to listen on")
                .takes_value(true)
                .default_value("0.0.0.0"),
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
                .value_names(&widgets),
            Arg::new("close")
                .short('c')
                .long("close")
                .help("Close the widget")
                .takes_value(true)
                .value_names(&widgets),
            Arg::new("show-windows")
                .long("show-windows")
                .help("Show all widgets installeds")
                .takes_value(false),
        ])
        .get_matches();

    if matches.is_present("show-windows") {
        println!("Installed widgets:");
        for widget in widgets {
            println!("- {}", widget);
        }
        return;
    }

    let ip = matches.value_of("ip").unwrap().parse::<IpAddr>().unwrap();
    let port: u16 = matches.value_of("port").unwrap().parse::<u16>().unwrap();

    let mut daemon = Daemon::new(ip, port);
    // daemon.set_command(
    //     "open".to_string(),
    //     matches.value_of("open").unwrap().to_string(),
    // );
    let receiver_data_callback = Rc::new(move |response: TransferData| match response {
        TransferData::Get((command, data)) => match command.as_str() {
            "open" => {
                println!("Open: {:?}", data);
            }
            "close" => {
                println!("Close: {:?}", data);
            }
            _ => {
                panic!("Unknown command");
            }
        },
        _ => {}
    });
    daemon.set_callbacks(receiver_data_callback);

    tokio::join!(async move { daemon.run().await });
}

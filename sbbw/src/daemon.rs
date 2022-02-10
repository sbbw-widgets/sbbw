use colored::Colorize;
use serde::Serialize;
use std::{
    io::{Read, Write},
    net::{IpAddr, Shutdown, SocketAddr, TcpListener, TcpStream},
    rc::Rc,
};

type TransferDataFunc = dyn Fn(TransferData);

#[allow(dead_code)]
#[derive(Serialize)]
pub enum TransferData {
    Get((String, String)),
    Set((String, String)),
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Daemon {
    callback: Option<Rc<TransferDataFunc>>,
    socket_addr: SocketAddr,
    client_connected: bool,
    command: (String, String),
}

///
/// Create a new Daemon
/// # Arguments
/// * `socket_addr` - The address to bind to
/// * `callback` - The callback to call when a request is received
///
/// # Returns
/// A new daemon
///
/// daemon -- if not existing -- will be created and listening the other instance commands
/// daemon -- if existing -- will be connected to the main instance and send commands
///
#[allow(dead_code)]
impl Daemon {
    pub fn new(ip: IpAddr, port: u16) -> Self {
        Self {
            socket_addr: SocketAddr::new(ip, port),
            callback: Some(Rc::new(|_| {})),
            client_connected: false,
            command: ("".to_string(), "".to_string()),
        }
    }

    pub fn set_callbacks(&mut self, callback: Rc<TransferDataFunc>) {
        self.callback = Some(callback);
    }
    pub fn set_command(&mut self, command: String, args: String) {
        self.command = (command, args);
    }

    pub fn is_connected(&self) -> bool {
        self.client_connected
    }
    fn set_connected(&mut self, connected: bool) {
        self.client_connected = connected;
    }

    fn make_client(&mut self, stream: TcpStream) {
        self.set_connected(true);
        self.send_command(Some(stream), self.command.0.clone(), self.command.1.clone());
    }

    #[allow(unused_comparisons)]
    async fn handle_client(&self, stream: &mut TcpStream) {
        let mut data = String::new();
        'receiver: while match stream.read_to_string(&mut data) {
            Ok(size) => {
                if size == 0 {
                    break 'receiver;
                }
                println!("Size of data: {}", size);
                println!("Content of data: {}", data);
                let data_split = data.split('|').map(String::from).collect::<Vec<String>>();
                let data_response =
                    TransferData::Get((data_split[0].clone(), data_split[1].clone()));
                (self.callback.as_ref().unwrap())(data_response);
                stream.shutdown(Shutdown::Both).unwrap();
                drop(&stream);
                true
            }
            Err(e) => {
                println!("[{}] {}", "Error".red().bold(), e);
                stream.shutdown(Shutdown::Both).unwrap();
                false
            }
        } {}
    }

    async fn make_server(&mut self) {
        let addr = self.socket_addr.clone();
        let listener = TcpListener::bind(addr).unwrap();
        // accept connections and process them, spawning a new thread for each one
        println!("{}", "Daemon running".blue());
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    self.handle_client(&mut stream).await
                }
                Err(e) => {
                    println!("[{}] {}", "Error".red().bold(), e);
                }
            }
        }
        // close the socket server
        drop(listener);
    }

    pub async fn run(&mut self) {
        let addr = self.socket_addr.clone();
        match TcpStream::connect(&addr) {
            Ok(stream) => {
                println!("{}", "Successfully connected to daemon".green().bold());
                self.make_client(stream);
            }
            Err(_e) => {
                // create new conection
                println!("{}", "Creating new daemon".blue().bold());
                self.make_server().await;
            }
        }
    }

    pub fn send_command(&self, client_stream: Option<TcpStream>, command: String, data: String) {
        match client_stream.as_ref() {
            Some(mut stream) => {
                println!("{}: \"{}\" \"{}\"", "Sending command to daemon".blue().bold(), command, data);
                let data_response = format!("{}|{}", command, data);
                stream.write(data_response.as_bytes()).unwrap();
                stream.flush().unwrap();
                stream.shutdown(Shutdown::Both).unwrap();
            }
            None => {
                println!("{}", "The main daemon is not running".green().bold());
            }
        }
    }
}

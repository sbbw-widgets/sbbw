#![allow(unused)]

use actix_files::NamedFile;
use actix_web::{get, web, Error, HttpRequest, Result};
use colored::*;
use sbbw_widget_conf::{get_config_path, get_widgets, get_widgets_path};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::Error as IOError,
    ops::Deref,
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::Mutex,
};

lazy_static::lazy_static! {
    pub static ref WIDGETS: Mutex<HashMap<String, Child>> = Mutex::new(HashMap::new());
}

pub fn get_state() -> &'static impl Deref<Target = Mutex<HashMap<String, Child>>> {
    &WIDGETS
}

#[derive(Deserialize)]
pub struct RpcData {
    pub widget_name: String,
    pub widget_params: Vec<String>,
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    // static files or website
    cfg.default_service(
        actix_files::Files::new("/", get_widgets_path())
            .index_file("index.html")
            .path_filter(|path, _| {
                let widgets = get_widgets();
                let widget_name = path
                    .to_str()
                    .unwrap()
                    .split("/")
                    .next()
                    .unwrap()
                    .to_string();
                widgets.contains(&widget_name)
            }),
    );
    // api service
    // .service(web::scope("/rpc").route("", web::post().to(rpc)));
}

// fn rpc(body: JsonBody<RpcData>) -> HttpResponse {
//     HttpResponse::Ok()
// }

fn open_widget(key: String) -> Result<(), String> {
    let mut widgets = get_state().lock().unwrap();
    if !widgets.contains_key(&key) {
        return Err("Widget {} already opened"
            .red()
            .replace("{}", &key.yellow().bold()));
    }
    println!("Open: {:?}", key);
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(get_config_path().join(".log"))
        .unwrap();

    let out = Stdio::from(file);
    let subprocess = Command::new("sbbw-widget")
        .args(key.split_whitespace())
        .stderr(out)
        .spawn()
        .unwrap();
    widgets.insert(key, subprocess);
    Ok(())
}

fn close_widget(key: String) -> Result<(), String> {
    let mut widgets = get_state().lock().unwrap();

    if !widgets.contains_key(&key) {
        return Err("Widget {} not running"
            .red()
            .replace("{}", &key.yellow().bold()));
    }
    println!("Close: {:?}", key);
    if let Some(mut subprocess) = widgets.remove(&key) {
        subprocess.kill().unwrap();
        drop(subprocess);
    }
    Ok(())
}

fn toggle_widget(key: String) -> Result<(), String> {
    let widgets = get_state().lock().unwrap();
    if !widgets.contains_key(&key) {
        open_widget(key)
    } else {
        close_widget(key)
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, dev::Service, http, test, web, App, Error};

    use super::*;

    #[actix_web::test]
    async fn test_widget_req() -> Result<(), Error> {
        let app = App::new().configure(routes);
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/sidebar/ui").to_request();
        println!("{:?}", &req);
        let resp = app.call(req).await?;

        println!("{:?}", resp);

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = resp.into_body();
        print!("{:?}", response_body);

        Ok(())
    }
}

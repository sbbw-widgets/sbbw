#![allow(unused)]

use actix_files::NamedFile;
use actix_web::{
    error, get,
    http::StatusCode,
    web::{self, Json},
    Error, HttpResponse, Result,
};
use colored::*;
use sbbw_widget_conf::{get_config_path, get_widgets, get_widgets_path, RpcAction, RpcDataRequest};
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
    )
    // api service
    .service(web::scope("/rpc").route("", web::post().to(rpc)));
}

async fn rpc(body: Json<RpcDataRequest>) -> HttpResponse {
    match body.action {
        RpcAction::Open => open_widget(body).await,
        RpcAction::Close => close_widget(body).await,
        RpcAction::Toggle => toggle_widget(body).await,
        RpcAction::Test => toggle_widget(body).await,
        _ => HttpResponse::BadRequest().finish(),
    }
}

async fn open_widget(data: Json<RpcDataRequest>) -> HttpResponse {
    let mut widgets = get_state().lock().unwrap();
    if !widgets.contains_key(&data.widget_name) {
        return HttpResponse::build(StatusCode::UNAUTHORIZED).body(
            "Widget {} already opened"
                .red()
                .replace("{}", &data.widget_name.yellow().bold()),
        );
    }
    println!("Open: {:?}", data.widget_name);
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(get_config_path().join(".log"))
        .unwrap();

    let out = Stdio::from(file);
    let subprocess = Command::new("sbbw-widget")
        .args(data.clone().get_args())
        .stderr(out)
        .spawn()
        .unwrap();
    widgets.insert(data.widget_name.clone(), subprocess);
    HttpResponse::Ok().finish()
}

async fn close_widget(data: Json<RpcDataRequest>) -> HttpResponse {
    let mut widgets = get_state().lock().unwrap();

    if !widgets.contains_key(&data.widget_name) {
        return HttpResponse::build(StatusCode::BAD_GATEWAY).body(
            "Widget {} not running"
                .red()
                .replace("{}", &data.widget_name.yellow().bold()),
        );
    }
    println!("Close: {:?}", data.widget_name);
    if let Some(mut subprocess) = widgets.remove(&data.widget_name) {
        subprocess.kill().unwrap();
        drop(subprocess);
    }
    HttpResponse::Ok().finish()
}

async fn toggle_widget(data: Json<RpcDataRequest>) -> HttpResponse {
    let widgets = get_state().lock().unwrap();
    if !widgets.contains_key(&data.widget_name) {
        open_widget(data).await
    } else {
        close_widget(data).await
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

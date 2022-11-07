use actix_web::{
    web::{self, Json},
    HttpResponse,
};
use colored::*;
use log::{info, trace};
use sbbw_widget_conf::{get_widgets, get_widgets_path, RpcAction, RpcDataRequest};
use std::{collections::HashMap, process::Child};

use crate::widget::{
    prelude::{close_widget, open_widget, toggle_widget},
    WIDGETS,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    // static files or website
    cfg.default_service(
        actix_files::Files::new("/", get_widgets_path())
            .index_file("index.html")
            .path_filter(|path, _| {
                trace!("[{}] Url requested: {:?}", "Daemon".green().bold(), path);
                let widgets = get_widgets();
                let widget_name = path
                    .to_str()
                    .unwrap()
                    .split('/')
                    .next()
                    .unwrap()
                    .to_string();
                widgets.iter().any(|(w, _)| w == &widget_name)
            }),
    )
    // api service
    .service(web::scope("/rpc").route("", web::post().to(rpc)));
}

async fn rpc(body: Json<RpcDataRequest>) -> HttpResponse {
    info!("[{}] Data received: {:?}", "Daemon".green().bold(), &body);
    let mut widgets = WIDGETS.lock().unwrap();
    match body.action {
        RpcAction::Open | RpcAction::Test => open_widget_rpc(&mut widgets, body),
        RpcAction::Close => close_widget_rpc(&mut widgets, body),
        RpcAction::Toggle => toggle_widget_rpc(&mut widgets, body),
    }
}

fn open_widget_rpc(
    widgets: &mut HashMap<String, Child>,
    data: Json<RpcDataRequest>,
) -> HttpResponse {
    info!(
        "[{}] Current widgets openned: {:?}",
        "Daemon".green().bold(),
        widgets
    );
    let RpcDataRequest {
        widget_name,
        action,
        url,
        widget_params,
    } = data.0;
    match open_widget(
        widgets,
        &(widget_name, None),
        action,
        widget_params.unwrap_or_default(),
        Some(url),
    ) {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

fn close_widget_rpc(
    widgets: &mut HashMap<String, Child>,
    data: Json<RpcDataRequest>,
) -> HttpResponse {
    info!(
        "[{}] Current widgets openned: {:?}",
        "Daemon".green().bold(),
        widgets
    );
    let RpcDataRequest {
        widget_name,
        action: _,
        url: _,
        widget_params: _,
    } = data.0;
    match close_widget(widgets, &(widget_name, None)) {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

fn toggle_widget_rpc(
    widgets: &mut HashMap<String, Child>,
    data: Json<RpcDataRequest>,
) -> HttpResponse {
    info!(
        "[{}] Current widgets openned: {:?}",
        "Daemon".green().bold(),
        widgets
    );

    let RpcDataRequest {
        widget_name,
        action,
        url,
        widget_params,
    } = data.0;
    match toggle_widget(
        widgets,
        &(widget_name, None),
        action,
        widget_params.unwrap_or_default(),
        Some(url),
    ) {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{dev::Service, http, test, App, Error};

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

use actix_web::{get, HttpResponse, Responder, web, http::header::ContentType};
use std::sync::atomic::Ordering;

use crate::AppState;
use crate::middleware::request_info;
use super::structs::health::HealthInfo;

//Revisa el estado de la aplicacion y devuelve la informacion de la app
#[get("/healthchecker")]
async fn healthchecker(data: web::Data<AppState>) -> impl Responder {
    request_info::count_increase(data.clone());

    let info = HealthInfo {
        app_name: data.app_name.to_string(),
        connections_number: data.connections.load(Ordering::Relaxed),
        total_request_recibed: *data.requests_recibed.lock().unwrap(),
        is_alive: data.alive
    };
    HttpResponse::Ok()
    .content_type(ContentType::json())
    .json(info)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(healthchecker);
}
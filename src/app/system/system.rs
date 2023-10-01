use actix_web::{get, HttpResponse, Responder, web, http::header::ContentType};
use std::sync::atomic::Ordering;

use crate::AppState;
use super::structs::health::{HealthInfo};

//Revisa el estado de la aplicacion y devuelve la informacion de la app
#[get("/healthchecker")]
async fn healthchecker(data: web::Data<AppState>) -> impl Responder {
    let mut request_count = data.requests_recibed.lock().unwrap();
    *request_count += 1;

    let info = HealthInfo {
        app_name: data.app_name.to_string(),
        connections_number: data.connections.load(Ordering::Relaxed),
        total_request_recibed: *request_count,
        is_alive: data.alive
    };
    HttpResponse::Ok()
    .content_type(ContentType::json())
    .json(info)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(healthchecker);
}
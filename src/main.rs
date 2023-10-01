use std::sync::{Arc, atomic::{AtomicUsize, Ordering}, Mutex};

use actix_web::{get, HttpResponse, Responder, web, http::header::ContentType, middleware::Logger};

use serde::Serialize;

use env_logger::Env;

#[derive(Serialize)]
struct HealthInfo {
    app_name: String,
    connections_number: usize,
    total_request_recibed: usize,
    is_alive: bool,
}

struct AppState {
    app_name: String,
    connections: Arc<AtomicUsize>,
    requests_recibed: Mutex<usize>,
    alive: bool
}

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let data = web::Data::new(AppState {
        app_name: String::from("Practica API Rest con Actix-Web"),
        connections: Arc::new(AtomicUsize::new(0)),
        requests_recibed: Mutex::new(0),
        alive: true
    });

    HttpServer::new(move || {
        App::new()
        //.wrap(Logger::default())
        .wrap(Logger::new("%a %t %r %s %b %{Referer}i %{User-Agent}i %D"))
        .app_data(data.clone())
        .service(healthchecker)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
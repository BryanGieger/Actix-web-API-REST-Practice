use std::sync::{Arc, atomic::AtomicUsize, Mutex};
use actix_web::{web, middleware::Logger};
use serde::Serialize;
use env_logger::Env;

use app::system::system::config;

mod app;

pub struct AppState {
    pub app_name: String,
    pub connections: Arc<AtomicUsize>,
    pub requests_recibed: Mutex<usize>,
    pub alive: bool
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
        .configure(config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
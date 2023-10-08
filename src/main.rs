use std::sync::{Arc, atomic::AtomicUsize, Mutex};
use actix_web::{web, middleware::Logger};
use env_logger::Env;
use dotenv;

mod models;
mod routes;
mod controllers;
mod middleware;
mod auth;
mod db;

use models::AppState;
use mongodb::Client;
use routes::{config, config_tests};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv::from_filename(".env").ok();
    let uri = dotenv::var("MONGO_DB").unwrap();
    let client = Client::with_uri_str(uri).await.expect("error al conectar");

    let data = web::Data::new(AppState {
        app_name: String::from("Practica API Rest con Actix-Web"),
        app_desc: String::from("Ruta de pruebas: app/test/ | Ruta de la app: app/"),
        n_connections: Arc::new(AtomicUsize::new(0)),
        n_connections_errors: Arc::new(AtomicUsize::new(0)),
        n_requests_recibed: Mutex::new(0),
        n_requests_errrors: Mutex::new(0),
        mongodb_client: client.clone(),
    });

    log::info!("Start server at localhost:8080 🚀");

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
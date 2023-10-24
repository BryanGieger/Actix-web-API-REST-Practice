use std::sync::{Arc, atomic::AtomicUsize, Mutex};
use actix_web::{web::Data, middleware::Logger, rt::task};
use env_logger::Env;
use dotenv;
use mongodb::sync::{
    Client as SyncClient,
};

use mongodb::{
    Client as AsyncClient,
};

mod models;
mod routes;
mod controllers;
mod middleware;
mod auth;
mod db;
mod error;

use db::{init_async_mongodbcli, init_sync_mongodbcli, redis_conn};
use middleware::prepare_jwt;
//use routes::{config, config_tests};
use routes::config;

//Estructura que manejara la informacion del estado de la aplicación
pub struct AppState {
    pub app_name: String,
    pub app_desc: String,
    pub n_connections: Arc<AtomicUsize>,
    pub n_connections_errors: Arc<AtomicUsize>,
    pub n_requests_recibed: Mutex<usize>,
    pub n_requests_errrors: Mutex<usize>,
}

impl AppState {
    fn new() -> AppState{
        AppState {
            app_name: String::from("Practica API Rest con Actix-Web"),
            app_desc: String::from("Ruta de pruebas: app/test/ | Ruta de la app: app/"),
            n_connections: Arc::new(AtomicUsize::new(0)),
            n_connections_errors: Arc::new(AtomicUsize::new(0)),
            n_requests_recibed: Mutex::new(0),
            n_requests_errrors: Mutex::new(0),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv::from_filename(".env").ok();

    //Prepara los clientes de las bases de datos
    // - Redis
    let redis_pool = redis_conn().await;
    // - MongoDB Sync
    let uri = dotenv::var("MONGO_DB").unwrap();
    let sync_mongodbcli:SyncClient = task::spawn_blocking(|| {
        //SyncClient::with_uri_str(uri).unwrap()
        init_sync_mongodbcli()
    }).await.unwrap();
    // - MongoDB Async
    let async_mongodbcli: AsyncClient = init_async_mongodbcli().await;

    //Prepara el middleware de los JSON WEB TOKENS
    let (storage, 
        factory, 
        jwt_ttl, 
        refresh_ttl) = prepare_jwt(redis_pool.clone());

    /*std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();*/

    log::info!("Servidor iniciado en http://localhost:9090/ 🚀");

    HttpServer::new(move || {
        //let logger = Logger::default();

        App::new()
        .wrap(Logger::new("%a %t %r %s %b %{Referer}i %{User-Agent}i %D"))
        //.wrap(logger)
        .app_data(Data::new(AppState::new()))
        .app_data(Data::new(async_mongodbcli.clone()))
        .app_data(Data::new(sync_mongodbcli.clone()))
        .app_data(Data::new(storage.clone()))
        .app_data(Data::new(jwt_ttl))
        .app_data(Data::new(refresh_ttl))
        .wrap(factory.clone())
        .app_data(Data::new(redis_pool.clone()))
        .configure(config)
    })
    .bind(("0.0.0.0", 9090))?
    .run()
    .await
}
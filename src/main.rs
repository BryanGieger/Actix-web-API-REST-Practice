use std::sync::{Arc, atomic::AtomicUsize, Mutex};
use actix_web::{web::Data, middleware::Logger};
use env_logger::Env;
use dotenv;
use actix_jwt_session::*;

mod models;
mod routes;
mod controllers;
mod middleware;
mod auth;
mod db;

use db::{init_mongodbcli, redis_conn};
use auth::AppClaims;
use routes::{config, config_tests};

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
    // - MongoDB
    let mongodbcli: mongodb::Client = init_mongodbcli().await;

    //Prepara los JSON WEB TOKENS
    let keys = JwtSigningKeys::load_or_create();
    let (storage, factory) = SessionMiddlewareFactory::<AppClaims>::build(
        Arc::new(keys.encoding_key), 
        Arc::new(keys.decoding_key), 
        Algorithm::HS256
    )
    .with_redis_pool(redis_pool.clone())
    .with_jwt_header(JWT_HEADER_NAME)
    .with_jwt_cookie(JWT_COOKIE_NAME)
    .finish();

    let jwt_ttl = JwtTtl(Duration::days(14));
    let refresh_ttl = RefreshTtl(Duration::days(3*31));

    log::info!("Servidor iniciado en http://localhost:9090/ 🚀");

    HttpServer::new(move || {
        App::new()
        .wrap(Logger::new("%a %t %r %s %b %{Referer}i %{User-Agent}i %D"))
        .app_data(Data::new(AppState::new()))
        .app_data(Data::new(mongodbcli.clone()))
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
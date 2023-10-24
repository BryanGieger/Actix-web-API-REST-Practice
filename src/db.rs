use std::time::Duration;

use actix_web::rt::Runtime;

/*use mongodb::{
    Client, bson::doc, options::{SelectionCriteria, InsertOneOptions, ClientOptions
    }, results::InsertOneResult};
use mongodb::sync::{Client as SyncClient, Collection as SyncCollection};*/

use mongodb::sync::{
    Client as SyncClient,
};

use mongodb::{
    Client as AsyncClient,
    Collection as AsyncCollection
};

use mongodb::{
    bson::doc,
    options::{SelectionCriteria, InsertOneOptions, ClientOptions}, 
    results::InsertOneResult
};

use redis::RedisError;
use redis_async_pool::{RedisConnection, RedisPool, RedisConnectionManager, deadpool::managed::Pool};
use dotenv;
use validator::ValidationError;

use crate::auth::AccountModel;

// Inicializar DBClients o Conectar -----------------------
//Inicializa el cliente de MongoDB asincrono
pub async fn init_async_mongodbcli() -> AsyncClient {
    log::info!("Inicializando el cliente asíncrono de MongoDB... 👀");
    
    let uri = dotenv::var("MONGO_DB").unwrap();

    match ClientOptions::parse_async(uri).await {
        Err(err) => {
            log::error!("No se pudo iniciar el cliente asíncrono de MongoDB ❌");
            log::error!("MONGODB::Client::ERROR: {:?}", err);
            std::process::exit(1);
        },
        Ok(mut options) => {
            options.app_name = Some("TaskManager API async".to_string());
            options.connect_timeout = Some(Duration::new(5, 0));
            options.server_selection_timeout = Some(Duration::new(5, 0));
            options.default_database = Some("TaskManager".to_string());
            options.max_connecting = Some(6);
    
            match AsyncClient::with_options(options) {
                Ok(client) => {
                    log::info!("¡¡Cliente asíncrono de MongoDB inicializado!! ✅");
                    client
                }
                Err(err) => {
                    log::error!("No se pudo iniciar el cliente asíncrono de MongoDB ❌");
                    log::error!("MONGODB::Client::ERROR: {:?}", err);
                    std::process::exit(1);
                }
            }
        }
    }
}

//Inicializa el cliente de MongoDB Sincrono
pub fn init_sync_mongodbcli() -> SyncClient {
    log::info!("Inicializando el cliente síncrono de MongoDB... 👀");
    
    let uri = dotenv::var("MONGO_DB").unwrap();

    // match SyncClient::with_options(ClientOptions::parse(uri).unwrap()) {
    //     Ok(client) => {
    //         log::info!("¡¡Cliente síncrono de MongoDB inicializado!! ✅");
    //         client
    //     }
    //     Err(err) => {
    //         log::error!("No se pudo iniciar el cliente síncrono de MongoDB ❌");
    //         log::error!("MONGODB::Client::ERROR: {:?}", err);
    //         std::process::exit(1);
    //     }
    // }

    match ClientOptions::parse(uri) {
        Err(err) => {
            log::error!("No se pudo iniciar el cliente síncrono de MongoDB ❌");
            log::error!("MONGODB::Client::ERROR: {:?}", err);
            std::process::exit(1);
        },
        Ok(mut options) => {
            options.app_name = Some("TaskManager API sync".to_string());
            options.connect_timeout = Some(Duration::new(5, 0));
            options.server_selection_timeout = Some(Duration::new(5, 0));
            options.default_database = Some("TaskManager".to_string());
            options.max_connecting = Some(6);
    
            match SyncClient::with_options(options) {
                Ok(client) => {
                    log::info!("¡¡Cliente síncrono de MongoDB inicializado!! ✅");
                    return client;
                }
                Err(err) => {
                    log::error!("No se pudo iniciar el cliente síncrono de MongoDB ❌");
                    log::error!("MONGODB::Client::ERROR: {:?}", err);
                    std::process::exit(1);
                }
            }
        }
    }
    
}


//Conecta con la base de datos en memoria de Redis
pub async fn redis_conn() -> Pool<RedisConnection, RedisError> {
    let uri = dotenv::var("REDIS_URI").unwrap();

    log::info!("Conectando con el servidor de Redis... 👀");

    let rediscli = redis::Client::open(uri);

    match rediscli {
        Ok(client) => {
            log::info!("¡¡Cliente conectado a Redis!! ✅");
            
            RedisPool::new(
                RedisConnectionManager::new(
                    client,
                    true, 
                    None
                ), 
                5
            )
        }
        Err(err) => {
            log::error!("No se pudo conectar con el servidor de Redis ❌");
            log::error!("Redis::client::error: {}", err.detail().unwrap().to_string());
            std::process::exit(2);
        }
    }

    
}

// Metodos que interactuan con la DB ----------------------

//MongoDB - Revisa el estado del servidor de MongoDB
pub async fn mongodb_status(client: AsyncClient) -> Result<String, mongodb::error::Error> {
    let db_name = dotenv::var("DB_NAME").unwrap();
    
    let result = client.database(&db_name).run_command(doc! {
        "hostInfo": 1
    }, SelectionCriteria::ReadPreference(mongodb::options::ReadPreference::Primary)).await;

    match result {
        Ok(document) => {
            Ok(document.to_string())
        },
        Err(err) => {
            Err(err)
        }
    }
}

pub fn validate_unique_username(user: &String, db_client: &SyncClient) -> Result<(), ValidationError> {
    let filter = doc! {"user": user};
    let db_name = dotenv::var("DB_NAME").unwrap();
    let db = db_client.database(&db_name);
    let collection = db.collection::<AccountModel>("users");

    let cursor = collection.find_one(filter, None);


    match cursor {
        Err(err) => {
            log::error!("MONGODB::Client::ERROR: {:?}", err);
            Err(ValidationError::new("InternalServerErrorOnValidation"))
        }
        Ok(result) => {
            match result {
                None => {
                    return Ok(());
                }
                Some(model) => {
                    return Err(ValidationError::new("InvalidUsername"));
                }
            }
        }
    }
    
}

//MongoDB - guardar datos del usuario en la db
pub async fn new_user(client: AsyncClient, model: AccountModel) -> Result<InsertOneResult, mongodb::error::Error> {
    let db_name = dotenv::var("DB_NAME").unwrap();
    let db = client.database(&db_name);
    let collection = db.collection::<AccountModel>("users");
    
    collection.insert_one(model, None).await
}
use mongodb::{Client, bson::doc, options::SelectionCriteria};
use redis::RedisError;
use redis_async_pool::{RedisConnection, RedisPool, RedisConnectionManager, deadpool::managed::Pool};
use dotenv;

// Inicializar DBClients o Conectar -----------------------
//Inicializa el cliente de MongoDB
pub async fn init_mongodbcli() -> mongodb::Client {
    log::info!("Inicializando el cliente de MongoDB... 👀");
    
    let uri = dotenv::var("MONGO_DB").unwrap();

    let conection = Client::with_uri_str(uri).await;
    
    match conection {
        Ok(cliente_conectado) => {
            log::info!("¡¡Cliente de MongoDB inicializado!! ✅");
            cliente_conectado
        }
        Err(err) => {
            log::error!("No se pudo iniciar el cliente de MongoDB ❌");
            log::error!("MongoDB_init_error:: {}", err.to_string());
            std::process::exit(1);
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
pub async fn mongodb_status(client: Client) -> Result<String, mongodb::error::Error> {
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
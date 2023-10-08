use std::sync::{Arc, atomic::AtomicUsize, Mutex};
use serde::Serialize;
use mongodb::Client;

// ESTADO --------------------------------------------------
// Estructura que manejara la informacion del estado de la aplicación
pub struct AppState {
    pub app_name: String,
    pub app_desc: String,
    pub n_connections: Arc<AtomicUsize>,
    pub n_connections_errors: Arc<AtomicUsize>,
    pub n_requests_recibed: Mutex<usize>,
    pub n_requests_errrors: Mutex<usize>,
    pub mongodb_client: Client,
}

// Health Info --------------------------------------------------
// Serializa la informacion proporcionada por el estado de la App
#[derive(Serialize)]
pub struct StatusInfo {
    pub nombre: String,
    pub descripcion: String,
    pub numero_conexiones: usize,
    pub numero_errores_de_conexion: usize,
    pub numero_peticiones: usize,
    pub numero_peticiones_con_errores: usize,
    pub db_estado: String
}
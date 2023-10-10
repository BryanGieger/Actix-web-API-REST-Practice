use serde::{Serialize, Deserialize};

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

// Auth ---------------------------------------------------------

//Estructura de los datos de la sesion
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionData {
    id: uuid::Uuid,
    subject: String,
}

//Maneja los datos entrantes de la peticion de registro de un usuario
#[derive(Deserialize)]
struct SignUpPayload {
    login: String,
    password: String,
    password_confirmation: String,
}

//Maneja los datos entrantes de la peticion de inicio de sesion de un usuario
#[derive(Deserialize)]
struct SignInPayload {
    login: String,
    password: String,
}


use serde::{Serialize, Deserialize};
use sanitizer::prelude::*;
use validator::Validate;

use crate::auth::RoleTypes;
use crate::db;

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

// Estructura de los datos de la sesion
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionData {
    pub id: uuid::Uuid,
    pub subject: String,
}

// Maneja los datos entrantes de la peticion de registro de un usuario
#[derive(Sanitize, Clone, Validate, Deserialize, Debug)]
pub struct SignUpPayload {
    #[serde(rename = "usuario")]
    #[sanitize(trim)]
    #[validate(
        length(min = 2, code = "InvalidMinLenght", message="El usuario debe tener almenos 2 caracteres"),
        custom(function = "db::validate_unique_username", arg = "&'v_a mongodb::sync::Client", message = "El usuario ya existe")
    )]
    pub user: String,

    #[serde(rename = "contraseña")]
    #[sanitize(trim)]
    #[validate(
        length(min = 8, code = "InvalidMinLength", message = "La contraseña debe tener minimo 8 caracteres"), 
        length(max = 18, code = "InvalidMaxLength", message = "La contraseña debe tener maximo 18 caracteres")
    )]
    pub password: String,

    #[serde(rename = "verificacion_contraseña")]
    #[sanitize(trim)]
    #[validate(
        must_match(other = "password", code = "PasswordNotSame", message = "Las contraseñas no coinciden")
    )]
    pub password_confirmation: String,

    #[serde(rename = "rol_asignado")]
    pub role: RoleTypes
}

// Maneja los datos entrantes de la peticion de inicio de sesion de un usuario
#[derive(Sanitize, Validate, Deserialize)]
pub struct SignInPayload {
    #[sanitize(trim, numeric)]
    pub user: String,
    #[sanitize(trim)]
    pub password: String,
}


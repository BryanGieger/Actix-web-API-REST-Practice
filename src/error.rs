use serde::Serialize;
use std::collections::HashMap;
use validator::ValidationError;

// Estructura que manejara los errores personalizados de la aplicacion
#[derive(Serialize)]
pub struct ErrorResponse {
    #[serde(rename = "respuesta")] 
    pub response: String,
    #[serde(rename = "mensaje")] 
    pub message: String,
    #[serde(rename = "detalles")] 
    pub details: TypeError
}

impl ErrorResponse {
    // Prepara los errores de validacion asignandole el tipo de error correcto, dependiendo de el error que arroje la validacion.
    pub fn new_validation_error(validation_errors: HashMap<&str, &Vec<ValidationError>>, mensaje: &str) -> Self {

        let mut validation_error_list = Vec::<ValidationErrorResponse>::new();

        for errors in validation_errors.iter() {
            let mut message = String::new();
            let message_option = errors.1[0].message.clone();

            if message_option.is_none() {
                message = "None".to_string();
            }
            else {
                message = message_option.unwrap().to_string();
            }
            
            validation_error_list.push(ValidationErrorResponse {
                input: errors.0.to_string(),
                code: errors.1[0].code.to_string(),
                message: message
            });

        }

        Self {
            response: "error".to_string(),
            message: mensaje.to_string(),
            details: TypeError::Validation(
                validation_error_list
            )
        }
    }

    pub fn new_db_error(db_errors: mongodb::error::Error, mensaje: &str) -> Self {
        let mut errors_list = String::new();

        log::error!("MONGODB::Client::ERROR: {:?}", db_errors.kind);

        for error in db_errors.labels() {
            errors_list.push_str(&error.as_str());
        }

        Self { 
            response: "error".to_string(),
            message: mensaje.to_string(),
            details: TypeError::Other("Error en la base de datos, por favor contactarse con servicio tecnico.".to_string())
        }
    }

}

// Enumeracion para los distintos tipos de errores que existen
#[derive(Serialize)]
pub enum TypeError {
    #[serde(rename = "errores_validacion")]
    Validation(Vec<ValidationErrorResponse>),
    Other(String)
}

// Estructuras para cada tipo de error ------------------------------

// Estructura para los errores de validación
#[derive(Serialize)]
pub struct ValidationErrorResponse {
    #[serde(rename = "campo")]
    pub input: String,
    #[serde(rename = "codigo")]
    pub code: String,
    #[serde(rename = "mensaje")]
    pub message: String
}
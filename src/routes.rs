use actix_web::{HttpResponseBuilder, HttpResponse};
use actix_web::{get, web};
use mongodb::Client;
use serde_json::json;
use std::borrow::BorrowMut;
use std::sync::atomic::Ordering;

use crate::models::*;
use crate::controllers::*;
use crate::db::*;
use crate::AppState;
//-- APP -------------------------------------------

// Sistema -----------------------------------------

//Toma la informacion del estado, revisa el estado de la base de datos y da un response JSON
#[get("/estado")]
async fn state(data: web::Data<AppState>, mongodbcli: web::Data<mongodb::Client>) -> HttpResponse {
    update_state_number(data.n_requests_recibed.lock().unwrap().borrow_mut());//<- Añade +1 al numero de peticiones
    
    let status_result = mongodb_status(mongodbcli.as_ref().clone()).await;

    match status_result {
        Ok(status) => {
            let body = StatusInfo {
                nombre: data.app_name.clone(),
                descripcion: data.app_desc.clone(),
                numero_conexiones: data.n_connections.load(Ordering::Relaxed),
                numero_errores_de_conexion: data.n_connections_errors.load(Ordering::Relaxed),
                numero_peticiones: *data.n_requests_recibed.lock().unwrap(),
                numero_peticiones_con_errores: *data.n_requests_errrors.lock().unwrap(),
                db_estado: status,
            };
        
            HttpResponse::Ok().json(body)
        }

        Err(err) => {
            update_state_number(data.n_requests_errrors.lock().unwrap().borrow_mut());
            HttpResponse::InternalServerError().json(json!({ 
                "error": true, 
                "error_code": 1, 
                "error_details": String::to_owned(&err.to_string()).as_str(), 
            }))
        }
    }
}



// Configuracion -----------------------------------

//Configura todas las rutas para luego llamarlo en el main.rs
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/app") // <- Genera un prefijo en la ruta: app/{route_path}
        .service(state)
    );
}

//-- TEST ------------------------------------------

// Configuracion -----------------------------------

//Configura todas las rutas de pruebas para luego llamarlo en el main.rs
pub fn config_tests(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app/test") // <- Genera un prefijo en la ruta: app/test/{route_path}
        
        // Se agregan todas las rutas una por una
        
    );
}
use actix_web::{HttpResponse, get, post, web, web::{Data, Json}, rt::task};
use mongodb::sync::{
    Client as SyncClient
};

use mongodb::{
    Client as AsyncClient,
};
use serde_json::json;
use validator::{Validate, ValidateArgs};
use std::borrow::BorrowMut;
use std::sync::atomic::Ordering;
use actix_jwt_session::*;

use crate::{models::*, error::ErrorResponse, db};
use crate::controllers::*;
use crate::db::*;
use crate::AppState;
use crate::auth::{AppClaims, AccountModel, Audience};

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

#[get("/authorized")]
async fn must_be_signed_in(session: Authenticated<AppClaims>) -> HttpResponse {
    use actix_jwt_session::Claims;
    let jit = session.jti().to_string();
    log::info!("AwA::log {}", jit);
    HttpResponse::Ok().finish()
}

#[get("/maybe-authorized")]
async fn may_be_signed_in(session: MaybeAuthenticated<AppClaims>) -> HttpResponse {
    if let Some(session) = session.into_option() {
    }
    HttpResponse::Ok().finish()
}

// Ruta para el registro de nuevos usuarios
#[post("/usuarios/registrar_usuario")]
async fn register(payload: Json<SignUpPayload>, db_async_client: web::Data<AsyncClient>, db_sync_client: web::Data<SyncClient>) -> HttpResponse {
    let payload_clone = payload.clone();

    // Validar los datos ingresados
    let validation_result: Result<(), validator::ValidationErrors> = task::spawn_blocking(move || {
        let payload_clone = payload_clone.clone();
        payload_clone.validate_args(db_sync_client.as_ref())
    }).await.unwrap();

    match validation_result {
        Err(err) => {
            // Enviamos un error de validacion en caso de que algo este mal

            let lista_errores = err.field_errors();

            let response = ErrorResponse::new_validation_error(
                lista_errores, 
                "Error al crear el usuario, error en la validacion de los campos."
            );

            HttpResponse::Forbidden().json(response)
        },
        Ok(_) => {
            let payload = payload.into_inner();

            // Si paso la validacion creamos el modelo y lo almacenamos en la db
            let model = AccountModel {
                uuid: Uuid::new_v4(),
                user: payload.user,
                // Encrypt password before saving to database
                pass_hash: Hashing::encrypt(&payload.password).unwrap(),
                role: payload.role
            };
            
            // Guardar el modelo en la base de datos
            match db::new_user(db_async_client.as_ref().clone(), model).await {
                Err(err) => {
                    // Si falla algo en el proceso de guardado en la base de datos, se envia un error
                    let response = ErrorResponse::new_db_error(
                        err,
                        "Error al crear el usuario."
                    );

                    HttpResponse::InternalServerError().json(response)
                },
                Ok(_) => {
                    // Retornamos la HttpResponse en caso de que todo haya ido bien.
                    HttpResponse::Ok().json(json!({
                        "respuesta": "correcto",
                        "mensaje": "¡Usuario creado correctamente!"
                    }))
                }
            }
        }
    }


    
}

/*#[post("/session/sign-in")]
async fn sign_in(
    store: Data<SessionStorage>,
    payload: Json<SignInPayload>,
    jwt_ttl: Data<JwtTtl>,
    refresh_ttl: Data<RefreshTtl>,
) -> Result<HttpResponse, actix_web::Error> {
    let payload = payload.into_inner();
    let store = store.into_inner();
    let account = AccountModel {
        id: Uuid::new_v4(),
        user: payload.user.to_string(),
        pass_hash: payload.password.to_string()
    };

    if let Err(e) = Hashing::verify(account.pass_hash.as_str(), payload.password.as_str()) {
        return Ok(HttpResponse::Unauthorized().finish());
    }
    let claims = AppClaims {
         issues_at: OffsetDateTime::now_utc().unix_timestamp() as usize,
         subject: account.user.clone(),
         expiration_time: jwt_ttl.0.as_seconds_f64() as u64,
         audience: Audience::Web,
         jwt_id: uuid::Uuid::new_v4(),
         account_id: account.id,
         not_before: 0,
    };
    let pair = store
        .clone()
        .store(claims, *jwt_ttl.into_inner(), *refresh_ttl.into_inner())
        .await
        .unwrap();
    Ok(HttpResponse::Ok()
        .append_header((JWT_HEADER_NAME, pair.jwt.encode().unwrap()))
        .append_header((REFRESH_HEADER_NAME, pair.refresh.encode().unwrap()))
        .finish())
}
*/

#[post("/session/sign-out")]
async fn sign_out(store: Data<SessionStorage>, auth: Authenticated<AppClaims>) -> HttpResponse {
    let store = store.into_inner();
    store.erase::<AppClaims>(auth.jwt_id).await.unwrap();
    HttpResponse::Ok()
        .append_header((JWT_HEADER_NAME, ""))
        .append_header((REFRESH_HEADER_NAME, ""))
        .cookie(
            actix_web::cookie::Cookie::build(JWT_COOKIE_NAME, "")
                .expires(OffsetDateTime::now_utc())
                .finish(),
        )
        .cookie(
            actix_web::cookie::Cookie::build(REFRESH_COOKIE_NAME, "")
                .expires(OffsetDateTime::now_utc())
                .finish(),
        )
        .finish()
}

#[get("/session/info")]
async fn session_info(auth: Authenticated<AppClaims>) -> HttpResponse {
    HttpResponse::Ok().json(&*auth)
}

#[get("/session/refresh")]
async fn refresh_session(
    refresh_token: Authenticated<RefreshToken>,
    storage: Data<SessionStorage>,
) -> HttpResponse {
    let s = storage.into_inner();
    let pair = match s.refresh::<AppClaims>(refresh_token.access_jti()).await {
        Err(e) => {
            log::warn!("Failed to refresh token: {e}");
            return HttpResponse::BadRequest().finish();
        }
        Ok(pair) => pair,
    };

    let encrypted_jwt = match pair.jwt.encode() {
        Ok(text) => text,
        Err(e) => {
            log::warn!("Failed to encode claims: {e}");
            return HttpResponse::InternalServerError().finish();
        }
    };
    let encrypted_refresh = match pair.refresh.encode() {
        Err(e) => {
            log::warn!("Failed to encode claims: {e}");
            return HttpResponse::InternalServerError().finish();
        }
        Ok(text) => text,
    };
    HttpResponse::Ok()
        .append_header((
            actix_jwt_session::JWT_HEADER_NAME,
            format!("Bearer {encrypted_jwt}").as_str(),
        ))
        .append_header((
            actix_jwt_session::REFRESH_HEADER_NAME,
            format!("Bearer {}", encrypted_refresh).as_str(),
        ))
        .append_header((
            "ACX-JWT-TTL",
            (pair.refresh.issues_at + pair.refresh.refresh_ttl.0).to_string(),
        ))
        .finish()
}


// Configuracion -----------------------------------

//Configura todas las rutas para luego llamarlo en el main.rs
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/app") // <- Genera un prefijo en la ruta: app/{route_path}
        .service(state)
        .service(must_be_signed_in)
        .service(may_be_signed_in)
        .service(session_info)
        .service(register)
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
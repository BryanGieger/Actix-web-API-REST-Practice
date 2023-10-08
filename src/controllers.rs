use std::sync::{MutexGuard, Mutex};
use actix_web::web;

use crate::models;
// Estado de la APP --------------------------------------

//Aumenta el numero de consultas en el estado (+1) cada llamada a la funcion.
pub fn update_state_number(counter: &mut usize) {
    *counter += 1;
}
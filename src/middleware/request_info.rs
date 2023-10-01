use actix_web::web;

use crate::AppState;

pub fn count_increase(data: web::Data<AppState>) {
    let mut request_count = data.requests_recibed.lock().unwrap();
    *request_count += 1;
}
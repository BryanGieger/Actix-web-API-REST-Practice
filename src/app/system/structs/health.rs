use serde::Serialize;
// Serializa la informacion proporcionada por el estado de la App
#[derive(Serialize)]
pub struct HealthInfo {
    pub app_name: String,
    pub connections_number: usize,
    pub total_request_recibed: usize,
    pub is_alive: bool,
}
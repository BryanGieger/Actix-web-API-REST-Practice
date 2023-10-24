use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Audience {
    Web,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub struct AppClaims {
    #[serde(rename = "exp")]
    pub expiration_time: u64,
    #[serde(rename = "iat")]
    pub issues_at: usize,
    /// Account login
    #[serde(rename = "sub")]
    pub subject: String,
    #[serde(rename = "aud")]
    pub audience: Audience,
    #[serde(rename = "jti")]
    pub jwt_id: uuid::Uuid,
    #[serde(rename = "aci")]
    pub account_id: uuid::Uuid,
    #[serde(rename = "nbf")]
    pub not_before: u64,
}

impl actix_jwt_session::Claims for AppClaims {
    fn jti(&self) -> uuid::Uuid {
        self.jwt_id
    }

    fn subject(&self) -> &str {
        &self.subject
    }
}

impl AppClaims {
    pub fn account_id(&self) -> uuid::Uuid {
        self.account_id
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountModel {
    pub uuid: uuid::Uuid,
    pub user: String,
    pub pass_hash: String,
    pub role: RoleTypes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoleTypes {
    Admin,
    User
}
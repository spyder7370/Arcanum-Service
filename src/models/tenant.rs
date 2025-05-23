use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub image: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct TenantRequest {
    pub name: String,
    pub slug: String,
    pub image: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenantDescription {
    pub redirection_url: String,
}

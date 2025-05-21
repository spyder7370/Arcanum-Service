use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub image: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct TenantRequest {
    pub name: String,
    pub image: String,
    pub description: String,
}

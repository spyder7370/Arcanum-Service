use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::models::tenant::Tenant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub image: String,
    pub guide_available: bool,
    pub tenant: Arc<Tenant>
}
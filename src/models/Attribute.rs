use crate::models::tenant::Tenant;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub id: String,
    pub name: String,
    pub parent: Option<Arc<Attribute>>,
    pub children: Vec<Arc<Attribute>>,
    pub tenant: Arc<Tenant>,
}

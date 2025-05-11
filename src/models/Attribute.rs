use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::models::tenet::Tenet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub id: String,
    pub name: String,
    pub parent: Option<Arc<Attribute>>,
    pub children: Vec<Arc<Attribute>>,
    pub tenet: Arc<Tenet>,
}

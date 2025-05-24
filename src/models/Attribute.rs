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

// create table if not exists attributes (
// id text primary key,
// tenet_id text references tenet(id),
// name text not null unique,
// parent_attribute_id text,
// preference integer,
// foreign key (parent_attribute_id) references attributes(id)
// id);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddAttributeRequest {
   pub tenet_id: String,
    pub name: String,
    pub parent_attribute_id: Option<String>,
    pub preference: Option<i32>,
}
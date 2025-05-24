use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::models::tenet::Tenet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub image: String,
    pub guide_available: bool,
    // pub tenet: Arc<Tenet>
}

#[derive(Debug, Deserialize)]
pub struct GetAllCharactersRequest {
    pub tenet_id: String,
}

#[derive(Debug,Deserialize)]
pub struct AddCharacterRequest {
    pub tenet_id: String,
    pub name: String,
    pub description: Option<String>,
    pub image: String,
    pub guide_available: Option<bool>,
    
}

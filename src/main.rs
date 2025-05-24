mod models;
mod routes;
mod constants;
mod handlers;

use std::time::Duration;
use tower::ServiceBuilder;
use axum::{Json, Router, routing::get};
use axum::error_handling::HandleErrorLayer;
use axum::routing::post;
use serde::Serialize;
use sqlx::sqlite::SqlitePoolOptions;
use routes::health;
use crate::handlers::error_handlers::handle_json_error;
use crate::routes::character::{add_character, get_game_character};
use crate::routes::attribute::{add_attribute};
use crate::routes::health::health;
use crate::routes::tenet::{add_tenet, get_tenets};

#[tokio::main]
async fn main() {

    // Initialize tracing
    tracing_subscriber::fmt::init();

    let pool = SqlitePoolOptions::new().max_connections(16)
        .connect("sqlite:Arcanum.sqlite")
        .await.unwrap();
    
    let db_routes = Router::new()
        .route("/tenet",post(add_tenet))
        .route("/tenet",get(get_tenets))
        .route("/add_character",post(add_character))
        .route("/list_characters",get(get_game_character))
        .route("/add_attribute",post(add_attribute))
        .with_state(pool);
    
    let app = Router::new().route("/health", get(health)).nest("/api/v1",db_routes);
    println!("Running on http://localhost:8080");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct GenericResponse {
    message: String,
}


mod models;
mod routes;
mod utils;

use crate::routes::health::health;
use crate::routes::tenant::{add_tenant, delete_tenant, edit_tenant, get_tenants};
use axum::http::{Method, header};
use axum::middleware;
use axum::routing::{delete, patch, post};
use axum::{Router, routing::get};
use sqlx::sqlite::SqlitePoolOptions;
use tower_http::cors::CorsLayer;
use utils::middleware::auth_validator;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let pool = SqlitePoolOptions::new()
        .max_connections(16)
        .connect("sqlite:Arcanum.sqlite")
        .await
        .unwrap();

    let tenant_routes = Router::new()
        .route("/api/v1/tenants", post(add_tenant))
        .route("/api/v1/tenants", get(get_tenants))
        .route("/api/v1/tenants/:id", patch(edit_tenant))
        .route("/api/v1/tenants/:id", delete(delete_tenant))
        .with_state(pool);

    let cors = CorsLayer::new()
        .allow_origin(axum::http::HeaderValue::from_static(
            "http://localhost:5173",
        ))
        .allow_methods([Method::GET])
        .allow_headers([header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/health", get(health))
        .merge(tenant_routes)
        .layer(middleware::from_fn(auth_validator))
        .layer(cors);
    println!("Running on http://localhost:8080");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

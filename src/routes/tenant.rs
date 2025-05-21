use crate::models::json_response::JsonResultResponse;
use crate::models::tenant::{Tenant, TenantRequest};
use axum::Json;
use axum::extract::State;
use axum::headers::Te;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn add_tenant(
    State(pool): State<SqlitePool>,
    Json(tenant_request): Json<TenantRequest>,
) -> impl IntoResponse {
    let tenant_id = Uuid::new_v4().to_string();
    let result = sqlx::query!(
        r#"insert into tenant (id, name, image,description) values (?, ?, ?, ?)"#,
        tenant_id,
        tenant_request.name,
        tenant_request.image,
        tenant_request.description
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => {
            let tenant = Tenant {
                id: tenant_id,
                name: tenant_request.name,
                image: tenant_request.image,
                description: tenant_request.description,
            };
            (StatusCode::CREATED, Json(tenant)).into_response()
        }
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error":err.to_string()})),
        )
            .into_response(),
    }
}

pub async fn get_tenants(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let tenants = sqlx::query!(r#"select id,name,image,description from tenant"#);
    let tenants = tenants.fetch_all(&pool).await;
    let response = match tenants {
        Ok(tenants) => {
            let mut result: Vec<Tenant> = Vec::new();
            for tenant in tenants {
                let t = Tenant {
                    id: tenant.id.unwrap(),
                    name: tenant.name.to_string(),
                    image: tenant.image.to_string(),
                    description: tenant.description.to_string(),
                };
                result.push(t);
            }
            JsonResultResponse {
                status: "SUCCESS".to_string(),
                result: Option::from(result),
                error: None,
            }
        }
        Err(err) => JsonResultResponse {
            status: "ERROR".to_string(),
            result: None,
            error: Some(err.to_string()),
        },
    };
    Json(response)
    // JsonResultResponse<>
}

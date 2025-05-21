use crate::models::tenant::{Tenant, TenantRequest};
use crate::utils::response_utils::{generate_error, generate_response};
use axum::Json;
use axum::extract::State;
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

    let response = match result {
        Ok(_) => {
            let tenant = Tenant {
                id: tenant_id,
                name: tenant_request.name,
                image: tenant_request.image,
                description: tenant_request.description,
            };
            generate_response(tenant, Some(201)).await
        }
        Err(err) => generate_error(Some(err.to_string()), None).await,
    };
    response
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
            generate_response(result, None).await
        }
        Err(err) => generate_error(Some(err.to_string()), None).await,
    };
    response
}

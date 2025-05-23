use crate::models::tenant::{Tenant, TenantDescription, TenantRequest};
use crate::utils::json::to_json_string;
use crate::utils::response_utils::{generate_error, generate_response};
use axum::Json;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn add_tenant(
    State(pool): State<SqlitePool>,
    Json(tenant_request): Json<TenantRequest>,
) -> impl IntoResponse {
    let tenant_id = Uuid::new_v4().to_string();
    let tenant_description = TenantDescription {
        redirection_url: format!("/titles/{}", tenant_request.slug),
    };
    let description_json = to_json_string(&tenant_description);

    let result = sqlx::query!(
        r#"insert into tenant (id, name, slug, image, description) values (?, ?, ?, ?, ?)"#,
        tenant_id,
        tenant_request.name,
        tenant_request.slug,
        tenant_request.image,
        description_json
    )
    .execute(&pool)
    .await;

    let response = match result {
        Ok(_) => {
            let tenant = Tenant {
                id: tenant_id,
                name: tenant_request.name,
                slug: tenant_request.slug,
                image: tenant_request.image,
                description: description_json,
            };
            generate_response(tenant, Some(201)).await
        }
        Err(err) => generate_error(Some(err.to_string()), None).await,
    };
    response
}

pub async fn get_tenants(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let tenants =
        sqlx::query!(r#"select id,name,slug,image,description from tenant order by name"#);
    let tenants = tenants.fetch_all(&pool).await;
    let response = match tenants {
        Ok(tenants) => {
            let mut result: Vec<Tenant> = Vec::new();
            for tenant in tenants {
                let t = Tenant {
                    id: tenant.id.unwrap(),
                    name: tenant.name.to_string(),
                    slug: tenant.slug.to_string(),
                    image: tenant.image.to_string(),
                    description: tenant.description.unwrap(),
                };
                result.push(t);
            }
            generate_response(result, None).await
        }
        Err(err) => generate_error(Some(err.to_string()), None).await,
    };
    response
}

pub async fn edit_tenant(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
    Json(tenant_request): Json<TenantRequest>,
) -> impl IntoResponse {
    let name = tenant_request.name;
    let slug = tenant_request.slug;
    let image = tenant_request.image;
    let tenant_description = TenantDescription {
        redirection_url: format!("/titles/{}", slug.to_string()),
    };
    let description = to_json_string(&tenant_description);

    let result = sqlx::query!(
        r#"UPDATE tenant SET name = ?, slug = ?, image = ?, description = ? WHERE id = ?"#,
        name,
        slug,
        image,
        description,
        id
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => generate_response(format!("Tenant {} updated", id), Some(200)).await,
        Err(err) => generate_error(Some(err.to_string()), None).await,
    }
}

pub async fn delete_tenant(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    let result = sqlx::query!(r#"DELETE FROM tenant WHERE id = ?"#, id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => generate_response(format!("Tenant {} deleted", id), Some(200)).await,
        Err(err) => generate_error(Some(err.to_string()), None).await,
    }
}

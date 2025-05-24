use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::SqlitePool;
use crate::models::Attribute::AddAttributeRequest;
use crate::models::character::AddCharacterRequest;
use crate::models::json_response::JsonResultResponse;

pub async fn add_attribute(State(pool): State<SqlitePool>, Json(attribute): Json<AddAttributeRequest>) -> impl IntoResponse {
    let attribute_id = uuid::Uuid::new_v4().to_string();
    let result = sqlx::query!(r#"insert into attributes (id,tenet_id, name, parent_attribute_id, preference) values (?, ?, ?, ?, ?)"#, attribute_id,attribute.tenet_id,attribute.name, attribute.parent_attribute_id, attribute.preference).execute(&pool).await;
    let response = match result {
        Ok(_) => {
            JsonResultResponse::<String> {
                status: "SUCCESS".to_string(),
                result: None,
                error: None,
                status_code: StatusCode::OK,
            }
        }, Err(err)=>{
            JsonResultResponse {
                status: "ERROR".to_string(),
                result: None,
                error: Some(err.to_string()),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            }
        }
    };
    Json(response)
}

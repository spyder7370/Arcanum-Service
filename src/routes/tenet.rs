use axum::extract::State;
use axum::headers::Te;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::models::json_response::JsonResultResponse;
use crate::models::tenet::{Tenet, TenetRequest};


pub async fn add_tenet(State(pool): State<SqlitePool>, Json(tenet_request): Json<TenetRequest>) -> impl IntoResponse {
    let tenet_id = Uuid::new_v4().to_string();
    let result = sqlx::query!(r#"insert into tenet (id, name, image,description) values (?, ?, ?, ?)"#,
    tenet_id,tenet_request.name,tenet_request.image,tenet_request.description).execute(&pool).await;
    
    match result { 
        Ok(_) => {
            let tenet =Tenet {
                id: tenet_id,
                name: tenet_request.name,
                image: tenet_request.image,
                description: tenet_request.description,
            };
            (StatusCode::CREATED, Json(tenet)).into_response()
        },
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error":err.to_string()}))).into_response()
        }
    }
}

pub async fn get_tenets(State(pool):State<SqlitePool>) -> impl IntoResponse {
    let tenets = sqlx::query!(r#"select id,name,image,description from tenet"#);
    let tenets = tenets.fetch_all(&pool).await;
    let response = match tenets {
        Ok(tenets) =>{
            let mut result:Vec<Tenet> = Vec::new();
            for tenet in tenets {
                let t = Tenet{
                    id: tenet.id.unwrap(),
                    name: tenet.name.to_string(),
                    image: tenet.image.to_string(),
                    description: tenet.description.to_string(),
                };
                result.push(t);
            }
            JsonResultResponse{
                status:"SUCCESS".to_string(),
                result: Option::from(result),
                error: None
            }
        },
        Err(err) => {
            JsonResultResponse{
                status:"ERROR".to_string(),
                result: None,
                error: Some(err.to_string())
            }
        }
    }; 
    Json(response)
    // JsonResultResponse<>
}
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use crate::models::json_response::JsonResultResponse;

pub async fn handle_json_error(err: JsonRejection) -> (StatusCode, Json<JsonResultResponse<()>>) {
    let error_message = err.to_string();
    let status_code = match err {
        JsonRejection::JsonDataError(_) => StatusCode::BAD_REQUEST,
        JsonRejection::JsonSyntaxError(_) => StatusCode::BAD_REQUEST,
        JsonRejection::MissingJsonContentType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };

    let response = JsonResultResponse::<()>::error(error_message, status_code);
    (status_code, Json(response))
}

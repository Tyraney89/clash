use actix_web::{get, web, HttpResponse, Result};
use crate::models::ApiError;
use crate::services::PlayerService;

#[get("/api/player/{player_tag}")]
pub async fn get_player(
    player_tag: web::Path<String>,
    player_service: web::Data<PlayerService>,
) -> Result<HttpResponse> {
    match player_service.get_player(&player_tag).await {
        Ok(player_data) => Ok(HttpResponse::Ok().json(player_data)),
        Err(ApiError::ClashApiError { reason, message }) => {
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "reason": reason,
                "message": message
            })))
        }
        Err(ApiError::InvalidPlayerTag(message)) => {
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid player tag",
                "message": message
            })))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to fetch player data",
            "message": e.to_string()
        }))),
    }
}

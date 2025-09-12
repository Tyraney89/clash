use crate::config::ClashApiSettings;
use crate::models::{ApiError, Player, ApiErrorResponse};
use reqwest::Client;
use urlencoding::encode;

#[derive(Clone)]
pub struct ClashApiService {
    client: Client,
    settings: ClashApiSettings,
}

impl ClashApiService {
    pub fn new(settings: ClashApiSettings) -> Self {
        Self {
            client: Client::new(),
            settings,
        }
    }

    pub async fn get_player(&self, player_tag: &str) -> Result<Player, ApiError> {
        // Validate and format player tag
        let formatted_tag = if player_tag.starts_with('#') {
            player_tag.to_string()
        } else {
            format!("#{}", player_tag)
        };

        // URL encode the tag for the API call
        let encoded_tag = encode(&formatted_tag);
        let url = format!("{}/players/{}", self.settings.base_url, encoded_tag);

        // Make the API request
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.settings.token))
            .send()
            .await?;

        if response.status().is_success() {
            let player_data: Player = response.json().await?;
            Ok(player_data)
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            let api_error: ApiErrorResponse = serde_json::from_str(&error_text)
                .unwrap_or_else(|_| ApiErrorResponse {
                    reason: "unknown".to_string(),
                    message: error_text,
                });
            
            Err(ApiError::ClashApiError {
                reason: api_error.reason,
                message: api_error.message,
            })
        }
    }
}

use crate::config::ClashApiSettings;
use crate::models::{ApiError, Player, ApiErrorResponse};
use reqwest::Client;
use urlencoding::encode;

/// Repository layer responsible for data access to the Clash Royale API
/// This layer handles the low-level details of making HTTP requests
#[derive(Clone)]
pub struct ClashApiRepository {
    client: Client,
    settings: ClashApiSettings,
}

impl ClashApiRepository {
    pub fn new(settings: ClashApiSettings) -> Self {
        Self {
            client: Client::new(),
            settings,
        }
    }

    /// Fetches raw player data from the Clash Royale API
    /// This is a pure data access method - no business logic
    pub async fn fetch_player_data(&self, player_tag: &str) -> Result<Player, ApiError> {
        // Format player tag for API call
        let formatted_tag = self.format_player_tag(player_tag);
        let encoded_tag = encode(&formatted_tag);
        let url = format!("{}/players/{}", self.settings.base_url, encoded_tag);

        // Make the HTTP request
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

    /// Helper method to format player tags consistently
    fn format_player_tag(&self, player_tag: &str) -> String {
        if player_tag.starts_with('#') {
            player_tag.to_string()
        } else {
            format!("#{}", player_tag)
        }
    }
}

use crate::models::{ApiError, Player};
use crate::repositories::ClashApiRepository;

/// Service layer responsible for business logic
/// This layer contains the application's business rules and orchestrates data access
#[derive(Clone)]
pub struct PlayerService {
    clash_repository: ClashApiRepository,
}

impl PlayerService {
    pub fn new(clash_repository: ClashApiRepository) -> Self {
        Self { clash_repository }
    }

    /// Gets player data with business logic applied
    /// This is where you'd add caching, validation, data transformation, etc.
    pub async fn get_player(&self, player_tag: &str) -> Result<Player, ApiError> {
        // Business logic: Validate player tag format
        self.validate_player_tag(player_tag)?;

        // Business logic: Could add caching here
        // let cached_player = self.cache.get(player_tag).await?;
        // if let Some(player) = cached_player {
        //     return Ok(player);
        // }

        // Delegate data access to repository
        let player = self.clash_repository.fetch_player_data(player_tag).await?;

        // Business logic: Could add data transformation here
        // let enriched_player = self.enrich_player_data(player).await?;

        // Business logic: Could add caching here
        // self.cache.set(player_tag, &player).await?;

        Ok(player)
    }

    /// Business logic: Validate player tag
    fn validate_player_tag(&self, player_tag: &str) -> Result<(), ApiError> {
        if player_tag.is_empty() {
            return Err(ApiError::InvalidPlayerTag("Player tag cannot be empty".to_string()));
        }

        if player_tag.len() < 3 {
            return Err(ApiError::InvalidPlayerTag("Player tag too short".to_string()));
        }

        // Could add more validation rules here
        // - Check for invalid characters
        // - Check format patterns
        // - etc.

        Ok(())
    }

    /// Business logic: Enrich player data (example of what you might do)
    async fn enrich_player_data(&self, mut player: Player) -> Result<Player, ApiError> {
        // Example business logic:
        // - Calculate win rate
        // - Add computed fields
        // - Apply business rules
        // - etc.

        // For now, just return the player as-is
        Ok(player)
    }
}

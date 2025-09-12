use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Player data structures
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub tag: String,
    pub name: String,
    #[serde(rename = "expLevel")]
    pub exp_level: u32,
    pub trophies: u32,
    #[serde(rename = "bestTrophies")]
    pub best_trophies: u32,
    pub wins: u32,
    pub losses: u32,
    #[serde(rename = "battleCount")]
    pub battle_count: u32,
    #[serde(rename = "threeCrownWins")]
    pub three_crown_wins: u32,
    #[serde(rename = "challengeCardsWon")]
    pub challenge_cards_won: u32,
    #[serde(rename = "challengeMaxWins")]
    pub challenge_max_wins: u32,
    #[serde(rename = "tournamentCardsWon")]
    pub tournament_cards_won: u32,
    #[serde(rename = "tournamentBattleCount")]
    pub tournament_battle_count: u32,
    pub role: Option<String>,
    pub donations: u32,
    #[serde(rename = "donationsReceived")]
    pub donations_received: u32,
    #[serde(rename = "totalDonations")]
    pub total_donations: u32,
    #[serde(rename = "warDayWins")]
    pub war_day_wins: u32,
    #[serde(rename = "clanCardsCollected")]
    pub clan_cards_collected: u32,
    pub clan: Option<Clan>,
    pub arena: Arena,
    #[serde(rename = "leagueStatistics")]
    pub league_statistics: Option<LeagueStatistics>,
    pub badges: Vec<Badge>,
    pub achievements: Vec<Achievement>,
    pub cards: Vec<Card>,
    #[serde(rename = "supportCards")]
    pub support_cards: Vec<Card>,
    #[serde(rename = "currentDeck")]
    pub current_deck: Vec<Card>,
    #[serde(rename = "currentDeckSupportCards")]
    pub current_deck_support_cards: Vec<Card>,
    #[serde(rename = "currentFavouriteCard")]
    pub current_favourite_card: Option<Card>,
    #[serde(rename = "starPoints")]
    pub star_points: u32,
    #[serde(rename = "expPoints")]
    pub exp_points: u32,
    #[serde(rename = "legacyTrophyRoadHighScore")]
    pub legacy_trophy_road_high_score: u32,
    #[serde(rename = "currentPathOfLegendSeasonResult")]
    pub current_path_of_legend_season_result: Option<PathOfLegendSeasonResult>,
    #[serde(rename = "lastPathOfLegendSeasonResult")]
    pub last_path_of_legend_season_result: Option<PathOfLegendSeasonResult>,
    #[serde(rename = "bestPathOfLegendSeasonResult")]
    pub best_path_of_legend_season_result: Option<PathOfLegendSeasonResult>,
    pub progress: HashMap<String, ProgressData>,
    #[serde(rename = "totalExpPoints")]
    pub total_exp_points: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Clan {
    pub tag: String,
    pub name: String,
    #[serde(rename = "badgeId")]
    pub badge_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Arena {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeagueStatistics {
    #[serde(rename = "currentSeason")]
    pub current_season: Season,
    #[serde(rename = "previousSeason")]
    pub previous_season: Option<Season>,
    #[serde(rename = "bestSeason")]
    pub best_season: Option<Season>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Season {
    pub id: Option<String>,
    pub trophies: u32,
    #[serde(rename = "bestTrophies")]
    pub best_trophies: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Badge {
    pub name: String,
    pub level: u32,
    #[serde(rename = "maxLevel")]
    pub max_level: u32,
    pub progress: u32,
    pub target: u32,
    #[serde(rename = "iconUrls")]
    pub icon_urls: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Achievement {
    pub name: String,
    pub stars: u32,
    pub value: u32,
    pub target: u32,
    pub info: String,
    #[serde(rename = "completionInfo")]
    pub completion_info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    pub id: u32,
    pub level: Option<u32>,
    #[serde(rename = "evolutionLevel")]
    pub evolution_level: Option<u32>,
    #[serde(rename = "maxLevel")]
    pub max_level: Option<u32>,
    #[serde(rename = "maxEvolutionLevel")]
    pub max_evolution_level: Option<u32>,
    pub rarity: Option<String>,
    pub count: Option<u32>,
    #[serde(rename = "elixirCost")]
    pub elixir_cost: Option<u32>,
    #[serde(rename = "iconUrls")]
    pub icon_urls: Option<HashMap<String, String>>,
    #[serde(rename = "starLevel")]
    pub star_level: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PathOfLegendSeasonResult {
    #[serde(rename = "leagueNumber")]
    pub league_number: u32,
    pub trophies: u32,
    pub rank: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgressData {
    pub arena: Arena,
    pub trophies: u32,
    #[serde(rename = "bestTrophies")]
    pub best_trophies: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    pub reason: String,
    pub message: String,
}

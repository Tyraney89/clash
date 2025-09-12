use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Player data structures
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub tag: String,
    pub name: String,
    pub exp_level: u32,
    pub trophies: u32,
    pub best_trophies: u32,
    pub wins: u32,
    pub losses: u32,
    pub battle_count: u32,
    pub three_crown_wins: u32,
    pub win_streak: u32,
    pub best_win_streak: u32,
    pub total_donations: u32,
    pub challenge_cards_won: u32,
    pub challenge_max_wins: u32,
    pub tournament_cards_won: u32,
    pub tournament_battle_count: u32,
    pub role: Option<String>,
    pub donations: u32,
    pub donations_received: u32,
    pub war_day_wins: u32,
    pub clan_cards_collected: u32,
    pub clan: Option<Clan>,
    pub arena: Arena,
    pub league_statistics: Option<LeagueStatistics>,
    pub badges: Vec<Badge>,
    pub achievements: Vec<Achievement>,
    pub cards: Vec<Card>,
    pub current_deck: Vec<Card>,
    pub current_favourite_card: Option<Card>,
    pub star_points: u32,
    pub exp_points: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Clan {
    pub tag: String,
    pub name: String,
    pub badge_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Arena {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LeagueStatistics {
    pub current_season: Season,
    pub previous_season: Option<Season>,
    pub best_season: Option<Season>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Season {
    pub id: String,
    pub trophies: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Badge {
    pub name: String,
    pub level: u32,
    pub max_level: u32,
    pub progress: u32,
    pub target: u32,
    pub icon_urls: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Achievement {
    pub name: String,
    pub stars: u32,
    pub value: u32,
    pub target: u32,
    pub info: String,
    pub completion_info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    pub id: u32,
    pub level: u32,
    pub max_level: u32,
    pub count: u32,
    pub icon_urls: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    pub reason: String,
    pub message: String,
}

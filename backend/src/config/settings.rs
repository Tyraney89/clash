use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
    pub clash_api: ClashApiSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClashApiSettings {
    pub base_url: String,
    pub token: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server: ServerSettings {
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
            clash_api: ClashApiSettings {
                base_url: "https://api.clashroyale.com/v1".to_string(),
                token: "YOUR_API_TOKEN_HERE".to_string(),
            },
        }
    }
}

impl Settings {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();
        
        let settings = Settings {
            server: ServerSettings {
                host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: std::env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()?,
            },
            clash_api: ClashApiSettings {
                base_url: std::env::var("CLASH_API_BASE_URL")
                    .unwrap_or_else(|_| "https://api.clashroyale.com/v1".to_string()),
                token: std::env::var("CLASH_API_TOKEN")
                    .unwrap_or_else(|_| "YOUR_API_TOKEN_HERE".to_string()),
            },
        };
        
        Ok(settings)
    }
}

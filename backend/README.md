# Clash Royale Stats API

A professional Rust backend API for fetching Clash Royale player statistics using the official Clash Royale API.

## Project Structure

```
backend/
├── Cargo.toml              # Dependencies and project metadata
├── src/
│   ├── main.rs             # Application entry point
│   ├── lib.rs              # Library root and public API
│   ├── config/             # Configuration management
│   │   ├── mod.rs
│   │   └── settings.rs     # Settings and environment variables
│   ├── handlers/           # HTTP request handlers
│   │   ├── mod.rs
│   │   ├── health.rs       # Health check endpoint
│   │   └── player.rs       # Player data endpoint
│   ├── models/             # Data structures and types
│   │   ├── mod.rs
│   │   ├── player.rs       # Player-related data structures
│   │   └── error.rs        # Error types and handling
│   └── services/           # Business logic and external API calls
│       ├── mod.rs
│       └── clash_api.rs    # Clash Royale API integration
├── tests/                  # Integration tests
├── .gitignore
└── README.md
```

## Features

- **Modular Architecture**: Clean separation of concerns with dedicated modules
- **Configuration Management**: Environment-based configuration with sensible defaults
- **Error Handling**: Comprehensive error types with proper error propagation
- **Player Data Endpoint**: Get comprehensive player statistics
- **Health Check**: Monitor API status
- **Environment Variables**: Secure configuration management

## Setup

### Prerequisites

- Rust (latest stable version)
- Clash Royale API token from [Supercell Developer Portal](https://developer.clashroyale.com/)

### Installation

1. Navigate to the backend directory:
   ```bash
   cd backend
   ```

2. Create a `.env` file with your configuration:
   ```bash
   # Server Configuration
   SERVER_HOST=127.0.0.1
   SERVER_PORT=8080
   
   # Clash Royale API Configuration
   CLASH_API_BASE_URL=https://api.clashroyale.com/v1
   CLASH_API_TOKEN=your_actual_api_token_here
   ```

3. Build and run:
   ```bash
   cargo run
   ```

The server will start on `http://127.0.0.1:8080`

## API Endpoints

### GET `/`
Returns API information and available endpoints.

### GET `/health`
Health check endpoint with timestamp.

### GET `/api/player/{player_tag}`
Fetches player data from the Clash Royale API.

**Parameters:**
- `player_tag`: Player tag (with or without # prefix)

**Example:**
```bash
curl http://127.0.0.1:8080/api/player/2PP
```

## Architecture Benefits

### Modular Design
- **Handlers**: Handle HTTP requests and responses
- **Services**: Contain business logic and external API calls
- **Models**: Define data structures and error types
- **Config**: Manage application settings and environment variables

### Error Handling
- Custom error types with proper error propagation
- Structured error responses
- Graceful error handling throughout the application

### Configuration
- Environment-based configuration
- Sensible defaults
- Easy to override with environment variables

### Testability
- Each module can be tested independently
- Clear separation of concerns
- Easy to mock dependencies

## Development

### Adding New Endpoints

1. Create a new handler in `handlers/`
2. Add the handler to `handlers/mod.rs`
3. Register the handler in `main.rs`

### Adding New Services

1. Create a new service in `services/`
2. Add the service to `services/mod.rs`
3. Inject the service in `main.rs`

### Adding New Models

1. Create a new model file in `models/`
2. Add the model to `models/mod.rs`
3. Export the model in `lib.rs`

## Dependencies

- **actix-web**: Web framework
- **reqwest**: HTTP client
- **serde**: Serialization/deserialization
- **tokio**: Async runtime
- **chrono**: Date/time handling
- **dotenv**: Environment variable loading
- **anyhow**: Error handling
- **thiserror**: Error derive macros

## Next Steps

- Add database integration
- Implement caching
- Add more Clash Royale endpoints
- Add authentication
- Add rate limiting
- Add comprehensive logging
- Add metrics and monitoring

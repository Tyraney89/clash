# Clash Royale Stats Website

> **Note:** This project is a simple experiment to explore building a full-stack web app using Rust (Actix-Web) for the backend and Next.js with shadcn/ui for the frontend. It is not intended for production use, but as a learning and prototyping exercise.

A full-stack web application focused on Clash Royale player statistics, built with Next.js (frontend) and Rust Actix-Web (backend).

## Project Structure

```
clash/
├── backend/                 # Rust backend API
│   ├── Cargo.toml          # Rust dependencies
│   ├── src/                # Source code
│   │   ├── main.rs         # Application entry point
│   │   ├── lib.rs          # Library root
│   │   ├── config/         # Configuration management
│   │   ├── handlers/       # HTTP request handlers
│   │   ├── models/         # Data structures and types
│   │   ├── services/       # Business logic
│   │   └── tests/          # Integration tests
│   └── README.md           # Backend documentation
├── frontend/               # Next.js frontend
│   ├── package.json        # Node.js dependencies
│   ├── app/                # Next.js app directory
│   ├── components/         # React components
│   ├── lib/                # Utility functions
│   └── README.md           # Frontend documentation
└── README.md               # This file
```

## Features

### Backend (Rust + Actix-Web)
- **Player Statistics API**: Comprehensive player data from Clash Royale API
- **Modular Architecture**: Clean separation of concerns
- **Error Handling**: Structured error responses
- **Configuration Management**: Environment-based settings
- **Health Monitoring**: API status endpoints

### Frontend (Next.js + shadcn/ui)
- **Modern UI**: Built with shadcn/ui components
- **Player Stats Display**: Win/loss ratios, recent battles
- **Responsive Design**: Mobile-first approach
- **TypeScript**: Type-safe development

## Quick Start

### Backend Setup
```bash
cd backend
cargo run
```

### Frontend Setup
```bash
cd frontend
npm install
npm run dev
```

## API Endpoints

- `GET /` - API information
- `GET /health` - Health check
- `GET /api/player/{tag}` - Player statistics

## Development

This project uses a monorepo structure with separate frontend and backend directories. Each has its own dependencies and build process.

### Backend Development
- Language: Rust
- Framework: Actix-Web
- Dependencies: Managed by Cargo

### Frontend Development
- Language: TypeScript
- Framework: Next.js
- UI Library: shadcn/ui
- Dependencies: Managed by npm

## Next Steps

- [ ] Add authentication
- [ ] Implement caching
- [ ] Add more Clash Royale endpoints
- [ ] Database integration
- [ ] Rate limiting
- [ ] Monitoring and logging

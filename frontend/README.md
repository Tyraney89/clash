# Clash Royale Stats Frontend

> **Note:** This frontend is a simple experiment built with [Next.js](https://nextjs.org) and [shadcn/ui](https://ui.shadcn.com/) for modern, composable React UI components. It is not intended for production use, but as a learning and prototyping exercise.

## Getting Started

1. Install dependencies:
   ```bash
   npm install
   ```
2. Run the development server:
   ```bash
   npm run dev
   ```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

## Features
- Built with [Next.js](https://nextjs.org)
- Uses [shadcn/ui](https://ui.shadcn.com/) for UI components
- TypeScript for type safety
- Modern, responsive design
- Connects to the Rust backend API for Clash Royale player stats

## Project Structure
- `app/` — Next.js app directory
- `lib/` — Utility functions
- `globals.css` — Tailwind and shadcn/ui styles
- `components.json` — shadcn/ui configuration

## Notes
- Make sure the Rust backend is running on `localhost:8080` for API calls to work.
- This project is for experimentation and learning purposes only.

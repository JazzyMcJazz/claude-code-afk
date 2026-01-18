# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Claude Code AFK is a push notification system that sends notifications to a smartphone when Claude Code requires user interaction. The system uses Web Push API with a service worker and consists of two main components:

1. **CLI Tool** (`cli/`): Rust command-line tool that integrates with Claude Code hooks
2. **Backend** (`site/`): SvelteKit server with SQLite database that handles device pairing and sends Web Push notifications

## Architecture

The system uses a pairing flow where the CLI generates a QR code, the user scans it on their phone, grants push notification permissions, and the backend stores the push subscription. When Claude Code needs attention, it triggers a hook that calls the CLI, which sends a notification request to the backend, which pushes to the subscribed browser via Web Push API.

**Key Technologies:**
- CLI: Rust (sync, no async runtime), uses `ureq` for HTTP, `confy` for config, `qrcode` for terminal QR codes
- Backend: SvelteKit + TypeScript + Drizzle ORM + SQLite + TailwindCSS + web-push library + Docker
- Backend Adapter: `@sveltejs/adapter-node` for Docker/VPS deployment
- Frontend: Svelte 5 with service worker for push notifications

## Development Commands

### Backend (SvelteKit)

```bash
cd site

# Install dependencies
npm install

# Generate VAPID keys for Web Push (first time only)
npx web-push generate-vapid-keys
# Copy keys to .env file (see site/.env.example)

# Push database schema to SQLite (development only)
npm run db:push

# Start development server (http://localhost:5173)
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview

# Type checking
npm run check

# Linting and formatting
npm run lint
npm run format

# Tests
npm run test        # Run once
npm run test:unit   # Watch mode

# Database management
npm run db:push      # Push schema changes (development only - not for production)
npm run db:generate  # Generate migrations from schema (commit these for production)
npm run db:migrate   # Run migrations (alternative to db:push)
npm run db:studio    # Open Drizzle Studio
```

### Docker Commands

```bash
# Using Docker Compose (from project root)
docker-compose up -d              # Start backend
docker-compose logs -f site       # View logs
docker-compose down               # Stop backend

# Using Docker directly (from site/)
docker build -t claude-afk-site .    # Build image
docker run -d -p 3000:3000 \            # Run container
  -v $(pwd)/data:/data \
  -e VAPID_PUBLIC_KEY="..." \
  -e VAPID_PRIVATE_KEY="..." \
  -e VAPID_SUBJECT="mailto:..." \
  claude-afk-site
```

### Database Migrations & Production Deployment

The project uses Drizzle ORM with SQLite. Database schema changes work differently in development vs production:

**Development:**
- Use `npm run db:push` to sync schema changes directly to your local database
- Fast iteration, no migration files generated

**Production:**
- Schema changes must be generated as migration files: `npm run db:generate`
- Commit the generated `drizzle/` folder to git
- Migrations run automatically on app startup via `hooks.server.ts`
- No need for `drizzle-kit` in production - migrations use only `drizzle-orm`

**Workflow for schema changes:**
1. Edit `src/lib/server/db/schema.ts`
2. Generate migration: `cd site && npm run db:generate`
3. Review generated SQL in `drizzle/` folder
4. Commit migration files to git
5. Deploy - migrations run automatically on container startup

**Note:** The Dockerfile copies the `drizzle/` folder and migrations run via `drizzle-orm/better-sqlite3/migrator` which doesn't require `drizzle-kit`. The `npm prune --production` step removes `drizzle-kit` but keeps `drizzle-orm` which is all that's needed to run migrations.

### CLI (Rust)

```bash
cd cli

# Build in debug mode
cargo build

# Build release binary (outputs to target/release/claude-afk)
cargo build --release

# Linting and formatting
cargo fmt
cargo clippy

# Run without building
cargo run -- <subcommand>

# Run tests
cargo test
cargo test -- --nocapture  # With output
```

## Key Implementation Details

### Database Schema

Single table `pairing_sessions` with:
- `id`: Primary key (nanoid, 21 chars) - used for status polling
- `pairingToken`: Temporary token (nanoid, 32 chars) - used in QR URL
- `deviceToken`: Permanent token (nanoid, 32 chars) - used for CLI authentication
- `pushSubscription`: JSON string of Web Push PushSubscription object
- `createdAt`, `completedAt`: Timestamps

### API Endpoints

- `POST /api/pairing/initiate` - Start pairing, returns `pairingId` and `pairingToken`
- `GET /api/pairing/[pairingId]/status` - Check if pairing is complete (polled by CLI)
- `POST /api/pairing/[pairingToken]/complete` - Complete pairing with push subscription from browser
- `POST /api/notify` - Send notification (requires `Authorization: Bearer {deviceToken}`)
- `GET /api/vapid-public-key` - Get VAPID public key for browser subscription

### CLI Configuration

Config stored using `confy` crate in platform-appropriate location:
- Linux: `~/.config/claude-afk/claude-afk.toml`
- macOS: `~/Library/Application Support/claude-afk/claude-afk.toml`
- Windows: `%APPDATA%\claude-afk\claude-afk.toml`

Config fields: `device_token`, `backend_url`, `active`

Backend URL: Uses `https://ccafk.treeleaf.dev` by default. Can be overridden with `CLAUDE_AFK_API_URL` env var for local development.

### CLI Commands

- `pair` - Generate QR code and poll for pairing completion
- `notify` - Read JSON from stdin and send notification (exits silently if not configured/active)
- `status` - Show configuration status
- `afk` - Enable notifications
- `back` - Disable notifications (keeps pairing)
- `clear` - Remove device pairing
- `install-hooks` - Install Claude Code hooks for push notifications
- `clear-logs` - Clear all debug logs (debug builds only)

### Service Worker

Located at `site/static/sw.js`. Handles:
- `push` event: Shows notification with title, body, icon, badge
- `notificationclick` event: Focuses existing window or opens new one
- `install`/`activate` events: Immediately activates new service worker

## Code Style

- **Indentation**: 4-space tabs for all code
- **TypeScript**: Use for all JavaScript code
- **Svelte**: Use Svelte 5 syntax (runes: `$state`, `$derived`, `$effect`, etc.)
- **Styling**: TailwindCSS v4 for all styling
- **Rust**: Follow standard Rust conventions, use `cargo fmt`

## Error Handling

- CLI `notify` command exits silently on errors (prints to stderr but doesn't crash Claude Code hook)
- Backend validates all inputs and returns appropriate HTTP status codes
- Use user-friendly error messages

## Testing Strategy

- **CLI**: Unit tests in `cli/src/cmd.rs` (config, parsing, URL construction, state logic)
- **Backend**: Tests in `backend/src/*.spec.ts` using Vitest
- **E2E**: Manual testing flow documented in README.md

## MCP Server: Svelte Documentation

You have access to the Svelte MCP server with comprehensive Svelte 5 and SvelteKit documentation.

### Available MCP Tools

1. **list-sections**: Discover all available documentation sections with titles, use_cases, and paths. Use this FIRST when asked about Svelte/SvelteKit topics.

2. **get-documentation**: Retrieve full documentation for specific sections. After calling list-sections, analyze the use_cases field and fetch ALL relevant sections at once.

3. **svelte-autofixer**: Analyze Svelte code and return issues/suggestions. MUST be used whenever writing Svelte code before sending to user. Keep calling until no issues remain.

4. **playground-link**: Generate a Svelte Playground link. Ask user first if they want a playground link. NEVER use if code was written to project files.
